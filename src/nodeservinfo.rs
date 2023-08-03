use core::time::Duration;
use ticker::Ticker;
use tokio::sync::{RwLock, Mutex};
use std::{error::Error, sync::Arc};

use snowflake::SnowflakeIdGenerator;
use systemstat::{Platform, System};

use synerex_nodeapi::nodeapi;
use synerex_proto;

use crate::{nodestate::NodeState, GIT_VER, WAIT_TIME, DEFAULT_NI, IDType, SxServerOpt, SXSynerexClient, SXServiceClient};


// NodeservInfo is a connection info for each Node Server
#[derive(Debug)]
pub struct NodeServInfo {
    // we keep this for each nodeserver.
    pub node: SnowflakeIdGenerator, // package variable for keeping unique ID.
    pub nid: nodeapi::NodeId,
    pub nupd: RwLock<nodeapi::NodeUpdate>,
    pub my_node_name: String,
    pub my_server_info: String,
    pub my_node_type: nodeapi::NodeType,
    pub nodeclt: Option<Arc<Mutex<nodeapi::node_client::NodeClient<tonic::transport::Channel>>>>,
    pub msg_count: u64,
    pub node_state: NodeState,
}



impl NodeServInfo {
    pub fn new() -> NodeServInfo {
        debug!("Initializing NodeServInfo");
        NodeServInfo {
            node_state: NodeState::new(),
            node: SnowflakeIdGenerator::new(0, 0),
            nid: nodeapi::NodeId {
                node_id: -1,
                secret: 0,
                server_info: String::new(),
                keepalive_duration: 0,
            },
            nupd: RwLock::new(nodeapi::NodeUpdate {
                node_id: 0,
                secret: 0,
                update_count: 0,
                node_status: 0,
                node_arg: String::new(),
                status: None,
            }),
            my_node_name: String::new(),
            my_server_info: String::new(),
            my_node_type: nodeapi::NodeType::Provider,
            // conn: None,
            nodeclt: None,
            msg_count: 0,
        }
    }

    // GetNodeName returns node name from node_id
    pub async fn get_node_name(&mut self, n: i32) -> String {
        match self
            .nodeclt
            .as_mut()
            .unwrap()
            .lock().await
            .query_node(nodeapi::NodeId {
                node_id: n,
                secret: 0,
                server_info: String::new(),
                keepalive_duration: 60,
            })
            .await
        {
            Ok(ni) => ni.get_ref().node_name.clone(),
            Err(_) => String::from("Unknown"),
        }
    }

    // SetNodeStatus updates KeepAlive info to NodeServer
    pub async fn set_node_status(&self, status: i32, arg: String) {
        let mut nupd = self.nupd.write().await;
        nupd.node_status = status;
        nupd.node_arg = arg;
    }

    pub async fn reconnect_node_serv(&mut self) -> Result<(), Box<dyn Error>> {
        // re_send connection info to server.
        let nif = nodeapi::NodeInfo {
            node_name: self.my_node_name.clone(),
            node_type: self.my_node_type.into(),
            server_info: self.my_server_info.clone(), // TODO: this is not correctly initialized
            node_pbase_version: synerex_proto::CHANNEL_TYPE_VERSION.to_string(), // this is defined at compile time
            with_node_id: self.nid.node_id,
            bin_version: GIT_VER.to_string(),  // git bin tag version
            cluster_id: 0,
            area_id: String::new(),
            channel_types: Vec::new(),
            gw_info: String::new(),
            count: 0,
            last_alive_time: Some(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            keepalive_arg: String::new(),
        };

        match self.nodeclt.as_mut().unwrap().lock().await.register_node(nif).await {
            Ok(nid) => {
                self.nid = nid.get_ref().clone();
                self.node = snowflake::SnowflakeIdGenerator::new(0, self.nid.node_id);
                info!("Successfully ReInitialize node {}", self.nid.node_id);
                self.nupd = RwLock::new(nodeapi::NodeUpdate {
                    node_id: self.nid.node_id,
                    secret: self.nid.secret,
                    update_count: 0,
                    node_status: 0,
                    node_arg: String::new(),
                    status: None,
                });
                Ok(())
            }
            Err(e) => {
                error!("{:?}", e);
                Err(Box::from(e))
            }
        }
    }

    pub async fn start_keep_alive_with_cmd(
        &mut self,
        cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>,
    ) {
        let keepalive_duration = self.nid.keepalive_duration as u64;
        loop {
            self.msg_count = 0; // how count message?
            {
                debug!(
                    "KeepAlive {} {}",
                    // self.nupd.read().as_ref().unwrap().node_status,
                    self.nupd.read().await.node_status,
                    keepalive_duration
                );
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(keepalive_duration)).await;
            if self.nid.secret == 0 {
                // this means the node is disconnected
                break;
            }

            if self.my_node_type == nodeapi::NodeType::Server {
                // obtain cpu status
                let sys = System::new();
                let cpu_percent = match sys.load_average() {
                    Ok(loadavg) => loadavg.one,
                    Err(x) => {
                        error!("\nLoad average: error: {}", x);
                        0.0
                    }
                };
                let mem_percent = match sys.memory() {
                    Ok(mem) => {
                        ((mem.total.as_u64() - mem.free.as_u64()) as f64
                            / (mem.total.as_u64() as f64))
                            * 100.0
                    }
                    Err(x) => {
                        error!("\nMemory: error: {}", x);
                        0.0
                    }
                };
                let status = nodeapi::ServerStatus {
                    cpu: cpu_percent as f64,
                    memory: mem_percent,
                    msg_count: self.msg_count,
                };
                self.nupd.write().await.status = Some(status);
            }

            self.nupd.write().await.update_count += 1;

            let nupd_clone = self.nupd.read().await.clone();
            let nodeclt_arc = Arc::clone(&self.nodeclt.as_ref().unwrap());

            match nodeclt_arc.lock().await.keep_alive(nupd_clone).await {
                Ok(resp) => {
                    // there might be some errors in response
                    match resp.get_ref().command() {
                        nodeapi::KeepAliveCommand::None => {}
                        nodeapi::KeepAliveCommand::Reconnect => {
                            // order is reconnect to node.
                            match self.reconnect_node_serv().await {
                                Ok(_) => {},
                                Err(_) => { error!("Above error was happend when nodeapi::KeepAliveCommand::Reconnect") },
                            };
                        }
                        nodeapi::KeepAliveCommand::ServerChange => {
                            info!("receive SERVER_CHANGE\n");

                            if self.node_state.is_safe_state() {
                                self.un_register_node().await;

                                // if !self.conn.is_none() {
                                //     // self.conn.unwrap().close();  // TODO: inspect this.
                                // }

                                if cmd_func.is_some() {
                                    cmd_func.unwrap()(
                                        resp.get_ref().command(),
                                        resp.get_ref().err.clone(),
                                    );
                                    self.node_state.init();
                                }
                            } else {
                                // wait
                                if !self.node_state.locked {
                                    self.node_state.locked = true;
                                    // TODO: fix here (currently assume DEFAULT_NI)
                                    tokio::spawn(async {
                                        let ticker = Ticker::new(0..1, Duration::from_secs(WAIT_TIME));
                                        for _ in ticker {
                                            DEFAULT_NI.write().await.node_state.init();
                                        }
                                    });
                                    // tokio::spawn(lazy_init_node(self));
                                }
                            }
                        }
                        nodeapi::KeepAliveCommand::ProviderDisconnect => {
                            info!("receive PROV_DISCONN {:?}\n", resp);
                            if self.my_node_type != nodeapi::NodeType::Server {
                                info!(
                                    "NodeType shoud be SERVER! {:?} {} {:?}",
                                    self.my_node_type, self.my_node_name, resp
                                );
                            } else if !cmd_func.is_none() {
                                // work provider disconnect
                                cmd_func.unwrap()(resp.get_ref().command(), resp.get_ref().err.clone());
                            }
                        }
                    }

                    true
                }
                Err(e) => {
                    error!("Error in response, may nodeserv failure {:?}", e);
                    false
                }
            };
        }
    }

    pub fn msg_count_up(&mut self) {
        self.msg_count += 1;
    }

    pub async fn un_register_node(&mut self) {
        info!("UnRegister Node {:?}", self.nid);
        let nid = self.nid.clone(); // TODO: fix nid definition,
        match self.nodeclt.as_mut().unwrap().lock().await.un_register_node(nid).await {
            Ok(resp) => {
                if !resp.get_ref().ok {
                    error!("Can't unregister (resp.ok == false)");
                }
            }
            Err(err) => {
                error!("Can't unregister {}", err);
            }
        };
        self.nid.secret = 0;
    }

    // RegisterNodeWithCmd is a function to register Node with node server address and KeepAlive Command Callback
    pub async fn register_node_with_cmd(&mut self, nodesrv: String, nm: String, channels: Vec<u32>, serv: Option<&SxServerOpt>, cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>) -> Result<String, &str> { // register ID to server
        self.nodeclt = match nodeapi::node_client::NodeClient::connect(nodesrv).await {
            Ok(clt) => Some(Arc::from(Mutex::from(clt))),
            Err(err) => { error!("{:?}", err); None },
        };
        if self.nodeclt.is_none() {
            return Err("register_node_with_cmd: node connection error");
        }

        let node_id: i32 = self.nid.node_id;
        self.my_node_type = nodeapi::NodeType::Provider;
        self.my_node_name = nm.clone();
        let mut nif = nodeapi::NodeInfo{
            node_name: nm,
            node_type: self.my_node_type.into(),
            server_info: String::from(""),
            node_pbase_version: String::from(synerex_proto::CHANNEL_TYPE_VERSION),
            with_node_id: node_id,
            cluster_id: 0,
            area_id: String::from("Default"),
            channel_types: channels,
            gw_info: String::from(""),
            bin_version: String::from(GIT_VER),
            count: 0,
            last_alive_time: None,
            keepalive_arg: String::from(""),
        };

        if serv.is_some() {
            self.my_node_type = serv.unwrap().node_type;
            self.my_server_info = serv.unwrap().server_info.clone();
            nif.node_type = self.my_node_type.into();
            nif.server_info = self.my_server_info.clone();
            nif.cluster_id = serv.unwrap().cluster_id;
            nif.area_id = serv.unwrap().area_id.clone();
            nif.gw_info = serv.unwrap().gw_info.clone();
        }

        self.nid = match self.nodeclt.as_mut().unwrap().lock().await.register_node(nif).await {
            Ok(resp) => resp.get_ref().clone(),
            Err(status) => {
                error!("{:?}", status);
                nodeapi::NodeId {
                    node_id: -1,
                    secret: 0,
                    server_info: String::from(""),
                    keepalive_duration: -1,
                }
            },
        };
        if self.nid.keepalive_duration == -1 {  // register_node error
            return Err("register_node error");
        }

        self.node = snowflake::SnowflakeIdGenerator::new(0, self.nid.node_id);

        *self.nupd.write().await = nodeapi::NodeUpdate {
            node_id: self.nid.node_id,
            secret: self.nid.secret,
            update_count: 0,
            node_status: 0,
            node_arg: String::from(""),
            status: None,
        };
        // if let Ok(mut nupd) = self.nupd.write() {
        //     *nupd = nodeapi::NodeUpdate {
        //         node_id: self.nid.node_id,
        //         secret: self.nid.secret,
        //         update_count: 0,
        //         node_status: 0,
        //         node_arg: String::from(""),
        //         status: None,
        //     };
        // }

        // start keepalive routine
        // tokio::spawn(self.startKeepAliveWithCmd(cmd_func));
        // go ni.startKeepAliveWithCmd(cmd_func)
        // //	fmt.Println("KeepAlive started!")

        Ok(self.nid.server_info.clone())
    }

    // NewSXServiceClient Creates wrapper structre SXServiceClient from SynerexClient
    // Warning: In Rust version, this function is not used.
    pub fn new_sx_service_client(&mut self, clt: SXSynerexClient, mtype: u32, arg_json: String) -> SXServiceClient {
        SXServiceClient {
            client_id: IDType::from(self.node.generate() as u64),
            channel_type: mtype,
            sxclient: Some(clt),
            arg_json,
            mbus_ids: RwLock::from(Vec::new()),
            ni: None,
        }
    }

    // GenerateIntID for generate uniquie ID
    pub fn generate_int_id(&mut self) -> u64 {
        self.node.generate() as u64
    }
}


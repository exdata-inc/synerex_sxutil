#[macro_use]
extern crate log;
extern crate env_logger as logger;

// use std::sync::RwLock;
use tokio::sync::RwLock;
use std::{thread, time};
use once_cell::sync::Lazy;

use build_time::build_time_local;
use git_version::git_version;
use snowflake::SnowflakeIdGenerator;
use systemstat::{Platform, System};

use synerex_api::api;
use synerex_nodeapi::nodeapi;
use synerex_proto;

// sxutil.go is a helper utility package for Synerex

// Helper structures for Synerex

// IDType for all ID in Synerex
type IDType = u64;

static WAIT_TIME: isize = 30;

// this is for Message Timeout for synerex server
static MSG_TIME_OUT: isize = 20; // from v0.6.1 10sec -> 20sec

static RECONNECT_WAIT: isize = 5; // from v0.6.1

// for git versions
const GIT_VER: &str = git_version!();
const BUILD_TIME: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

// var (
// 	Sha1Ver   &str // sha1 version used to build the program
// )

pub struct NodeState {
    pub proposed_supply: Vec<api::Supply>,
    pub proposed_demand: Vec<api::Demand>,
    pub locked: bool,
}

// NodeservInfo is a connection info for each Node Server
pub struct NodeServInfo{ //<'a> {
    // we keep this for each nodeserver.
    pub node: SnowflakeIdGenerator, // package variable for keeping unique ID.
    pub nid: nodeapi::NodeId,
    pub nupd: RwLock<nodeapi::NodeUpdate>,
    // pub numu:      sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub my_node_name: String,
    pub my_server_info: String,
    pub my_node_type: nodeapi::NodeType,
    // pub conn: Option<&'a tonic::client::Grpc<tonic::transport::Channel>>, // TODO: inspect grpc in rust
    pub clt: Option<nodeapi::node_client::NodeClient<tonic::transport::Channel>>,
    pub msg_count: u64,
    pub node_state: NodeState,
}

// DemandOpts is sender options for Demand
pub struct DemandOpts {
    pub id: u64,
    pub target: u64,
    pub name: String,
    pub json: String,
    pub cdata: api::Content,
}

// SupplyOpts is sender options for Supply
pub struct SupplyOpts {
    pub id: u64,
    pub target: u64,
    pub name: String,
    pub json: String,
    pub cdata: api::Content,
}

// SXSynerexClient is for each server from v0.5.0
pub struct SXSynerexClient {
    pub server_address: String,
    pub client: api::synerex_client::SynerexClient<tonic::transport::Channel>,
}

// SXServiceClient Wrappter Structure for synerex client
pub struct SXServiceClient<'a> {
    pub client_id: IDType,
    pub channel_type: u32,
    pub sxclient: &'a SXSynerexClient,
    pub arg_json: String,
    pub mbus_ids: RwLock<Vec<IDType>>,
    // pub mbusMutex:   sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub ni: &'a NodeServInfo,
}

pub trait DemandHandler {
    fn on_notify_demand<'a>(clt: &'a SXServiceClient<'a>, dm: &'a api::Demand) -> &'a SupplyOpts; // if propose return proposedID
    fn on_select_supply<'a>(clt: &'a SXServiceClient<'a>, dm: &'a api::Demand) -> bool; // if confirm return true
    fn on_confirm_response<'a>(
        clt: &'a SXServiceClient<'a>,
        idtype: IDType,
        err: dyn std::error::Error,
    ); // result of confirm
}

pub trait SupplyHandler {}

pub struct SxServerOpt {
    pub node_type: nodeapi::NodeType,
    pub server_info: String,
    pub cluster_id: i32,
    pub area_id: String,
    pub gw_info: String,
}

// replaced by Nodestate::new()
// fn NewNodeState() -> NodeState {
//     let mut obj = NodeState::new();
// 	   // obj.init();  // not necessary
//     debug!("Initializing NodeState");
//     obj
// }

impl NodeState {
    pub fn new() -> NodeState {
        debug!("Initializing NodeState");
        NodeState {
            proposed_supply: Vec::new(),
            proposed_demand: Vec::new(),
            locked: false,
        }
    }

    pub fn init(&mut self) {
        self.proposed_supply = Vec::new();
        self.proposed_demand = Vec::new();
        self.locked = false;
    }

    pub fn is_safe_state(&self) -> bool {
        self.proposed_supply.len() == 0 && self.proposed_demand.len() == 0
    }

    pub fn propose_supply(&mut self, supply: api::Supply) {
        info!("NodeState#proposeSupply[{}] is called", supply.id);
        self.proposed_supply.push(supply);
        info!("proposeSupply len {}", self.proposed_supply.len());
    }

    pub fn proposed_supply_index(&self, id: u64) -> i64 {
        for i in 0..self.proposed_supply.len() {
            if self.proposed_supply[i].id == id {
                return i as i64;
            }
        }
        return -1;
    }

    pub fn remove_proposed_supply_index(&mut self, pos: i64) {
        if pos < 0 {
            panic!("remove idx must be positive.");
        } else {
            self.proposed_supply.remove(pos as usize);
        }
    }

    pub fn select_supply(&mut self, id: u64) -> bool {
        debug!("NodeState#selectSupply[{}] is called\n", id);
        let pos = self.proposed_supply_index(id);
        if pos >= 0 {
            self.remove_proposed_supply_index(pos);
            return true;
        } else {
            warn!("not found supply[{}]\n", id);
            return false;
        }
    }

    pub fn propose_demand(&mut self, demand: api::Demand) {
        info!("NodeState#proposeDemand[{}] is called\n", demand.id);
        self.proposed_demand.push(demand);
    }

    pub fn select_demand(&mut self, id: u64) -> bool {
        info!("NodeState#selectDemand[{}] is called\n", id);

        let mut pos: i64 = -1;
        for i in 0..self.proposed_demand.len() {
            if self.proposed_demand[i].id == id {
                pos = i as i64
            }
        }

        if pos >= 0 {
            self.proposed_demand.remove(pos as usize);
            return true;
        } else {
            warn!("not found supply[{}]\n", id);
            return false;
        }
    }
}

static DEFAULT_NI: Lazy<RwLock<Option<NodeServInfo>>> = Lazy::new(|| {
    RwLock::new(None)
});
// static DEFAULT_NI: RwLock<Option<NodeServInfo>> = RwLock::new(None);

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
            clt: None,
            msg_count: 0,
        }
    }

    // GetNodeName returns node name from node_id
    pub async fn get_node_name(&mut self, n: i32) -> String {
        match self
            .clt
            .as_mut()
            .unwrap()
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
        // if let Ok(mut nupd) = self.nupd.write().await; {
        //     nupd.node_status = status;
        //     nupd.node_arg = arg;
        // }
    }

    pub async fn reconnect_node_serv(&mut self) -> bool {
        // re_send connection info to server.
        let nif = nodeapi::NodeInfo {
            node_name: self.my_node_name.clone(),
            node_type: self.my_node_type.into(),
            server_info: self.my_server_info.clone(), // TODO: this is not correctly initialized
            node_pbase_version: synerex_proto::CHANNEL_TYPE_VERSION.to_string(), // this is defined at compile time
            with_node_id: self.nid.node_id,
            bin_version: GIT_VER.to_string(),
            cluster_id: 0,
            area_id: String::new(),
            channel_types: Vec::new(),
            gw_info: String::new(),
            count: 0,
            last_alive_time: Option::from(prost_types::Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            keepalive_arg: String::new(), // git bin tag version
        };

        match self.clt.as_mut().unwrap().register_node(nif).await {
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
                true
            }
            Err(e) => {
                error!("{:?}", e);
                false
            }
        }
    }

    pub async fn start_keep_alive_with_cmd(
        &mut self,
        cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>,
    ) {
        loop {
            self.msg_count = 0; // how count message?
            {
                debug!(
                    "KeepAlive {} {}",
                    // self.nupd.read().as_ref().unwrap().node_status,
                    self.nupd.read().await.node_status,
                    self.nid.keepalive_duration
                );
            }
            thread::sleep(time::Duration::from_secs(
                self.nid.keepalive_duration as u64,
            ));
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
                self.nupd.write().await.status = Option::from(status);
                // if let Ok(mut nupd) = self.nupd.write() {
                //     nupd.status = Option::from(status);
                // }
            }

            self.nupd.write().await.update_count += 1;
            // if let Ok(mut nupd) = self.nupd.write() {
            //     nupd.update_count += 1;
            // }

            let mut fut = Option::from(None);
            {
                let nupd_clone = self.nupd.read().await.clone();
                fut = Option::from(self.clt.as_mut().unwrap().keep_alive(nupd_clone));
            }
            // if let Ok(nupd) = self.nupd.read() {
            //     let nupd_clone = self.nupd.read().unwrap().clone();
            //     fut = Option::from(self.clt.as_mut().unwrap().keep_alive(nupd_clone));
            // }

            if !fut.is_none() {
                let res = match fut.unwrap().await {
                    Ok(resp) => {
                        // there might be some errors in response
                        match resp.get_ref().command() {
                            nodeapi::KeepAliveCommand::None => {}
                            nodeapi::KeepAliveCommand::Reconnect => {
                                // order is reconnect to node.
                                self.reconnect_node_serv().await;
                            }
                            nodeapi::KeepAliveCommand::ServerChange => {
                                info!("receive SERVER_CHANGE\n");

                                if self.node_state.is_safe_state() {
                                    self.un_register_node().await;

                                    // if !self.conn.is_none() {
                                    //     // self.conn.unwrap().close();  // TODO: inspect this.
                                    // }

                                    if !cmd_func.is_none() {
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
                                        // go func() {
                                        //     t := time.NewTicker(WAIT_TIME * time.Second) // 30 seconds
                                        //     <-t.C
                                        //     self.nodeState.init()
                                        //     t.Stop() // タイマを止める。
                                        // }()
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
    }

    pub fn msg_count_up(&mut self) {
        self.msg_count += 1;
    }

    pub async fn un_register_node(&mut self) {
        info!("UnRegister Node {:?}", self.nid);
        let nid = self.nid.clone(); // TODO: fix nid definition,
        match self.clt.as_mut().unwrap().un_register_node(nid).await {
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
        self.clt = match nodeapi::node_client::NodeClient::connect(nodesrv).await {
            Ok(clt) => Option::from(clt),
            Err(err) => { error!("{:?}", err); None },
        };
        if self.clt.is_none() {
            return Err("node connection error");
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

        self.nid = match self.clt.as_mut().unwrap().register_node(nif).await {
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
}

// func init()
pub async fn initialize_default_ni() {
    *DEFAULT_NI.write().await = std::option::Option::<NodeServInfo>::from(NodeServInfo::new());
    // if let Ok(mut default_ni) = DEFAULT_NI.write() {
    //     *default_ni = std::option::Option::<NodeServInfo>::from(NodeServInfo::new());
    // }
}

// InitNodeNum for initialize NodeNum again
pub async fn init_node_num(n: i32) {
    DEFAULT_NI.write().await.as_mut().unwrap().node = snowflake::SnowflakeIdGenerator::new(0, n);
    info!("Successfully Initialize node {}", n);
    // if let Ok(mut ds) = DEFAULT_NI.write() {
    //     ds.as_mut().unwrap().node = snowflake::SnowflakeIdGenerator::new(0, n);
    //     info!("Successfully Initialize node {}", n);
    // }
}

// SetNodeStatus updates KeepAlive info to NodeServer
pub async fn set_node_status(status: i32, arg: String) {
    DEFAULT_NI.write().await.as_mut().unwrap().set_node_status(status, arg);
    // if let Ok(mut default_ni) = DEFAULT_NI.write() {
	// 	default_ni.as_mut().unwrap().set_node_status(status, arg);
    // }
}

pub async fn msg_count_up() { // is this needed?
    DEFAULT_NI.write().await.as_mut().unwrap().msg_count_up();
    // if let Ok(mut default_ni) = DEFAULT_NI.write() {
    //     default_ni.as_mut().unwrap().msg_count_up();
    // }
}

// RegisterNode is a function to register Node with node server address
pub async fn register_node(nodesrv: String, nm: String, channels: Vec<u32>, serv: Option<&SxServerOpt>) -> Result<String, String> { // register ID to server
	return register_node_with_cmd(nodesrv, nm, channels, serv, None).await
}

// RegisterNodeWithCmd is a function to register Node with node server address and KeepAlive Command Callback
pub async fn register_node_with_cmd(nodesrv: String, nm: String, channels: Vec<u32>, serv: Option<&SxServerOpt>, cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>) -> Result<String, String> { // register ID to server
    return match DEFAULT_NI.write().await.as_mut().unwrap().register_node_with_cmd(nodesrv, nm, channels, serv, cmd_func).await {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{}", err)),
    };
    // if let Ok(mut default_ni) = DEFAULT_NI.write() {
    //     return match default_ni.as_mut().unwrap().register_node_with_cmd(nodesrv, nm, channels, serv, cmd_func).await {
    //         Ok(result) => Ok(result),
    //         Err(err) => Err(format!("{}", err)),
    //     };
    // } else {
    //     Err(String::from("failed to lock"))
    // }
}

pub async fn start_keep_alive_with_cmd(cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>) -> Result<String, String> {
    DEFAULT_NI.write().await.as_mut().unwrap().start_keep_alive_with_cmd(cmd_func).await;
    Ok(String::from("keep alive finished"))
    // if let Ok(mut default_ni) = DEFAULT_NI.write() {
    //     default_ni.as_mut().unwrap().startKeepAliveWithCmd(cmd_func).await;
    //     Ok(String::from("keep alive finished"))
    // } else {
    //     Err(String::from("failed to lock"))
    // }    
}

pub async fn un_register_node() {
    DEFAULT_NI.write().await.as_mut().unwrap().un_register_node().await;
}


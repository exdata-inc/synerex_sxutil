#[macro_use]
extern crate log;
extern crate env_logger as logger;

use core::time::Duration;
use chrono::{Local, Datelike, Timelike};
use prost_types::Timestamp;
use ticker::Ticker;
// use std::sync::RwLock;
use tokio::sync::{RwLock, Mutex};
use std::{thread, time, sync::Arc}; //, future::Future, pin::Pin};
use once_cell::sync::Lazy;

use build_time::build_time_local;
use git_version::git_version;
use snowflake::SnowflakeIdGenerator;
use systemstat::{Platform, System};

use synerex_api::api;
use synerex_nodeapi::nodeapi;
use synerex_proto;

// sxutil is a helper utility package for Synerex

// Helper structures for Synerex

// IDType for all ID in Synerex
type IDType = u64;

static WAIT_TIME: u64 = 30;

// this is for Message Timeout for synerex server
static MSG_TIME_OUT: isize = 20; // from v0.6.1 10sec -> 20sec

static RECONNECT_WAIT: u64 = 5; // from v0.6.1

const GIT_VER: &str = git_version!();
const BUILD_TIME: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");


#[derive(Debug)]
pub struct NodeState {
    pub proposed_supply: Vec<api::Supply>,
    pub proposed_demand: Vec<api::Demand>,
    pub locked: bool,
}

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
    pub clt: Option<nodeapi::node_client::NodeClient<tonic::transport::Channel>>,
    pub msg_count: u64,
    pub node_state: NodeState,
}

// DemandOpts is sender options for Demand
#[derive(Debug)]
pub struct DemandOpts {
    pub id: u64,
    pub target: u64,
    pub name: String,
    pub json: String,
    pub cdata: api::Content,
}

// SupplyOpts is sender options for Supply
#[derive(Debug)]
pub struct SupplyOpts {
    pub id: u64,
    pub target: u64,
    pub name: String,
    pub json: String,
    pub cdata: api::Content,
}

// SXSynerexClient is for each server from v0.5.0
#[derive(Debug)]
pub struct SXSynerexClient {
    pub server_address: String,
    pub client: api::synerex_client::SynerexClient<tonic::transport::Channel>,
}

// SXServiceClient Wrappter Structure for synerex client
#[derive(Debug)]
pub struct SXServiceClient {
    pub client_id: IDType,
    pub channel_type: u32,
    pub sxclient: SXSynerexClient,
    pub arg_json: String,
    pub mbus_ids: RwLock<Vec<IDType>>,
    // pub mbusMutex:   sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub ni: Option<NodeServInfo>,
}

pub trait DemandHandler {
    fn on_notify_demand(&self, clt: &SXServiceClient, dm: &api::Demand) -> Option<SupplyOpts> where Self: Sized; // if propose return proposedID
    fn on_select_supply(&self, clt: &SXServiceClient, dm: &api::Demand) -> bool where Self: Sized; // if confirm return true
    fn on_confirm_response(
        &self, 
        clt: &SXServiceClient,
        idtype: IDType,
        err: dyn std::error::Error,
    ) where Self: Sized; // result of confirm
}

pub trait SupplyHandler {}

pub struct SxServerOpt {
    pub node_type: nodeapi::NodeType,
    pub server_info: String,
    pub cluster_id: i32,
    pub area_id: String,
    pub gw_info: String,
}

// below was replaced by Nodestate::new()
// fn NewNodeState() -> NodeState

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

// func init()
static DEFAULT_NI: Lazy<RwLock<NodeServInfo>> = Lazy::new(|| {
    debug!("sxutil: {} built {}", GIT_VER, BUILD_TIME);
    RwLock::new(NodeServInfo::new())
});

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
    }

    pub async fn reconnect_node_serv(&mut self) -> bool {
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
                self.nupd.write().await.status = Some(status);
            }

            self.nupd.write().await.update_count += 1;

            let fut;
            {
                let nupd_clone = self.nupd.read().await.clone();
                fut = Some(self.clt.as_mut().unwrap().keep_alive(nupd_clone));
            }

            if fut.is_some() {
                match fut.unwrap().await {
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
            Ok(clt) => Some(clt),
            Err(err) => { error!("{:?}", err); None },
        };
        if self.clt.is_none() {
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

    // NewSXServiceClient Creates wrapper structre SXServiceClient from SynerexClient
    // Warning: In Rust version, this function is not used.
    pub fn new_sx_service_client(&mut self, clt: SXSynerexClient, mtype: u32, arg_json: String) -> SXServiceClient {
        SXServiceClient {
            client_id: IDType::from(self.node.generate() as u64),
            channel_type: mtype,
            sxclient: clt,
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

pub async fn lazy_init_node(ni: &mut NodeServInfo) {
    let ticker = Ticker::new(0..1, Duration::from_secs(WAIT_TIME));
    for _ in ticker {
        // TODO: fix here (currently assume DEFAULT_NI)
        ni.node_state.init();
    }
}

// InitNodeNum for initialize NodeNum again
pub async fn init_node_num(n: i32) {
    DEFAULT_NI.write().await.node = snowflake::SnowflakeIdGenerator::new(0, n);
    info!("Successfully Initialize node {}", n);
}

// SetNodeStatus updates KeepAlive info to NodeServer
pub async fn set_node_status(status: i32, arg: String) {
    DEFAULT_NI.read().await.set_node_status(status, arg).await;
}

pub async fn msg_count_up() { // is this needed?
    DEFAULT_NI.write().await.msg_count_up();
}

// RegisterNode is a function to register Node with node server address
pub async fn register_node(nodesrv: String, nm: String, channels: Vec<u32>, serv: Option<&SxServerOpt>) -> Result<String, String> { // register ID to server
	return register_node_with_cmd(nodesrv, nm, channels, serv, None).await
}

// RegisterNodeWithCmd is a function to register Node with node server address and KeepAlive Command Callback
pub async fn register_node_with_cmd(nodesrv: String, nm: String, channels: Vec<u32>, serv: Option<&SxServerOpt>, cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>) -> Result<String, String> { // register ID to server
    return match DEFAULT_NI.write().await.register_node_with_cmd(nodesrv, nm, channels, serv, cmd_func).await {
        Ok(result) => Ok(result),
        Err(err) => Err(format!("{}", err)),
    };
}

pub async fn start_keep_alive_with_cmd(cmd_func: Option<fn(nodeapi::KeepAliveCommand, String)>) -> Result<String, String> {
    loop {
        DEFAULT_NI.write().await.msg_count = 0; // how count message?
        {
            debug!(
                "KeepAlive {} {}",
                // self.nupd.read().as_ref().unwrap().node_status,
                DEFAULT_NI.read().await.nupd.read().await.node_status,
                DEFAULT_NI.read().await.nid.keepalive_duration
            );
        }
        thread::sleep(time::Duration::from_secs(
            DEFAULT_NI.read().await.nid.keepalive_duration as u64,
        ));
        if DEFAULT_NI.read().await.nid.secret == 0 {
            // this means the node is disconnected
            break;
        }

        if DEFAULT_NI.read().await.my_node_type == nodeapi::NodeType::Server {
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
                msg_count: DEFAULT_NI.read().await.msg_count,
            };
            DEFAULT_NI.write().await.nupd.write().await.status = Some(status);
        }

        {
            DEFAULT_NI.write().await.nupd.write().await.update_count += 1;
        }

        let nupd_clone = DEFAULT_NI.read().await.nupd.read().await.clone();
        let fut = DEFAULT_NI.write().await.clt.as_mut().unwrap().keep_alive(nupd_clone).await;

        match fut {
            Ok(resp) => {
                // there might be some errors in response
                match resp.get_ref().command() {
                    nodeapi::KeepAliveCommand::None => {}
                    nodeapi::KeepAliveCommand::Reconnect => {
                        // order is reconnect to node.
                        DEFAULT_NI.write().await.reconnect_node_serv().await;
                    }
                    nodeapi::KeepAliveCommand::ServerChange => {
                        info!("receive SERVER_CHANGE\n");

                        if DEFAULT_NI.read().await.node_state.is_safe_state() {
                            DEFAULT_NI.write().await.un_register_node().await;

                            // if !self.conn.is_none() {
                            //     // self.conn.unwrap().close();  // TODO: inspect this.
                            // }

                            if cmd_func.is_some() {
                                cmd_func.unwrap()(
                                    resp.get_ref().command(),
                                    resp.get_ref().err.clone(),
                                );
                                DEFAULT_NI.write().await.node_state.init();
                            }
                        } else {
                            // wait
                            if !DEFAULT_NI.read().await.node_state.locked {
                                DEFAULT_NI.write().await.node_state.locked = true;
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
                        if DEFAULT_NI.read().await.my_node_type != nodeapi::NodeType::Server {
                            info!(
                                "NodeType shoud be SERVER! {:?} {} {:?}",
                                DEFAULT_NI.read().await.my_node_type, DEFAULT_NI.read().await.my_node_name, resp
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
    // DEFAULT_NI.write().await.start_keep_alive_with_cmd(cmd_func).await;
    Ok(String::from("keep alive finished"))
}

pub async fn un_register_node() {
    DEFAULT_NI.write().await.un_register_node().await;
}

// GrpcConnectServer is a utility function for conneting gRPC server
pub async fn grpc_connect_server(server_address: String) -> Option<SXSynerexClient> { // TODO: we may add connection option
	if server_address == "" {
		error!("sxutil: [FATAL] no server address cor GrpcConnectServer");
		return None
	}
	// opts = append(opts, grpc.WithInsecure()) // currently we do not use sercure connection //TODO: we need to udpate SSL
	// opts = append(opts, grpc.WithBlock()) // this is required to ensure client connection
	let client = match api::synerex_client::SynerexClient::connect(server_address.clone()).await {
        Ok(clt) => clt,
        Err(err) => {
            error!("sxutil:GRPC-conn  Failed to connect server {} err: {}", server_address, err);
            return None
        },
    };
    debug!("sxutil: clt: {:?}", client);

	// from v0.5.0 , we support Connection in sxutil.
	Some(SXSynerexClient{
		server_address: server_address,
		client,
	})
}

// NewSXServiceClient Creates wrapper structre SXServiceClient from SynerexClient
pub async fn new_sx_service_client(clt: SXSynerexClient, mtype: u32, arg_json: String) -> SXServiceClient {
    let client_id = DEFAULT_NI.write().await.generate_int_id();
    // sxServiceClient.ni = Some(&DEFAULT_NI);
    SXServiceClient {
        client_id,
        channel_type: mtype,
        sxclient: clt,
        arg_json,
        mbus_ids: RwLock::from(Vec::new()),
        ni: None,
    }
	// return defaultNI.NewSXServiceClient(clt, mtype, argJson)
}

// GenerateIntID for generate uniquie ID
pub async fn generate_int_id() -> u64 {
    DEFAULT_NI.write().await.generate_int_id()
}

impl SXServiceClient {
    pub fn get_channel(&self) -> api::Channel {
        api::Channel { client_id: self.client_id, channel_type: self.channel_type, arg_json: self.arg_json.clone() }
    }

    // IsSupplyTarget is a helper function to check target
    pub fn is_supply_target(&self, sp: &api::Supply, idlist: Vec<u64>) -> bool {
        let spid = sp.target_id;
        idlist.contains(&spid)
    }

    // IsDemandTarget is a helper function to check target
    pub fn is_demand_target(&self, dm: &api::Demand, idlist: Vec<u64>) -> bool {
        let dmid = dm.target_id;
        idlist.contains(&dmid)
    }

    // ProposeSupply send proposal Supply message to server
    pub async fn propose_supply(&mut self, spo: &SupplyOpts) -> u64 {
        let pid = generate_int_id().await;
        let dt = Local::now();
        let ts = Timestamp::date_time_nanos(dt.year() as i64, dt.month() as u8, dt.day() as u8, dt.hour() as u8, dt.minute() as u8, dt.second() as u8, dt.nanosecond() as u32).unwrap();
        let sp = api::Supply {
            id: pid,
            sender_id: self.client_id,
            target_id: spo.target,
            channel_type: self.channel_type,
            supply_name: spo.name.clone(),
            ts: Some(ts),
            arg_json: spo.json.clone(),
            mbus_id: u64::MAX,
            cdata: Some(spo.cdata.clone()),
        };

        //	match clt.channel_type {//
        //Todo: We need to make if for each channel type
        //	}

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        match self.sxclient.client.propose_supply(sp.clone()).await {
            Ok(resp) => {
                debug!("ProposeSupply Response: {:?} PID: {}", resp, pid);
            },
            Err(err) => {
                error!("{:?}.ProposeSupply err {}, [{:?}]", self, err, sp);
                return 0;
            },
        }

        self.ni.as_mut().unwrap().node_state.propose_supply(sp);

        pid
    }
    
    // ProposeDemand send proposal Demand message to server
    pub async fn propose_demand(&mut self, dmo: DemandOpts) -> u64 {
        let pid = generate_int_id().await;
        let dt = Local::now();
        let ts = Timestamp::date_time_nanos(dt.year() as i64, dt.month() as u8, dt.day() as u8, dt.hour() as u8, dt.minute() as u8, dt.second() as u8, dt.nanosecond() as u32).unwrap();
        let dm = api::Demand {
            id: pid,
            sender_id: self.client_id,
            target_id: dmo.target,
            channel_type: self.channel_type,
            demand_name: dmo.name.clone(),
            ts: Some(ts),
            arg_json: dmo.json.clone(),
            mbus_id: u64::MAX,
            cdata: Some(dmo.cdata.clone()),
        };

        //	match clt.channel_type {//
        //Todo: We need to make if for each channel type
        //	}

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        match self.sxclient.client.propose_demand(dm.clone()).await {
            Ok(resp) => {
                debug!("ProposeDemand Response: {:?} PID: {}", resp, pid);
            },
            Err(err) => {
                error!("{:?}.ProposeDemand err {}, [{:?}]", self, err, dm);
                return 0;
            },
        }

        self.ni.as_mut().unwrap().node_state.propose_demand(dm);

        pid
    }

    // SelectSupply send select message to server
    pub async fn select_supply(&mut self, sp: api::Supply) -> Option<u64> {
        let pid = generate_int_id().await;
        let tgt = api::Target {
            id: pid,
            sender_id: self.client_id,
            target_id: sp.id,
            channel_type: sp.channel_type,
            wait: None,
            mbus_id: u64::MAX,
        };

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        return match self.sxclient.client.select_supply(tgt.clone()).await {
            Ok(resp) => {
                debug!("SelectSupply Response: {:?} PID: {}", resp, pid);
                self.mbus_ids.write().await.push(resp.get_ref().mbus_id);
                //TODO:  We need to implement Mbus systems
                //		clt.SubscribeMbus()
                //	}
                Some(resp.get_ref().mbus_id)
            },
            Err(err) => {
                error!("{:?}.SelectSupply err {}, [{:?}]", self, err, tgt);
                None
            },
        }
    }

    // SelectDemand send select message to server
    pub async fn select_demand(&mut self, dm: api::Demand) -> Option<u64> {
        let pid = generate_int_id().await;
        let tgt = api::Target {
            id: pid,
            sender_id: self.client_id,
            target_id: dm.id,
            channel_type: dm.channel_type,
            wait: None,
            mbus_id: u64::MAX,
        };

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        return match self.sxclient.client.select_demand(tgt.clone()).await {
            Ok(resp) => {
                debug!("SelectDemand Response: {:?} PID: {}", resp, pid);
                self.mbus_ids.write().await.push(resp.get_ref().mbus_id);
                //TODO:  We need to implement Mbus systems
                //		clt.SubscribeMbus()
                //	}
                Some(resp.get_ref().mbus_id)
            },
            Err(err) => {
                error!("{:?}.SelectDemand err {}, [{:?}]", self, err, tgt);
                None
            },
        }
    }
    
    
    // SubscribeSupply  Wrapper function for SXServiceClient
    pub async fn subscribe_supply(&mut self, spcb: fn(&SXServiceClient, api::Supply)) -> bool {
        let ch = self.get_channel();
        // check status
        //	sclt := clt.SXClient.Client
        // if clt.SXClient == nil {
        //     err := errors.New("sxutil:SXClient is nil")
        //     log.Printf("sxutil: SXServiceClient.SubscribeSupply No Client Info!")
        //     return err
        // }
        
        let mut smc = match self.sxclient.client.subscribe_supply(ch).await {
            Ok(smc) => smc,
            Err(err) => {
                error!("sxutil: SXServiceClient.SubscribeSupply Error {}", err);
                return false;
            },
        };

        loop {
            let sp: api::Supply = match smc.get_mut().message().await {  // receive Supply
                Ok(msg) => msg.unwrap(),
                Err(err) => {
                    // if err == io.EOF {
                    //     log.Print("sxutil: End Supply subscribe OK")
                    // }
                    error!("sxutil: SXServiceClient SubscribeSupply error [{}]", err);
                    break;
                },
            };

            debug!("Receive SS: {:?}", sp);

            if !self.ni.as_ref().unwrap().node_state.locked {
                spcb(self, sp);
            } else {
                error!("sxutil: Provider is locked!"); // for movement
            }
        }
        
        true
    }

    // SubscribeDemand  Wrapper function for SXServiceClient
    pub async fn subscribe_demand(&mut self, dmcb: impl Fn(&SXServiceClient, api::Demand)) -> bool {
        let ch = self.get_channel();

        let mut dmc = match self.sxclient.client.subscribe_demand(ch).await {
            Ok(dmc) => dmc,
            Err(err) => {
                error!("sxutil: clt.SubscribeDemand Error [{}] {:?}", err, self);
                return false; // sender should handle error...
            },
        };

        loop {
            let dm: api::Demand = match dmc.get_mut().message().await {  // receive Demand
                Ok(msg) => msg.unwrap(),
                Err(err) => {
                    // if err == io.EOF {
                    //     log.Print("sxutil: End Demand subscribe OK")
                    // }
                    error!("sxutil: SXServiceClient SubscribeDemand error [{}]", err);
                    break;
                },
            };

            debug!("Receive SD: {:?}", dm);

            if !self.ni.as_ref().unwrap().node_state.locked {
                dmcb(self, dm);
            } else {
                error!("sxutil: Provider is locked!");
            }
        }
        
        true
    }
    
    // SubscribeMbus  Wrapper function for SXServiceClient
    pub async fn subscribe_mbus(&mut self, mbus_id: u64, mbcb: fn(&SXServiceClient, api::MbusMsg)) -> bool {

        //TODO: we need to check there is mbus in the clt.MbusIDs

        let mb = api::Mbus{
            client_id: self.client_id,
            mbus_id,
            arg_json: String::from(""),
        };

        let mut smc = match self.sxclient.client.subscribe_mbus(mb).await {
            Ok(smc) => smc,
            Err(err) => {
                error!("sxutil: Synerex_SubscribeMbusClient Error [{}] {:?}", err, self);
                return false; // sender should handle error...
            },
        };

        loop {
            let mes: api::MbusMsg = match smc.get_mut().message().await {  // receive Demand
                Ok(msg) => msg.unwrap(),
                Err(err) => {
                    // if err == io.EOF {
                    //     log.Print("sxutil: End Demand subscribe OK")
                    // }
                    error!("sxutil: SXServiceClient SubscribeMbus error [{}] {:?}", err, self);
                    break;
                },
            };

            debug!("Receive Mbus Message {:?}", mes);
            // call Callback!
            mbcb(self, mes);
        }

        true
    }
    
    // v0.4.1 name change
    pub async fn send_mbus_msg(&mut self, mbus_id: u64, mut msg: api::MbusMsg) -> Option<u64> { // return from mbus_msgID(sxutil v0.5.3)
        if self.mbus_ids.read().await.len() == 0 {
            error!("sxutil: No Mbus opened!");
            return None;
        }
        msg.msg_id = generate_int_id().await;
        msg.sender_id = self.client_id;
        msg.mbus_id = mbus_id; // uint64(clt.MbusID) // now we can use multiple mbus from v0.6.0
        //TODO: need to check response
        let resp = match self.sxclient.client.send_mbus_msg(msg).await {
            Ok(resp) => resp,
            Err(err) => {
                error!("sxutil: Error sending Mbus msg: {}", err);
                return None;
            },
        };
        if !resp.get_ref().ok {
            error!("sxutil: Error sending Mbus msg: {}", resp.get_ref().err);
            return None;
        }
    
        Some(mbus_id)
    }

    // from synerex_api v0.4.0
    pub async fn create_mbus(&mut self, opt: api::MbusOpt) -> Option<api::Mbus> {
        let mut mbus = match self.sxclient.client.create_mbus(opt).await {
            Ok(mbus) => mbus,
            Err(err) => {
                error!("sxutil: Error creating Mbus: {}", err);
                return None;
            },
        };
        mbus.get_mut().client_id = self.client_id;
        Some(mbus.into_inner())
    }
    
    // from synerex_api v0.4.0
    pub async fn get_mbus_status(&mut self, mb: api::Mbus) -> Option<api::MbusState> {
        let mbs = match self.sxclient.client.get_mbus_state(mb).await {
            Ok(mbs) => mbs,
            Err(err) => {
                error!("sxutil: Error getting MbusState: {}", err);
                return None;
            },
        };
        Some(mbs.into_inner())
    }
    
    pub async fn mbus_index(&self, id: u64) -> isize {
        let mut idx = 0;
        for mbus_id in self.mbus_ids.read().await.iter() {
            if *mbus_id == id {
                return idx;
            }
            idx += 1;
        }
        return -1;
    }
    
    pub async fn remove_mbus_index(&self, pos: usize) {
        self.mbus_ids.write().await.remove(pos);
    }

    pub async fn close_mbus(&mut self, mbus_id: u64) -> bool {
        if self.mbus_ids.read().await.len() == 0 {
            error!("sxutil: No Mbus opened!");
            return false;
        }
        let mbus = api::Mbus{
            client_id: self.client_id,
            mbus_id,
            arg_json: String::from(""),
        };
        match self.sxclient.client.close_mbus(mbus).await {
            Ok(res) => {
                debug!("{:?}", res);
            },
            Err(err) => {
                error!("sxutil: Error closing Mbus: {}", err);
                return false;
            },
        };
        let pos = self.mbus_index(mbus_id).await;
        if pos >= 0 {
            self.remove_mbus_index(pos as usize).await;
        } else {
            error!("not found mbusID[{}]\n", mbus_id);
        }

        true
    }
        
    // NotifyDemand sends Typed Demand to Server
    pub async fn notify_demand(&mut self, mut dmo: DemandOpts) -> Option<u64> {
        let id = generate_int_id().await;
        let dt = Local::now();
        let ts = Timestamp::date_time_nanos(dt.year() as i64, dt.month() as u8, dt.day() as u8, dt.hour() as u8, dt.minute() as u8, dt.second() as u8, dt.nanosecond() as u32).unwrap();
        let dm = api::Demand {
            id,
            sender_id: self.client_id,
            target_id: 0,
            channel_type: self.channel_type,
            demand_name: dmo.name.clone(),
            ts: Some(ts),
            arg_json: dmo.json.clone(),
            mbus_id: u64::MAX,
            cdata: Some(dmo.cdata.clone()),
        };

        //	match clt.channel_type {//
        //Todo: We need to make if for each channel type
        //	}

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        match self.sxclient.client.notify_demand(dm.clone()).await {
            Ok(resp) => {
                debug!("NotifyDemand Response: {:?} PID: {}", resp, id);
            },
            Err(err) => {
                error!("{:?}.NotifyDemand err {}, [{:?}]", self, err, dm);
                return None;
            },
        }

        dmo.id = id;
        Some(id)
    }
        
    // NotifySupply sends Typed Supply to Server
    pub async fn notify_supply(&mut self, mut smo: SupplyOpts) -> Option<u64> {
        let id = generate_int_id().await;
        let dt = Local::now();
        let ts = Timestamp::date_time_nanos(dt.year() as i64, dt.month() as u8, dt.day() as u8, dt.hour() as u8, dt.minute() as u8, dt.second() as u8, dt.nanosecond() as u32).unwrap();
        let sp = api::Supply {
            id,
            sender_id: self.client_id,
            target_id: 0,
            channel_type: self.channel_type,
            supply_name: smo.name.clone(),
            ts: Some(ts),
            arg_json: smo.json.clone(),
            mbus_id: u64::MAX,
            cdata: Some(smo.cdata.clone()),
        };

        //	match clt.channel_type {//
        //Todo: We need to make if for each channel type
        //	}

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        match self.sxclient.client.notify_supply(sp.clone()).await {
            Ok(resp) => {
                debug!("NotifySupply Response: {:?} PID: {}", resp, id);
            },
            Err(err) => {
                error!("{:?}.NotifySupply err {}, [{:?}]", self, err, sp);
                return None;
            },
        }

        smo.id = id;
        Some(id)
    }

    // Confirm sends confirm message to sender
    pub async fn confirm(&mut self, id: IDType, pid: IDType) -> bool {
        let tg = api::Target{
            id: generate_int_id().await,
            sender_id: self.client_id,
            target_id: id,
            channel_type: self.channel_type,
            wait: None,
            mbus_id: id,
        };

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        let resp = match self.sxclient.client.confirm(tg.clone()).await {
            Ok(resp) => resp,
            Err(err) => {
                error!("{:?}.Confirm failed {}, [{:?}]", self, err, tg);
                return false;
            },
        };

        self.mbus_ids.write().await.push(id);
        debug!("Confirm Success: {:?}", resp);

        // nodestate may not work v0.5.0.
        //	clt.NI.nodeState.selectDemand(uint64(id))
        self.ni.as_mut().unwrap().node_state.select_supply(pid);

        true
    }
}

// Simple Robust SubscribeDemand/Supply with ReConnect function. (2020/09~ v0.5.0)

pub async fn reconnect_client(client: Arc<Mutex<SXServiceClient>>, serv_addr: String) {
	// may need to reset old connection to stop redialing.
	
	// if client.SXClient != nil {
	// may need to reset old connection to stop redialing.
    // log.Printf("sxutil: Conn state: %v closeErr: %v",client.SXClient.GrpcConn.GetState(), client.SXClient.GrpcConn.Close())

    // client.SXClient = nil
    info!("sxutil:Client reset with srvaddr: {}\n", serv_addr);
	// }

    thread::sleep(time::Duration::from_secs(RECONNECT_WAIT));  // wait 5 seconds to reconnect

	if serv_addr.len() > 0 {
		let new_clt = grpc_connect_server(serv_addr.clone()).await;
		if new_clt.is_some() {
			info!("sxutil: Reconnect server [{}] {:?}\n", serv_addr, new_clt);
            client.lock().await.sxclient.server_address = new_clt.as_ref().unwrap().server_address.clone();
            client.lock().await.sxclient.client = new_clt.unwrap().client;
			// client.sxclient = new_clt.as_mut().unwrap();
		} else {
			error!("sxutil: Can't re-connect server..");
		}
	} else { // someone may connect!
		info!("sxutil: Use reconnected client.. {:?} : svadr: {}\n", client.lock().await.sxclient, serv_addr);
	}
}

// Simple Continuous (error free) subscriber for demand
pub fn simple_subscribe_demand(client: Arc<Mutex<SXServiceClient>>, dmcb: fn(&SXServiceClient, api::Demand)) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	tokio::spawn(subscribe_demand(client, dmcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}

// Continuous (error free) subscriber for demand
pub async fn subscribe_demand(client: Arc<Mutex<SXServiceClient>>, dmcb: impl Fn(&SXServiceClient, api::Demand), loop_flag: Arc<Mutex<bool>>) {
	// if client.SXClient == nil {
	// 	log.Printf("sxutil: SubscribeDemand should called with correct info")
	// }
	let mut serv_addr = client.lock().await.sxclient.server_address.clone();
	while *loop_flag.lock().await { // make it continuously working..
		let result = client.lock().await.subscribe_demand(&dmcb).await;
		//		log.Printf("sxutil:Error on subscribeDemand . %v", err)
		if result { 
			serv_addr = client.lock().await.sxclient.server_address.clone();
			info!("sxutil: SubscribeDemand: reset server address [{}]", serv_addr);
		} else {
			error!("sxutil:Error on SubscribeDemand.");
		}
		reconnect_client(Arc::clone(&client), serv_addr.clone()).await;
	}
}

// Simple Continuous (error free) subscriber for supply
pub async fn simple_subscribe_supply(client: Arc<Mutex<SXServiceClient>>, spcb: fn(&SXServiceClient, api::Supply)) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	tokio::spawn( subscribe_supply(client, spcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}

// Continuous (error free) subscriber for supply
pub async fn subscribe_supply(client: Arc<Mutex<SXServiceClient>>, spcb: fn(&SXServiceClient, api::Supply), loop_flag: Arc<Mutex<bool>>) {
	// if client.SXClient == nil || client.SXClient.ServerAddress == "" {
	// 	log.Printf("sxutil: [FATAL] SubscribeSupply should called with correct info")
	// 	return
	// }
    let mut serv_addr = client.lock().await.sxclient.server_address.clone();
	//	log.Printf("sxutil: SubscribeSupply with ServerAddress [%s]",servAddr)
	while *loop_flag.lock().await { // make it continuously working..
        let result = client.lock().await.subscribe_supply(spcb).await;  // this may block until the connection broken
		//
		if result { 
			serv_addr = client.lock().await.sxclient.server_address.clone();
			info!("sxutil: SubscribeSupply: reset server address [{}]", serv_addr);
		} else {
			error!("sxutil: SXClient is nil in SubscribeSupply.");
		}
		reconnect_client(Arc::clone(&client), serv_addr.clone()).await;
	}
}


// We need to simplify the logic of separate NotifyDemand/SelectSupply

// composit callback with selection checking
pub fn generate_demand_callback(ndcb: fn(&SXServiceClient, api::Demand), sscb: fn(&SXServiceClient, api::Demand)) -> impl Fn(&SXServiceClient, api::Demand) {
    move |clt: &SXServiceClient, dm: api::Demand| {
		if dm.target_id == 0 {
			ndcb(clt, dm);
		} else {
			//
			info!("SelectSupply: {}: {:?}", dm.target_id, clt.ni.as_ref().unwrap().node_state.proposed_supply);
            let pos = clt.ni.as_ref().unwrap().node_state.proposed_supply_index(dm.target_id);
			if pos >= 0 { // it is proposed by me.
				sscb(clt, dm);
			} else {
				info!("sxutil:Other Proposal? {}", dm.target_id);
			}
		}
	}
}

// Composit Subscriber for demand (ndcb = notify demand callback, sscb = selectsupply cb)
pub async fn combined_subscribe_demand(client: Arc<Mutex<SXServiceClient>>, ndcb: fn(&SXServiceClient, api::Demand), sscb: fn(&SXServiceClient, api::Demand)) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	let dmcb = generate_demand_callback(ndcb, sscb);
	tokio::spawn(subscribe_demand(client, dmcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}

// // composit callback with DemandHandler
// pub async fn demandHandlerCallback(dh: impl DemandHandler) -> impl Fn(&SXServiceClient, api::Demand) {
// 	let ret = async |clt: &SXServiceClient, dm: api::Demand| {
// 		if dm.target_id == 0 { // notify supply
// 			let spo = dh.on_notify_demand(clt, dm);
// 			if spo.is_some() { // register propose Id.
// 				spo.target = dm.id; // need to set!
//                 clt.propose_supply(spo);
// 				// currentry not used proposed Id.
// 			}
// 		} else { // select supply
// 			//
// 			info!("SelectSupply: {}: {}", dm.target_id, clt.ni.as_ref().unwrap().node_state.proposed_supply);
// 			let pos = clt.ni.as_ref().unwrap().node_state.proposed_supply_index(dm.target_id);
// 			if pos >= 0 { // it is proposed by me.
// 				if dh.on_select_supply(clt, dm) { // if OK. send Confirm
// 					let err = clt.confirm(dm.id as IDType, dm.target_id as IDType).await; // send confirm to sender!
// 					dh.on_confirm_response(clt, dm.id as IDType, err);
// 				} else { // no confirm.
// 					// may remove proposal.
// 				}
// 			} else {
// 				info!("sxutil:Other Proposal? {}", dm.target_id);
// 			}
// 		}
// 	};
//     ret
// }

// Register DemandHandler
// pub async fn RegisterDemandHandler(client: Arc<Mutex<SXServiceClient>>, dh: DemandHandler) -> Arc<Mutex<bool>> {
// 	let loop_flag = Arc::new(Mutex::new(true));
// 	let dmcb = demandHandlerCallback(dh).await;
// 	tokio::spawn(subscribe_demand(client, dmcb, Arc::clone(&loop_flag))); // loop
// 	return loop_flag;
// }


//
// signal.go
//

// pub struct DeferFunctions {
//     pub functions: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
// }

// static FN_SLICE: Lazy<Mutex<DeferFunctions>> = Lazy::new(|| {
//     Mutex::from(DeferFunctions {
//         functions: Vec::new(),
//     })
// });

// // register closing functions.
// pub async fn register_defer_function<T>(f: T)
// where
//     T: Future<Output = ()> + Send + 'static,
//     T::Output: Send + 'static,
// {
// 	FN_SLICE.lock().await.functions.push(Box::pin(f));
// }

// pub async fn call_defer_functions() {
// 	for mut f in &FN_SLICE.lock().await.functions.to_vec() {
// 		debug!("Calling defer functions...");
//         f.await;
// 	}
// }

pub async fn handle_sig_int() {
    ctrlc_async::set_async_handler(async {
        debug!("Received Ctrl-C");
        //call_defer_functions().await;
        un_register_node().await;
        debug!("End at HandleSigInt in sxutil");
        std::process::exit(1);
    }).expect("Error setting Ctrl-C handler");
    // let signals = signal_hook::iterator::Signals::new(&[signal_hook::SIGTERM])?;
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("Received signal {:?}", sig);
    //         CallDeferFunctions();
    //         debug!("End at HandleSigInt in sxutil");
    //         std::process::exit(1);        }
    // });
}

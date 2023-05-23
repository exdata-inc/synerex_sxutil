#[macro_use]
extern crate log;
extern crate env_logger as logger;

use std::sync::{Arc, Mutex, RwLock};

use build_time::build_time_local;
use git_version::git_version;
mod api;
mod nodeapi;
use snowflake::SnowflakeIdGenerator;

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
pub struct NodeServInfo<'a> {
    // we keep this for each nodeserver.
    pub node: SnowflakeIdGenerator, // package variable for keeping unique ID.
    pub nid: nodeapi::NodeId,
    pub nupd: RwLock<nodeapi::NodeUpdate>,
    // pub numu:      sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub my_node_name: String,
    pub my_server_info: String,
    pub my_node_type: nodeapi::NodeType,
    pub conn: Option<&'a tonic::client::Grpc<tonic::transport::Channel>>, // TODO: inspect grpc in rust
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
    pub mbus_ids: Arc<Mutex<Vec<IDType>>>,
    // pub mbusMutex:   sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub ni: &'a NodeServInfo<'a>,
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

static DEFAULT_NI: RwLock<Option<NodeServInfo>> = RwLock::new(None);

impl NodeServInfo<'_> {
    pub fn new() -> NodeServInfo<'static> {
        debug!("Initializing NodeServInfo");
        NodeServInfo {
            node_state: NodeState::new(),
            node: SnowflakeIdGenerator::new(0, 0),
            nid: nodeapi::NodeId {
                node_id: 0,
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
            conn: None,
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
    pub fn set_node_status(&self, status: i32, arg: String) {
        if let Ok(mut nupd) = self.nupd.write() {
            nupd.node_status = status;
            nupd.node_arg = arg;
        }
    }

    pub async fn reconnectNodeServ(&mut self) -> bool {
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
                self.node = snowflake::SnowflakeIdGenerator::new(0, self.nid.node_id);
                info!("Successfully ReInitialize node {}", self.nid.node_id);
                self.nupd = RwLock::new(nodeapi::NodeUpdate{
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
}

// func init()
pub fn initialize_default_ni() {
    if let Ok(mut default_ni) = DEFAULT_NI.write() {
        *default_ni = std::option::Option::<NodeServInfo>::from(NodeServInfo::new());
    }
}

// InitNodeNum for initialize NodeNum again
pub fn init_node_num(n: i32) {
    if let Ok(mut ds) = DEFAULT_NI.write() {
        ds.as_mut().unwrap().node = snowflake::SnowflakeIdGenerator::new(0, n);
        info!("Successfully Initialize node {}", n);
    }
}

// SetNodeStatus updates KeepAlive info to NodeServer
pub fn set_node_status(status: i32, arg: String) {
    if let Ok(mut default_ni) = DEFAULT_NI.write() {
        if let Ok(mut nupd) = default_ni.as_mut().unwrap().nupd.write() {
            nupd.node_status = status;
            nupd.node_arg = arg;
        }
    }
}

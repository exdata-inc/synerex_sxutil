use std::sync::{RwLock, Mutex, Arc};

use build_time::build_time_local;
use git_version::git_version;
use synerex_api;
use synerex_nodeapi;
use snowflake::SnowflakeIdGenerator;

// sxutil.go is a helper utility package for Synerex

// Helper structures for Synerex

// IDType for all ID in Synerex
type IDType = u64;

static WAIT_TIME: isize = 30;

// this is for Message Timeout for synerex server
static MSG_TIME_OUT: isize = 20; // from v0.6.1 10sec -> 20sec

static RECONNECT_WAIT: isize = 5; // from v0.6.1

// for git versions
const GitVer: &str = git_version!();
const BuildTime: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

// var (
// 	Sha1Ver   &str // sha1 version used to build the program
// )


pub struct NodeState {
	pub ProposedSupply: Vec<synerex_api::Supply>,
	pub ProposedDemand: Vec<synerex_api::Demand>,
	pub Locked:         bool,
}

// NodeservInfo is a connection info for each Node Server
pub struct NodeServInfo { // we keep this for each nodeserver.
	pub node:         &snowflake::SnowflakeIdGenerator, // package variable for keeping unique ID.
	pub nid:          &synerex_nodeapi::NodeID,
	pub nupd:         Arc<Mutex<&synerex_nodeapi::NodeUpdate>>,
	// pub numu:      sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
	pub myNodeName:   &str,
	pub myServerInfo: &str,
	pub myNodeType:   synerex_nodeapi::NodeType,
	// pub conn:         &grpc.ClientConn,  // TODO: inspect grpc in rust
	pub clt:          synerex_nodeapi::NodeClient,
	pub msgCount:     u64,
	pub nodeState:    &NodeState,
}

// DemandOpts is sender options for Demand
pub struct DemandOpts {
	pub ID:     u64,
	pub Target: u64,
	pub Name:   &str,
	pub JSON:   &str,
	pub Cdata:  synerex_api::Content,
}

// SupplyOpts is sender options for Supply
pub struct SupplyOpts {
	pub ID:     u64,
	pub Target: u64,
	pub Name:   &str,
	pub JSON:   &str,
	pub Cdata:  synerex_api::Content,
}

pub trait DemandHandler {
	fn OnNotifyDemand(&SXServiceClient, &synerex_api::Demand) -> *SupplyOpts; // if propose return proposedID
	fn OnSelectSupply(&SXServiceClient, &synerex_api::Demand) -> bool;        // if confirm return true
	fn OnConfirmResponse(&SXServiceClient, IDType, error);        // result of confirm
}

pub trait SupplyHandler {
}

static defaultNI: RwLock<NodeServInfo> = RwLock::new(NodeServInfo{ // we keep this for each nodeserver.
	node:         &snowflake::SnowflakeIdGenerator::new(0, 0), // package variable for keeping unique ID.
	nid:          &synerex_nodeapi::NodeID::new(),
	nupd:         Arc::new(Mutex::new(&synerex_nodeapi::NodeUpdate::new())),
	myNodeName:   "",
	myServerInfo: "",
	myNodeType:   synerex_nodeapi::NodeType::GATEWAY,
	conn:         &grpc.ClientConn,
	clt:          synerex_nodeapi::NodeClient,
	msgCount:     0,
	nodeState:    &NodeState{
        ProposedSupply: Vec::new(),
        ProposedDemand: Vec::new(),
        Locked:         false,
    },
});

pub struct SxServerOpt {
	pub NodeType:   synerex_nodeapi::NodeType,
	pub ServerInfo: &str,
	pub ClusterId:  i32,
	pub AreaId:     &str,
	pub GwInfo:     &str,
}



fn main() {
    println!("Hello, world!");
}

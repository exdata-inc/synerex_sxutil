#[macro_use]
extern crate log;
extern crate env_logger as logger;

use core::time::Duration;
use ticker::Ticker;
use tokio::sync::{RwLock, Mutex};
use std::{sync::Arc, error::Error, fmt, pin::Pin}; //, future::Future};
use once_cell::sync::Lazy;

use build_time::build_time_local;
use git_version::git_version;
use systemstat::{Platform, System};

use synerex_api::api;
use synerex_nodeapi::nodeapi;

mod nodestate;
mod nodeservinfo;
pub use nodeservinfo::NodeServInfo;
mod sxserviceclient;
pub use sxserviceclient::SXServiceClient;

// sxutil is a helper utility package for Synerex

// Helper structures for Synerex

// IDType for all ID in Synerex
type IDType = u64;

static WAIT_TIME: u64 = 30;

// this is for Message Timeout for synerex server
static MSG_TIME_OUT: u64 = 20; // from v0.6.1 10sec -> 20sec

static RECONNECT_WAIT: u64 = 5; // from v0.6.1

const GIT_VER: &str = git_version!();
const BUILD_TIME: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

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
    pub client: RwLock<api::synerex_client::SynerexClient<tonic::transport::Channel>>,
}


pub struct SxServerOpt {
    pub node_type: nodeapi::NodeType,
    pub server_info: String,
    pub cluster_id: i32,
    pub area_id: String,
    pub gw_info: String,
}


#[derive(Debug, Clone)]
struct SxutilError;

impl fmt::Display for SxutilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for SxutilError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// func init()
static DEFAULT_NI: Lazy<Arc<RwLock<NodeServInfo>>> = Lazy::new(|| {
    debug!("sxutil: {} built {}", GIT_VER, BUILD_TIME);
    Arc::from(RwLock::new(NodeServInfo::new()))
});

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
    let keepalive_duration = DEFAULT_NI.read().await.nid.keepalive_duration as u64;
    loop {
        DEFAULT_NI.write().await.msg_count = 0; // how count message?
        {
            debug!(
                "KeepAlive {} {}",
                // self.nupd.read().as_ref().unwrap().node_status,
                DEFAULT_NI.read().await.nupd.read().await.node_status,
                keepalive_duration
            );
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(keepalive_duration)).await;
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
            DEFAULT_NI.read().await.nupd.write().await.status = Some(status);
        }

        {
            DEFAULT_NI.read().await.nupd.write().await.update_count += 1;
        }

        let nupd_clone = DEFAULT_NI.read().await.nupd.read().await.clone();
        let nodeclt_arc = Arc::clone(&DEFAULT_NI.read().await.nodeclt.as_ref().unwrap());

        let fut = nodeclt_arc.lock().await.keep_alive(nupd_clone).await;

        match fut {
            Ok(resp) => {
                // there might be some errors in response
                debug!("KeepAlive Response: {:?}", resp.get_ref().command());
                match resp.get_ref().command() {
                    nodeapi::KeepAliveCommand::None => {}
                    nodeapi::KeepAliveCommand::Reconnect => {
                        // order is reconnect to node.
                        match DEFAULT_NI.write().await.reconnect_node_serv().await {
                            Ok(_) => {},
                            Err(_) => { error!("Above error was happend when nodeapi::KeepAliveCommand::Reconnect for DEFAULT_NI") },
                        };
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
		client: RwLock::from(client),
	})
}

// NewSXServiceClient Creates wrapper structre SXServiceClient from SynerexClient
pub async fn new_sx_service_client(clt: SXSynerexClient, mtype: u32, arg_json: String) -> SXServiceClient {
    let client_id = DEFAULT_NI.write().await.generate_int_id();
    // sxServiceClient.ni = Some(&DEFAULT_NI);
    SXServiceClient {
        client_id,
        channel_type: mtype,
        sxclient: RwLock::from(Some(clt)),
        arg_json,
        mbus_ids: RwLock::from(Vec::new()),
        ni: Some(Arc::clone(&*DEFAULT_NI)),
    }
	// return defaultNI.NewSXServiceClient(clt, mtype, argJson)
}

// GenerateIntID for generate uniquie ID
pub async fn generate_int_id() -> u64 {
    DEFAULT_NI.write().await.generate_int_id()
}

// Simple Robust SubscribeDemand/Supply with ReConnect function. (2020/09~ v0.5.0)

pub async fn reconnect_client(client: Arc<RwLock<SXServiceClient>>, serv_addr: String) {
	// may need to reset old connection to stop redialing.
	
	if client.read().await.sxclient.read().await.is_some() {
        // may need to reset old connection to stop redialing.
        *client.read().await.sxclient.write().await = None;
        info!("sxutil:Client reset with srvaddr: {}\n", serv_addr);
	}

    tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_WAIT)).await;  // wait 5 seconds to reconnect

	if serv_addr.len() > 0 {
		let new_clt = grpc_connect_server(serv_addr.clone()).await;
		if new_clt.is_some() && client.read().await.sxclient.read().await.is_some() {
			info!("sxutil: Reconnect server [{}] {:?}\n", serv_addr, new_clt);
			*client.read().await.sxclient.write().await = new_clt;
		} else {
			error!("sxutil: Can't re-connect server..");
		}
	} else { // someone may connect!
		info!("sxutil: Use reconnected client.. {:?} : svadr: {}\n", client.read().await.sxclient.read().await, serv_addr);
	}
}

// Type definition of DemandHandler
pub type DemandHandler = Pin<Box<dyn Fn(&SXServiceClient, api::Demand) -> futures::future::BoxFuture<()> + Send + Sync>>;

// Simple Continuous (error free) subscriber for demand
pub fn simple_subscribe_demand(client: Arc<RwLock<SXServiceClient>>, dmcb: DemandHandler) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	tokio::spawn(subscribe_demand(Arc::clone(&client), dmcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}

// Continuous (error free) subscriber for demand
pub async fn subscribe_demand(client: Arc<RwLock<SXServiceClient>>, dmcb: DemandHandler, loop_flag: Arc<Mutex<bool>>) {
    if client.read().await.sxclient.read().await.is_none() || client.read().await.sxclient.read().await.as_ref().unwrap().server_address == "" {
        error!("sxutil: SubscribeDemand should called with correct info!");
        return;
    }
	let mut serv_addr = client.read().await.sxclient.read().await.as_ref().unwrap().server_address.clone();
	while *loop_flag.lock().await { // make it continuously working..
		let result = client.read().await.subscribe_demand(&dmcb).await;
		//		log.Printf("sxutil:Error on subscribeDemand . %v", err)
		if result && client.read().await.sxclient.read().await.is_some() { 
			serv_addr = client.read().await.sxclient.read().await.as_ref().unwrap().server_address.clone();
			info!("sxutil: SubscribeDemand: reset server address [{}]", serv_addr);
		} else {
			error!("sxutil:Error on SubscribeDemand.");
		}
		reconnect_client(Arc::clone(&client), serv_addr.clone()).await;
	}
}

// Type definition of SupplyHandler
pub type SupplyHandler = Pin<Box<dyn Fn(&SXServiceClient, api::Supply) -> futures::future::BoxFuture<()> + Send + Sync>>;


// Simple Continuous (error free) subscriber for supply
pub fn simple_subscribe_supply(client: Arc<RwLock<SXServiceClient>>, spcb: SupplyHandler) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	tokio::spawn( subscribe_supply(Arc::clone(&client), spcb, Arc::clone(&loop_flag))); // loop
	loop_flag
}

// Continuous (error free) subscriber for supply
pub async fn subscribe_supply(client: Arc<RwLock<SXServiceClient>>, spcb: SupplyHandler, loop_flag: Arc<Mutex<bool>>) {
    if client.read().await.sxclient.read().await.is_none() || client.read().await.sxclient.read().await.as_ref().unwrap().server_address == "" {
        error!("sxutil: SubscribeSupply should called with correct info!");
        return;
    }
    let mut serv_addr = client.read().await.sxclient.read().await.as_ref().unwrap().server_address.clone();
	//	log.Printf("sxutil: SubscribeSupply with ServerAddress [%s]",servAddr)
	while *loop_flag.lock().await { // make it continuously working..
        let result = client.read().await.subscribe_supply(&spcb).await;  // this may block until the connection broken
		//
		if result { 
			serv_addr = client.read().await.sxclient.read().await.as_ref().unwrap().server_address.clone();
			info!("sxutil: SubscribeSupply: reset server address [{}]", serv_addr);
		} else {
			error!("sxutil: SXClient is nil in SubscribeSupply.");
		}
		reconnect_client(Arc::clone(&client), serv_addr.clone()).await;
	}
}


// We need to simplify the logic of separate NotifyDemand/SelectSupply

// composit callback with selection checking
pub fn generate_demand_callback(ndcb: Arc<fn(&SXServiceClient, api::Demand)>, sscb: Arc<fn(&SXServiceClient, api::Demand)>) -> DemandHandler {
    let async_fn_ptr: DemandHandler = Box::pin(move |clt: &SXServiceClient, dm: api::Demand| {
        let ndcb = ndcb.clone();
        let sscb = sscb.clone();
        Box::pin(async move {
            if dm.target_id == 0 {
                ndcb(clt, dm);
            } else {
                //
                info!("SelectSupply: {}: {:?}", dm.target_id, clt.ni.as_ref().unwrap().read().await.node_state.proposed_supply);
                let pos = clt.ni.as_ref().unwrap().write().await.node_state.proposed_supply_index(dm.target_id);
                if pos >= 0 { // it is proposed by me.
                    sscb(clt, dm);
                } else {
                    info!("sxutil:Other Proposal? {}", dm.target_id);
                }
            }    
        })
    });

    async_fn_ptr
}

// Composit Subscriber for demand (ndcb = notify demand callback, sscb = selectsupply cb)
pub async fn combined_subscribe_demand(client: Arc<RwLock<SXServiceClient>>, ndcb: Arc<fn(&SXServiceClient, api::Demand)>, sscb: Arc<fn(&SXServiceClient, api::Demand)>) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	let dmcb = generate_demand_callback(ndcb, sscb);
	tokio::spawn(subscribe_demand(client, dmcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}


pub struct DemandCallbackAsync {
    pub on_notify_demand: Pin<Box<dyn for<'a> Fn(&'a SXServiceClient, &'a api::Demand) -> futures::future::BoxFuture<'a, Option<SupplyOpts>> + Send + Sync>>,
    pub on_select_supply: Pin<Box<dyn for<'a> Fn(&'a SXServiceClient, &'a api::Demand) -> futures::future::BoxFuture<'a, bool> + Send + Sync>>,
    pub on_confirm_response: Pin<Box<dyn Fn(&SXServiceClient, IDType, Option<Box<dyn std::error::Error>>) -> futures::future::BoxFuture<()> + Send + Sync>>,
}

// composit callback with DemandHandler
pub fn demand_handler_callback(dh: Arc<DemandCallbackAsync>) -> DemandHandler {
    let async_fn_ptr: DemandHandler = Box::pin(move |clt: &SXServiceClient, dm: api::Demand| {
        let dh = dh.clone();
        Box::pin(async move {
            if dm.target_id == 0 { // notify supply
                let mut spo = (dh.on_notify_demand)(clt, &dm).await;
                if spo.is_some() { // register propose Id.
                    spo.as_mut().unwrap().target = dm.id; // need to set!
                    clt.propose_supply(spo.as_ref().unwrap()).await;
                    // currentry not used proposed Id.
                }
            } else { // select supply
                //
                info!("SelectSupply: {}: {:?}", dm.target_id, clt.ni.as_ref().unwrap().read().await.node_state.proposed_supply);
                let pos = clt.ni.as_ref().unwrap().read().await.node_state.proposed_supply_index(dm.target_id);
                if pos >= 0 { // it is proposed by me.
                    if (dh.on_select_supply)(clt, &dm).await { // if OK. send Confirm
                        match clt.confirm(dm.id as IDType, dm.target_id as IDType).await {
                            Ok(_) => {
                                (dh.on_confirm_response)(clt, dm.id as IDType, None);
                            },
                            Err(err) => {
                                (dh.on_confirm_response)(clt, dm.id as IDType, Some(err));
                            },
                        }; // send confirm to sender!
                    } else { // no confirm.
                        // may remove proposal.
                    }
                } else {
                    info!("sxutil:Other Proposal? {}", dm.target_id);
                }
            }    
        })
    });

    async_fn_ptr
}

// Register DemandHandler
pub async fn register_demand_handler(client: Arc<RwLock<SXServiceClient>>, dh: Arc<DemandCallbackAsync>) -> Arc<Mutex<bool>> {
	let loop_flag = Arc::new(Mutex::new(true));
	let dmcb = demand_handler_callback(dh);
	tokio::spawn(subscribe_demand(client, dmcb, Arc::clone(&loop_flag))); // loop
	return loop_flag;
}


//
// signal.go
//

pub struct DeferFunctions {
    pub functions: Vec<Arc<DeferFunction>>,
}

pub struct DeferFunction {
    pub func: Pin<Box<dyn Fn() -> futures::future::BoxFuture<'static, ()> + Send + Sync>>,
}

static FN_SLICE: Lazy<Mutex<DeferFunctions>> = Lazy::new(|| {
    Mutex::from(DeferFunctions {
        functions: Vec::new(),
    })
});

// register closing functions.
pub async fn register_defer_function(func: Arc<DeferFunction>)
{
	FN_SLICE.lock().await.functions.push(func);
}

pub async fn call_defer_functions() {
	for f in &FN_SLICE.lock().await.functions {
		debug!("Calling defer functions...");
        (f.func)().await;
	}
}

pub async fn handle_sig_int() {
    ctrlc_async::set_async_handler(async {
        debug!("Received Ctrl-C");
        call_defer_functions().await;
        debug!("End at HandleSigInt in sxutil");
        std::process::exit(1);
    }).expect("Error setting Ctrl-C handler");
    // let signals = signal_hook::iterator::Signals::new(&[signal_hook::SIGTERM])?;
    // thread::spawn(move || {
    //     for sig in signals.forever() {
    //         println!("Received signal {:?}", sig);
    //         call_defer_functions().await;
    //         debug!("End at HandleSigInt in sxutil");
    //         std::process::exit(1);
    //     }
    // });
}

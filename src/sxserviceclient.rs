use chrono::{Local, Datelike, Timelike};
use prost_types::Timestamp;
use tokio::sync::RwLock;
use tokio::time::{timeout};
use std::{time, sync::Arc, error::Error, pin::Pin}; //, future::Future};

use synerex_api::api;

use crate::{IDType, SXSynerexClient, NodeServInfo, SupplyOpts, generate_int_id, MSG_TIME_OUT, DemandOpts, SxutilError};


// SXServiceClient Wrappter Structure for synerex client
#[derive(Debug)]
pub struct SXServiceClient {
    pub client_id: IDType,
    pub channel_type: u32,
    pub sxclient: Option<SXSynerexClient>,
    pub arg_json: String,
    pub mbus_ids: RwLock<Vec<IDType>>,
    // pub mbusMutex:   sync.RWMutex,  // TODO: Rewrite using https://fits.hatenablog.com/entry/2020/11/22/213250
    pub ni: Option<Arc<RwLock<NodeServInfo>>>,
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

        let async_func = || async {
            if self.sxclient.is_some() {
                let pid = match self.sxclient.as_mut().unwrap().client.propose_supply(sp.clone()).await {
                    Ok(resp) => {
                        debug!("ProposeSupply Response: {:?} PID: {}", resp, pid);
                        pid
                    },
                    Err(err) => {
                        error!("{:?}.ProposeSupply err {}, [{:?}]", self, err, sp);
                       0
                    },
                };
                self.ni.as_mut().unwrap().write().await.node_state.propose_supply(sp);
                pid
            } else {
                error!("SXClient is None!");
                0
            }
        };

        let timeout_duration = time::Duration::from_secs(MSG_TIME_OUT);
        let result = timeout(timeout_duration, async_func()).await;
    
        match result {
            Ok(value) => value,
            Err(_) =>  {
                error!("Timeout occurred.");
                0
            },
        }
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

        if self.sxclient.is_some() {
            match self.sxclient.as_mut().unwrap().client.propose_demand(dm.clone()).await {
                Ok(resp) => {
                    debug!("ProposeDemand Response: {:?} PID: {}", resp, pid);
                },
                Err(err) => {
                    error!("{:?}.ProposeDemand err {}, [{:?}]", self, err, dm);
                    return 0;
                },
            }
        } else {
            error!("SXClient is None!");
        }

        self.ni.as_mut().unwrap().write().await.node_state.propose_demand(dm);

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

        if self.sxclient.is_some() {
            return match self.sxclient.as_mut().unwrap().client.select_supply(tgt.clone()).await {
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
        } else {
            None
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

        if self.sxclient.is_some() {
            return match self.sxclient.as_mut().unwrap().client.select_demand(tgt.clone()).await {
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
        } else {
            None
        } 
    }
        
    
    // SubscribeSupply  Wrapper function for SXServiceClient
    pub async fn subscribe_supply(&mut self, spcb_async: Arc<Pin<Box<dyn Fn(&SXServiceClient, api::Supply) -> futures::future::BoxFuture<()> + Send + Sync>>>) -> bool {
        let ch = self.get_channel();
        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return false;
        }
        
        let mut smc = match self.sxclient.as_mut().unwrap().client.subscribe_supply(ch).await {
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

            if !self.ni.as_ref().unwrap().write().await.node_state.locked {
                spcb_async(self, sp).await;
            } else {
                error!("sxutil: Provider is locked!"); // for movement
            }
        }
        
        true
    }


    // SubscribeDemand  Wrapper function for SXServiceClient
    pub async fn subscribe_demand(&mut self, dmcb: &Pin<Box<dyn Fn(&mut SXServiceClient, api::Demand) -> futures::future::BoxFuture<()> + Send + Sync>>) -> bool {
        let ch = self.get_channel();
        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return false;
        }

        let mut dmc = match self.sxclient.as_mut().unwrap().client.subscribe_demand(ch).await {
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

            if !self.ni.as_ref().unwrap().write().await.node_state.locked {
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

        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return false;
        }

        let mut smc = match self.sxclient.as_mut().unwrap().client.subscribe_mbus(mb).await {
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

        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return None;
        }

        //TODO: need to check response
        let resp = match self.sxclient.as_mut().unwrap().client.send_mbus_msg(msg).await {
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
        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return None;
        }

        let mut mbus = match self.sxclient.as_mut().unwrap().client.create_mbus(opt).await {
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
        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return None;
        }

        let mbs = match self.sxclient.as_mut().unwrap().client.get_mbus_state(mb).await {
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
        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return false;
        }
        match self.sxclient.as_mut().unwrap().client.close_mbus(mbus).await {
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

        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return None;
        }

        match self.sxclient.as_mut().unwrap().client.notify_demand(dm.clone()).await {
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

        debug!("NotifySupply: {:?}", sp);

        //	match clt.channel_type {//
        //Todo: We need to make if for each channel type
        //	}

        // ctx, cancel := context.WithTimeout(context.Background(), MSG_TIME_OUT*time.Second)
        // defer cancel()

        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return None;
        }

        match self.sxclient.as_mut().unwrap().client.notify_supply(sp.clone()).await {
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
    pub async fn confirm(&mut self, id: IDType, pid: IDType) -> Result<(), Box<dyn Error>> {
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

        if self.sxclient.is_none() {
            error!("sxutil: SXClient is None!");
            return Err(Box::from(SxutilError));
        }

        let resp = match self.sxclient.as_mut().unwrap().client.confirm(tg.clone()).await {
            Ok(resp) => resp,
            Err(err) => {
                error!("{:?}.Confirm failed {}, [{:?}]", self, err, tg);
                return Err(Box::from(err))
            },
        };

        self.mbus_ids.write().await.push(id);
        debug!("Confirm Success: {:?}", resp);

        // nodestate may not work v0.5.0.
        //	clt.NI.nodeState.selectDemand(uint64(id))
        self.ni.as_mut().unwrap().write().await.node_state.select_supply(pid);

        Ok(())
    }
}
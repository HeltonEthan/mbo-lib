use dbn::MboMsg;

use crate::api::action::cancel_request::CancelRequest;
use crate::api::action::modify_request::ModifyRequest;
use crate::api::action::trade_request::TradeRequest;
use crate::api::latency::LatencyModel;
use crate::prelude::ack::Ack;

pub mod cancel_request;
pub mod modify_request;
pub mod trade_request;

pub enum Request {
    Trade(TradeRequest),
    Cancel(CancelRequest),
    Modify(ModifyRequest),
}

impl Request {
    pub fn process<LM: LatencyModel>(self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        match self {
            Request::Trade(request) => request.submit(mbo, latency),
            Request::Cancel(request) => request.submit(mbo, latency),
            Request::Modify(request) => request.submit(mbo, latency),
        }
    }
}

pub trait Submit {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack;

    fn check_request(&self) -> Ack;
}

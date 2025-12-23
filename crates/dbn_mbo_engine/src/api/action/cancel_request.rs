use super::*;

#[derive(Debug)]
pub struct CancelRequest {
    pub instrument_id: u32,
    pub order_id: u64,
}

impl CancelRequest {}

impl Submit for CancelRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        todo!()
    }
    
    fn check_request(&self) -> Ack {
        todo!()
    }
}

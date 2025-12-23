use super::*;

#[derive(Debug)]
pub struct ModifyRequest {
    pub instrument_id: u32,
    pub order_id: u64,
    pub new_price: Option<i64>,
    pub new_size: Option<u32>,
}

impl ModifyRequest {}

impl Submit for ModifyRequest {
    fn submit<LM: LatencyModel>(&self, mbo: &MboMsg, latency: &mut LM) -> Ack {
        todo!()
    }
    
    fn check_request(&self) -> Ack {
        todo!()
    }
}

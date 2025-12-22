use dbn::{MboMsg, Side};

use crate::api::latency::LatencyModel;

/// Action types for making orders
pub enum Request {
    Order(u16, u32, Side, i64, u32), // (publisher_id, instrument_id, side, price, size)
    Cancel(u64),           // (order_id)
    Modify(u64),           // (order_id)
}

#[allow(dead_code)]
pub struct OrderRequest {
    order_id: u64,
    ts_recv: u64,
    ts_event: u64,
    request: Request,
}

impl OrderRequest {
    pub fn new<L: LatencyModel>(request: Request, mbo: &MboMsg, latency: &mut L) -> Self {
        let ts_event = mbo.ts_recv;
        let ts_recv = latency.ts_recv_sim(ts_event);
        Self {
            order_id: Self::set_order_id(mbo),
            ts_recv: ts_recv,
            ts_event: ts_event,
            request,
        }
    }

    pub fn set_order_id(mbo: &MboMsg) -> u64 {
        let _ = mbo;
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::api::latency::UnitNormalLatency;

    #[test]
    fn slippage() -> anyhow::Result<()> {
        let mut latency = UnitNormalLatency::new(25_000_000, 1_000_000);
        let ts_events: [u64; 5] = [1766368150000000000; 5];
        println!("=== Latency ===");
        for ts in ts_events {
            let ts_recv = latency.ts_recv_sim(ts);
            println!("{:#?}", ts_recv - ts);
        }
        Ok(())
    }
}

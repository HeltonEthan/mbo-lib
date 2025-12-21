use dbn::{MboMsg, Side};

/// Action types for making orders
pub enum Action {
    Order(Side, i64, u32), // (side, price, size)
    Cancel(u64),           // (order_id)
    Modify(u64),           // (order_id)
}

pub struct Order {
    order_id: u64,
    ts_event: u64,
    action: Action,
}

impl Order {
    pub fn new(action: Action, mbo: &MboMsg) -> Self {
        Self {
            order_id: 0,
            ts_event: 0,
            action,
        }
    }

    fn order_id(mbo: &MboMsg) -> u64 {
        todo!()
    }

    fn ts_event(mbo: &MboMsg) -> u64 {
        todo!()
    }
}

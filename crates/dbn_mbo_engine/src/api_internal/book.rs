use dbn::{Action, Side};
use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::api_internal::order::Order;

#[derive(Debug, Default)]
pub struct Book {
    queue_by_action: HashMap<Action, u64>,
    trade: BTreeMap<u64, Level>,
    cancel: BTreeMap<u64, Level>,
    modify: BTreeMap<u64, Level>,
    orders_by_id: HashMap<u64, (Side, i64)>,
    offers: BTreeMap<i64, Level>,
    bids: BTreeMap<i64, Level>,
}

type Level = VecDeque<Order>;

impl Book {
    pub fn apply(&mut self, action: Action, order: Order) {
        match action {
            Action::Trade => {},
            Action::Cancel => {},
            Action::Modify => {},
            _ => {},
        }
    }

    fn trade(&mut self) {
        todo!()
    }

    fn cancel(&mut self) {
        todo!()
    }

    fn modify(&mut self) {
        todo!()
    }

    fn clear(&mut self) {
        self.orders_by_id.clear()
    }
}

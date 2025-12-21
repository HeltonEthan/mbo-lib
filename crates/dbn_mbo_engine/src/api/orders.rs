use dbn::Side;
use std::collections::HashMap;

use crate::api::action;

pub struct Orders {
    orders_by_id: HashMap<u64, (Side, i64)>,
}

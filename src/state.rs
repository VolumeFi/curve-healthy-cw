use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Metadata;
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub retry_delay: u64,
    pub job_id: String,
    pub owner: Addr,
    pub metadata: Metadata,
}

pub const WITHDRAW_TIMESTAMP: Map<(String, String), Timestamp> = Map::new("withdraw_timestamp");
pub const STATE: Item<State> = Item::new("state");

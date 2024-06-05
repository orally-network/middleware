use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct MiddlewareInstance {
    pub id: u64,
    pub canister_id: Principal,
    pub user: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct AddMiddlewareInstanceRequest {}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct GetMiddlewareInstancesFilter {
    pub chain_id: Option<Nat>,
    pub search: Option<String>,
}

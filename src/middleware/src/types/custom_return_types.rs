use candid::CandidType;

use super::middleware_instance::MiddlewareInstance;

#[derive(Debug, CandidType, Clone)]
pub struct GetMiddlewareInstanceResult {
    pub id: u64,
    pub middleware_instance_example: MiddlewareInstance,
}

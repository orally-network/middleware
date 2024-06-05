use candid::candid_method;
use ic_cdk::update;

use crate::call_middleware_instance;
use crate::{utils::canister::validate_caller, MiddlewareResult};

#[candid_method]
#[update]
async fn call_middleware_instance(id: u64, args: Vec<u8>) -> MiddlewareResult<Vec<u8>> {
    validate_caller()?;

    call_middleware_instance!(call(id, args) => Result<Vec<u8>, String>).await
    // .map_err(|err| MiddlewareError::MiddlewareInstanceError(err.to_string()))
}

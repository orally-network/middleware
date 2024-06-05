use candid::Principal;
use ic_cdk::{api::is_controller, query, update};

use crate::types::errors::UtilsError;

pub fn validate_caller() -> Result<(), UtilsError> {
    if is_controller(&ic_cdk::caller()) {
        return Ok(());
    }

    Err(UtilsError::NotAController)
}

fn _validate_canistergeek_caller() {
    match Principal::from_text("xcxtd-2qaaa-aaaah-qabfa-cai") {
        Ok(caller) if caller == ic_cdk::caller() => (),
        _ => ic_cdk::trap("Invalid caller"),
    }
}

#[query(name = "getCanistergeekInformation")]
pub async fn get_canistergeek_information(
    request: ic_utils::api_type::GetInformationRequest,
) -> ic_utils::api_type::GetInformationResponse<'static> {
    ic_utils::get_information(request)
}

#[update(name = "updateCanistergeekInformation")]
pub async fn update_canistergeek_information(
    request: ic_utils::api_type::UpdateInformationRequest,
) -> () {
    ic_utils::update_information(request);
}

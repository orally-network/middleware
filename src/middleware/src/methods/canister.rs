use crate::log;
use crate::methods::INIT_CYCLES_BALANCE;
use crate::types::errors::MiddlewareError;
use crate::types::middleware_instance::MiddlewareInstance;
use crate::types::UpdateMetadata;
use crate::types::{Metadata, STATE};
use crate::utils::address::normalize;
use crate::utils::canister::validate_caller;
use crate::utils::memory::Cbor;
use crate::utils::new_id;
use crate::utils::pagination::{Pagination, PaginationResult};
use candid::{candid_method, Principal};
use ic_cdk::api::management_canister::main::{
    canister_status, create_canister, delete_canister, install_code, stop_canister,
    update_settings, CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument,
    UpdateSettingsArgument,
};
use ic_cdk::api::management_canister::provisional::{CanisterIdRecord, CanisterSettings};
use ic_cdk::{query, update};

use crate::{GetMiddlewareInstanceResult, MiddlewareResult};

#[candid_method]
#[query]
fn get_metadata() -> Metadata {
    STATE.with(|s| s.borrow().metadata.get().0.clone())
}

#[candid_method]
#[update]
fn update_metadata(update_metadata_args: UpdateMetadata) -> Result<(), String> {
    _update_metadata(update_metadata_args)
        .map_err(|e| format!("failed to update the metadata: {e:?}"))
}

fn _update_metadata(update_metadata_args: UpdateMetadata) -> MiddlewareResult<()> {
    validate_caller()?;
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let mut metadata = state.metadata.get().0.clone();
        metadata.update(update_metadata_args);
        state.metadata.set(Cbor(metadata)).unwrap();
    });

    Ok(())
}

#[candid_method]
#[update]
async fn send_cycles_from_factory(destination: Principal, amount: u128) -> Result<(), String> {
    _send_cycles_from_factory(destination, amount)
        .await
        .map_err(|e| format!("failed to send cycles: {e:?}"))
}

#[inline]
async fn _send_cycles_from_factory(
    destination: Principal,
    amount: u128,
) -> Result<(), MiddlewareError> {
    validate_caller()?;

    log!("Sending {} cycles to {}", amount, destination);
    ic_cdk::api::management_canister::main::deposit_cycles(
        CanisterIdRecord {
            canister_id: destination,
        },
        amount,
    )
    .await
    .map_err(|(_, err)| MiddlewareError::FailedToSendCycles(err))?;

    log!("Send {} cycles to {}", amount, destination);

    Ok(())
}

#[candid_method]
#[query]
fn get_middleware_instances(
    pagination: Option<Pagination>,
) -> PaginationResult<GetMiddlewareInstanceResult> {
    let mut middleware_instances: Vec<_> = STATE.with(|s| {
        s.borrow()
            .middlewares
            .iter()
            .map(|(k, v)| GetMiddlewareInstanceResult {
                id: k,
                middleware_instance_example: v.0.clone(),
            })
            .collect()
    });

    match pagination {
        Some(pagination) => {
            middleware_instances.sort_by(|l, r| l.id.cmp(&r.id));
            pagination.paginate(middleware_instances)
        }
        None => middleware_instances.into(),
    }
}

#[candid_method]
#[update]
fn add_middleware_instances_manually(
    middleware_instances: Vec<MiddlewareInstance>,
) -> Result<(), String> {
    _add_middleware_instances_manually(middleware_instances)
        .map_err(|e| format!("failed to add middleware instances manually: {e:?}"))
}

#[inline]
fn _add_middleware_instances_manually(
    middleware_instances: Vec<MiddlewareInstance>,
) -> MiddlewareResult<()> {
    validate_caller()?;

    log!("ADDING APOLLO INSTANCES MANUALLY");
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        for middleware_instance_example in middleware_instances {
            state.middlewares.insert(
                middleware_instance_example.id,
                Cbor(middleware_instance_example),
            );
        }
    });

    Ok(())
}

#[candid_method]
#[update]
async fn add_middleware_instance(wasm: Vec<u8>, user: String) -> MiddlewareResult<u64> {
    validate_caller()?;
    let user = normalize(&user)?;

    let available = ic_cdk::api::call::msg_cycles_available128();

    if available < INIT_CYCLES_BALANCE {
        return Err(MiddlewareError::NotEnoughCycles(available, INIT_CYCLES_BALANCE).into());
    }

    log!(
        "Accepted {} cycles",
        ic_cdk::api::call::msg_cycles_accept128(INIT_CYCLES_BALANCE)
    );

    let id = new_id();
    log!("Adding middleware for user: {}", user);

    let canister_id = match create_canister(
        CreateCanisterArgument { settings: None },
        INIT_CYCLES_BALANCE,
    )
    .await
    {
        Ok((canister_id,)) => canister_id.canister_id,
        Err((_, error)) => {
            return Err(MiddlewareError::FailedToCreate(error.to_string()).into());
        }
    };

    match install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: wasm.to_vec(),
        // arg: encode_args(payload).unwrap(),
        arg: vec![],
    })
    .await
    {
        Ok(()) => {
            log!("Code installed in middleware instance with id: {}", id);

            STATE.with(|s| {
                let mut state = s.borrow_mut();
                let middleware_instance_example = MiddlewareInstance {
                    id,
                    user: user.clone(),
                    canister_id,
                };

                state
                    .middlewares
                    .insert(id, Cbor(middleware_instance_example));
            });
        }
        Err((_, error)) => {
            return Err(MiddlewareError::FailedToInstallCode(error.to_string()).into());
        }
    }

    log!("Middleware {} added for user {}", id, user);

    let (middleware_canister_status,) = canister_status(CanisterIdRecord {
        canister_id: ic_cdk::id(),
    })
    .await
    .map_err(|(_, err)| MiddlewareError::FailedToGetCanisterStatus(err.to_string()))?;

    let mut controllers = middleware_canister_status.settings.controllers;
    controllers.push(ic_cdk::id());

    update_settings(UpdateSettingsArgument {
        canister_id,
        settings: CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        },
    })
    .await
    .map_err(|(_, err)| MiddlewareError::FailedToUpdateSettings(err.to_string()))?;

    log!("Controllers updated for canister: {}", canister_id);

    Ok(id)
}

#[candid_method]
#[update]
// TODO: where do cycles go ?
async fn remove_middleware_instance(id: u64) -> MiddlewareResult<()> {
    validate_caller()?;

    log!("Removing middleware: {}", id);

    let middleware_instance_example = crate::get_middleware_instance!(id);

    match stop_canister(CanisterIdRecord {
        canister_id: middleware_instance_example.canister_id,
    })
    .await
    {
        Ok(()) => {
            log!("Middleware instance stopped: {}", id);
        }
        Err((_, error)) => {
            return Err(MiddlewareError::FailedToStop(error.to_string()).into());
        }
    }

    match delete_canister(CanisterIdRecord {
        canister_id: middleware_instance_example.canister_id,
    })
    .await
    {
        Ok(()) => {
            log!("Middleware instance removed: {}", id);
        }
        Err((_, error)) => {
            return Err(MiddlewareError::FailedToDelete(error.to_string()).into());
        }
    };

    STATE.with(|s| {
        s.borrow_mut().middlewares.remove(&id);
    });

    Ok(())
}

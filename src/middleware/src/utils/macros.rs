#[macro_export]
macro_rules! get_middleware_instance {
    ($id:expr) => {{
        let middleware_instance_result = crate::STATE.with(|s| {
            s.borrow().middlewares.get(&$id).ok_or(
                $crate::types::errors::MiddlewareError::MiddlewareNotFound($id),
            )
        });

        middleware_instance_result?.0
    }};
}

#[macro_export]
macro_rules! update_middleware_instance {
    ($chain_id:expr, $middleware_instance_example:expr) => {{
        use middleware_utils::nat::ToNativeTypes;
        let result = crate::STATE.with(|s| {
            s.borrow_mut()
                .chains
                .insert($chain_id.to_u32(), Cbor($middleware_instance_example))
                .ok_or($crate::types::errors::middlewareError::ChainNotFound(
                    $chain_id,
                ))
        });

        result?
    }};
}

#[macro_export]
macro_rules! call_middleware_instance {
    ($func:ident($chain_id:ident$(,)? $($args:ident),*) => $return:path) => {
        async {
            let middleware_instance_example = crate::get_middleware_instance!($chain_id);

            let (result,): ($return,) =
                crate::retry_until_success!(ic_cdk::call(
                    middleware_instance_example.canister_id,
                    stringify!($func),
                    ($($args.clone(),)*)
                ))
                .map_err(|(_, msg)| $crate::types::errors::MiddlewareError::CommunicationWithMiddlewareInstanceFailed(msg))?;

            std::result::Result::<_, $crate::types::errors::MiddlewareError>::Ok(result.map_err(|err| $crate::types::errors::MiddlewareError::MiddlewareInstanceError(err.to_string()))?)
        }
    };
}

#[macro_export]
macro_rules! call_middleware_instance_without_error {
    ($func:ident($chain_id:ident$(,)? $($args:ident),*) => $return:path) => {
        async {
            let middleware_instance_example = crate::get_middleware_instance!($chain_id);

            let (result,): ($return,) =
                retry_until_success!(ic_cdk::call(
                    middleware_instance_example.canister_id,
                    stringify!($func),
                    ($($args.clone(),)*)
                ))
                .map_err(|(_, msg)| $crate::types::errors::MiddlewareError::CommunicationWithmiddlewareInstanceFailed(msg))?;

            std::result::Result::<_, $crate::types::errors::MiddlewareError>::Ok(result)
        }
    };
}

#[macro_export]
macro_rules! get_metadata {
    ($field:ident) => {{
        crate::STATE.with(|state| state.borrow().metadata.get().0.$field.clone())
    }};
}

#[macro_export]
macro_rules! update_metadata {
    ($field:ident, $value:expr) => {{
        crate::STATE.with(|state| {
            let mut metadata = state.borrow_mut().metadata.get().clone();
            metadata.$field = $value;
            state.borrow_mut().metadata.set(metadata).unwrap();
        });
    }};
}

#[macro_export]
macro_rules! get_state {
    ($field:ident) => {{
        crate::STATE.with(|state| state.borrow().$field.clone())
    }};
}

#[macro_export]
macro_rules! update_state {
    ($field:ident, $value:expr) => {{
        crate::STATE.with(|state| {
            state.borrow_mut().$field = $value;
        })
    }};
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        // use crate::metrics;
        ic_cdk::println!($($arg)*);
        ic_utils::logger::log_message(format!($($arg)*));
        ic_utils::monitor::collect_metrics();

        // metrics!(set CYCLES, ic_cdk::api::canister_balance() as u128);
    }};
}

#[macro_export]
macro_rules! retry_until_success {
    ($func:expr) => {{
        const MAX_RETRIES: u32 = 5;
        const DURATION_BETWEEN_ATTEMPTS: std::time::Duration = std::time::Duration::from_millis(1000);

        let mut attempts = 0u32;
        let mut result = $func.await;

        let (func_name, func_other) = stringify!($func).rsplit_once("(").unwrap();

        while result.is_err()
            && (format!("{:?}", result.as_ref().unwrap_err()).contains("Canister http responses were different across replicas")
            || format!("{:?}", result.as_ref().unwrap_err()).contains("Timeout expired")
            || format!("{:?}", result.as_ref().unwrap_err()).contains("SysTransient")
            || format!("{:?}", result.as_ref().unwrap_err()).contains("pending") // or Exchange rate canister error: pending
            || format!("{:?}", result.as_ref().unwrap_err()).contains("No response")
            || format!("{:?}", result.as_ref().unwrap_err()).contains("already known"))
            && attempts < MAX_RETRIES
        {
            $crate::utils::time::sleep(DURATION_BETWEEN_ATTEMPTS).await;
            result = $func.await;
            ic_utils::logger::log_message(format!("[{func_name} : {func_other}] attempt: {attempts}"));
            attempts += 1;
        }



        result
    }};

}

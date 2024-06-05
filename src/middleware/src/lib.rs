use crate::types::errors::MiddlewareError;
use crate::utils::memory::Cbor;
use types::{Metadata, STATE};
use utils::set_custom_panic_hook;

mod memory;
mod methods;
mod migrations;
mod types;
mod utils;

#[ic_cdk::init]
fn init() {
    set_custom_panic_hook();

    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.metadata.set(Cbor(Metadata {})).unwrap();
    });
}

// For candid file auto-generation
pub type MiddlewareResult<T> = std::result::Result<T, MiddlewareError>;
use crate::types::custom_return_types::*;
use crate::types::middleware_instance::MiddlewareInstance;
use crate::types::UpdateMetadata;
use crate::utils::pagination::*;
use candid::Principal;

// Candid file auto-generation
candid::export_service!();

/// Not a test, but a helper function to save the candid file
#[cfg(test)]
mod save_candid {

    use super::*;

    fn export_candid() -> String {
        __export_service()
    }

    #[test]
    fn update_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("middleware");
        println!("{}", dir.to_str().unwrap());
        write(dir.join("middleware.did"), export_candid()).expect("Write failed.");
    }
}

use crate::{log, types::STATE};

pub mod address;
pub mod canister;
pub mod macros;
pub mod memory;
pub mod nat;
pub mod pagination;
pub mod time;

pub fn set_custom_panic_hook() {
    _ = std::panic::take_hook(); // clear custom panic hook and set default
    let old_handler = std::panic::take_hook(); // take default panic hook

    // set custom panic hook
    std::panic::set_hook(Box::new(move |info| {
        log!("PANIC OCCURRED: {:#?}", info);
        old_handler(info);
    }));
}

pub fn new_id() -> u64 {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.id_counter += 1;
        state.id_counter.clone()
    })
}

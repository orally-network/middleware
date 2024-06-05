use std::cell::RefCell;

use candid::CandidType;
use ic_stable_structures::{StableBTreeMap, StableCell};
use serde::{Deserialize, Serialize};

use crate::{memory::VMemory, utils::memory::Cbor};

use self::middleware_instance::MiddlewareInstance;

pub mod custom_return_types;
pub mod errors;
pub mod middleware_instance;

#[derive(Serialize, Deserialize, Debug, Default, CandidType, Clone)]
pub struct Metadata {}

#[derive(Serialize, Deserialize, CandidType, Clone)]
pub struct UpdateMetadata {}

impl Metadata {
    pub fn update(&mut self, _update: UpdateMetadata) {}
}

#[derive(Serialize, Deserialize)]
pub struct State {
    #[serde(skip, default = "init_metadata")]
    pub metadata: StableCell<Cbor<Metadata>, VMemory>,

    #[serde(skip, default = "init_middlewares")]
    pub middlewares: StableBTreeMap<u64, Cbor<MiddlewareInstance>, VMemory>,

    pub id_counter: u64,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

fn init_middlewares() -> StableBTreeMap<u64, Cbor<MiddlewareInstance>, VMemory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

fn init_metadata() -> StableCell<Cbor<Metadata>, VMemory> {
    let metadata = Cbor(Metadata::default());
    StableCell::init(crate::memory::get_metadata_memory(), metadata).unwrap()
}

impl Default for State {
    fn default() -> Self {
        Self {
            metadata: init_metadata(),
            middlewares: init_middlewares(),
            id_counter: 0,
        }
    }
}

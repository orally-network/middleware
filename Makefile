all: local_deploy_middleware

update_candid:
	cargo test update_candid

build_middleware_instance:
	dfx build --check middleware_instance_example
	gzip -f -1 ./.dfx/local/canisters/middleware_instance_example/middleware_instance_example.wasm

	mv ./.dfx/local/canisters/middleware_instance_example/middleware_instance_example.wasm.gz assets/middleware_instance_example.wasm.gz





local_upgrade: local_upgrade_middleware 

local_upgrade_middleware: update_candid 
	dfx build middleware 
	gzip -f -1 ./.dfx/local/canisters/middleware/middleware.wasm
	dfx canister install --mode upgrade --wasm ./.dfx/local/canisters/middleware/middleware.wasm.gz middleware

local_deploy_middleware: update_candid build_middleware_instance   
	dfx canister create middleware && dfx build middleware && gzip -f -1 ./.dfx/local/canisters/middleware/middleware.wasm
	dfx canister install --wasm ./.dfx/local/canisters/middleware/middleware.wasm.gz --argument \
		"()" middleware

ifndef PYTHIA_CANISTER
	dfx canister update-settings middleware --add-controller ${PYTHIA_CANISTER}
endif

local_add_middleware_instance_example: update_candid  build_middleware_instance
	./deploy_middleware_instance.sh --wasm assets/middleware_instance_example.wasm.gz --canister middleware --user 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf


ic_upgrade: ic_upgrade_middleware

ic_upgrade_middleware: update_candid
	dfx build middleware --network ic && gzip -f -1 ./.dfx/ic/canisters/middleware/middleware.wasm
	dfx canister install --mode upgrade --wasm ./.dfx/ic/canisters/middleware/middleware.wasm.gz --network ic middleware


ic_deploy_middleware: build_middleware_instance update_candid 
	dfx canister create middleware --ic && dfx build middleware --ic && gzip -f -1 ./.dfx/local/canisters/middleware/middleware.wasm
	dfx canister install --wasm ./.dfx/local/canisters/middleware/middleware.wasm.gz --argument \
		"()" middleware --ic

ifndef PYTHIA_CANISTER
	dfx canister update-settings middleware --add-controller ${PYTHIA_CANISTER} --ic
endif



ic_add_middleware_instance_example: update_candid  build_middleware_instance
	./deploy_middleware_instance.sh --wasm assets/middleware_instance_example.wasm.gz --canister middleware --user 0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf --network ic
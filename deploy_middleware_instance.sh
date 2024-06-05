#!/usr/bin/env bash

set -euo pipefail

declare wasm=
declare canister=
declare network=
declare user=

#########
# USAGE #
#########

function title() {
    echo "Deploy the archive to an II canister"
}

function usage() {
    cat << EOF

Usage:
  $0 --wasm PATH --canister CANISTER --user USER [--network URL]

Options:
  --wasm PATH                     Path to the archive wasm file to deploy.
  --canister CANISTER             Canister id of the II canister to deploy the archive to.
  --user USER                     User parameter for whom to deploy canister. 
  --network URL                   Optional network parameter. Defaults to local replica.
EOF
}

function help() {
    cat << EOF

Deploys the archive to the specified II canister.

NOTE: This requires dfx, hexdump, sed, a running II canister with the appropriate configuration to accept the supplied archive wasm.
EOF
}

# ARGUMENT PARSING


while [[ $# -gt 0 ]]
do
    case $1 in
        --help)
            title
            usage
            help
            exit 0
            ;;
        --wasm)
            wasm="${2:?missing value for '--wasm'}"
            shift; # shift past --wasm and value
            shift;
            ;;
        --canister)
            canister="${2:?missing value for '--canister'}"
            shift; # shift past --canister and value
            shift;
            ;;
        --user)
            user="${2:?missing value for '--user'}"
            shift; # shift past --user and value
            shift;
            ;;
        --network)
            network="${2:?missing value for '--network'}"
            shift; # shift past --network and value
            shift;
            ;;
        *)
            echo "ERROR: unknown argument $1"
            usage
            echo
            echo "Use 'release --help' for more information."
            exit 1
            ;;
    esac
done

if [ -z "$wasm" ]
then
    echo no wasm
    usage
    exit 1
fi

if [ -z "$canister" ]
then
    echo no canister 
    usage
    exit 1
fi

if [ -z "$user" ]
then
    echo no user
    usage
    exit 1
fi

declare -a network_arg=( )

if [ "$network" ]
then
    network_arg+=( "--network" "$network")
fi

dfx canister "${network_arg[@]}" call "$canister" add_middleware_instance --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')\", \"${user}\")")
use candid::candid_method;
use ic_cdk::query;
use ic_web3_rs::ethabi;

#[candid_method]
#[query]
async fn call(args: Vec<u8>) -> Result<Vec<u8>, String> {
    // Args that assumed to be passed to the user's smart contract
    // tokens = [workflow_id, symbol, rate, decimals, timestamp]
    let mut tokens: Vec<ethabi::Token> = serde_cbor::de::from_slice(&args)
        .map_err(|e| format!("failed to deserialize the arguments: {e:?}"))?;

    let rate = tokens[2].clone();

    // Multiply the rate by 2
    if let ethabi::Token::Uint(rate) = rate {
        let rate = rate.as_u64();
        let rate = rate * 2;
        let rate = ethabi::Token::Uint(ethabi::Uint::from(rate));
        tokens[2] = rate;
    } else {
        return Err("rate is not a Uint".to_string());
    }

    Ok(serde_cbor::ser::to_vec(&tokens)
        .map_err(|e| format!("failed to serialize the arguments: {e:?}"))?)
}

candid::export_service!();

/// Not a test, but a helper function to save the candid file.
/// Feel free to remove this module along with the candid::export_service!() macro above
/// if you don't need it.
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
            .join("middleware_instance_example");
        println!("{}", dir.to_str().unwrap());
        write(dir.join("middleware_instance_example.did"), export_candid()).expect("Write failed.");
    }
}

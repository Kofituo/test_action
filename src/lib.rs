use subxt::{OnlineClient, PolkadotConfig};
use subxt::backend::rpc::{RawRpcFuture, RawRpcSubscription, RawValue};
use subxt::ext::futures::{FutureExt, TryStreamExt};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};


wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
async fn testing() {
    return println!("works");
    let api: OnlineClient<PolkadotConfig> = OnlineClient::from_url("ws://127.0.0.1:9944").await.map_err(|_| "").unwrap();
}

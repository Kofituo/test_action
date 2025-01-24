use futures_util::{StreamExt, TryFutureExt, TryStreamExt};
use subxt::backend::rpc::{RawRpcFuture, RawRpcSubscription, RawValue};
use subxt::{OnlineClient, PolkadotConfig};
use wasm_bindgen_test::{console_log, wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);
#[derive(Clone)]
pub struct CustomRpc {
    rpc: subxt_lightclient::LightClientRpc,
}

impl subxt::backend::rpc::RpcClientT for CustomRpc {
    // copied from this repo
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<RawValue>>,
    ) -> RawRpcFuture<'a, Box<RawValue>> {
        Box::pin(
            self.rpc
                .request(method.to_string(), params)
                .map_err(|e| subxt::error::RpcError::ClientError(Box::new(e))),
        )
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<RawValue>>,
        unsub: &'a str,
    ) -> RawRpcFuture<'a, RawRpcSubscription> {
        Box::pin(async {
            let sub = self
                .rpc
                .subscribe(sub.to_string(), params, unsub.to_string())
                .await
                .map_err(|e| subxt::error::RpcError::ClientError(Box::new(e)))?;

            Ok(RawRpcSubscription {
                id: Some(sub.id().to_string()),
                stream: sub.map_err(|e| subxt::error::RpcError::ClientError(Box::new(e))).boxed(),
            })
        })
    }
}

async fn from_chain(
    chain_spec: &str,
    bootnodes: Vec<String>, ) -> OnlineClient::<PolkadotConfig> {
    let config = subxt_lightclient::ChainConfig::chain_spec(chain_spec)
        .set_bootnodes(bootnodes.clone().into_iter()).unwrap();
    let (_client, rpc) = subxt_lightclient::LightClient::relay_chain(config).unwrap();
    let custom_rpc = CustomRpc { rpc };
    OnlineClient::<PolkadotConfig>::from_rpc_client(custom_rpc).await.unwrap()
}

fn address() -> Vec<String> {
    // sample addresses
    vec![
        "/ip4/127.0.0.1/tcp/30344/ws/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"
            .into(),
    ]
}

#[wasm_bindgen_test]
async fn testing() {
    let api: OnlineClient<PolkadotConfig> = from_chain(include_str!("../custom_spec.json"), address()).await;
    panic!("does not work {:?}",api.genesis_hash());
}

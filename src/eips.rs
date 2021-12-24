use futures::future::LocalBoxFuture;
use serde_json::json;
use wasm_bindgen::prelude::*;
use web3::error;
use web3::helpers::CallFuture;
use web3::transports::eip_1193::Eip1193;
use web3::transports::eip_1193::Provider;
use web3::types::Address;
use web3::Transport;

// eip 2255
pub fn wallet_request_permissions(
    transport: &Eip1193,
) -> CallFuture<Vec<Address>, LocalBoxFuture<'static, error::Result<serde_json::value::Value>>> {
    let params = json!({
        "eth_accounts": {}
    });
    CallFuture::new(transport.execute("wallet_requestPermissions", vec![params]))
}

//eip 1193
#[wasm_bindgen(
    inline_js = "export function get_provider_js() { let provider = window.ethereum; if (!provider) {throw 'provider not found'}; return provider;}"
)]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn get_provider_js() -> Result<Option<Provider>, JsValue>;
}

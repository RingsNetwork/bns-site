use std::rc::Rc;
use std::cmp::PartialEq;
use wasm_bindgen::prelude::*;
use web3::api::Web3;
use web3::transports::eip_1193::Eip1193;
use web3::transports::eip_1193::Provider;

use yew::Properties;

//eip 1193
#[wasm_bindgen(
    inline_js = "export function get_provider_js() { let provider = window.ethereum; if (!provider) {throw 'provider not found'}; return provider;}"
)]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn get_provider_js() -> Result<Option<Provider>, JsValue>;
}

pub struct Web3Provider {
    pub web3: Option<Rc<Web3<Eip1193>>>
}

impl PartialEq for Web3Provider {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
    fn ne(&self, other: &Self) -> bool {
        self != other
    }
}


impl Web3Provider {
    pub fn create() -> Self {
        match get_provider_js() {
            Ok(Some(p)) => Self {
                web3: Some(Rc::new(Web3::new(Eip1193::new(p))))
            },
            _ => Self {
                web3: None
            }
        }
    }
}

impl Properties for Web3Provider {
    type Builder = Self;

    fn builder() -> Self::Builder {
        Self::create()
    }
}

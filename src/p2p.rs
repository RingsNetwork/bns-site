use log::info;
use bns_core::types::ice_transport::IceTransport;
use bns_core::types::ice_transport::IceTransportCallback;
use bns_core::encoder::{encode, decode};
use bns_core::types::channel::Channel;
use bns_core::channels::wasm::CbChannel;
use bns_core::transports::wasm::WasmTransport;
use bns_core::transports::wasm::SdpOfferStr;
use js_sys::Reflect;
use serde_json::json;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::RtcConfiguration;
use web_sys::RtcDataChannel;
use web_sys::InputEvent;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::html::Scope;
use yew::NodeRef;
use std::any::Any;
use std::sync::{Arc, Mutex};


// ref: https://rustwasm.github.io/wasm-bindgen/examples/webrtc_datachannel.html
// ref: https://mac-blog.org.ua/webrtc-one-to-one-without-signaling-server

#[derive(Properties, PartialEq, Clone)]
pub struct Web3Props {
    pub address: Option<Rc<String>>,
}

pub enum P2pMsg {
    ConnectChannel,
    UpdateP2p,
    ConnectPeer(String),
    None
}

pub struct P2p {
    pub address: Option<Rc<String>>,
    pub transport: Arc<Mutex<WasmTransport>>,
    pub textarea_ref: NodeRef
}

// impl IceTransport {
//     pub fn new() -> Self {
//         let mut config = RtcConfiguration::new();
//         config.ice_servers(
//             &JsValue::from_serde(&json! {[{"urls":"stun:stun.l.google.com:19302"}]}).unwrap(),
//         );

//         return Self {
//             offer: None,
//             peer: RtcPeerConnection::new_with_configuration(&config).ok(),
//             channel: None
//         };
//         // let onopen_callback = Closure::wrap(Self::on_open());

//         // transport.peer.set_ondatachannel(Some(onopen_callback.as_ref().unchecked_ref()));
//         //            link.send_message(P2pMsg::UpdateP2p((channel, sdp, Some(peer))));
//     }

//     pub async fn setup(&mut self) {
//         self.setup_channel("bns").await;
//         self.setup_offer().await;
//     }

//     pub async fn setup_channel(&mut self, name: &str) -> &Self {
//         if let Some(peer) = &self.peer {
//             let channel = peer.create_data_channel(&name);
//             let onmessage_callback = Closure::wrap(Self::on_message(channel.clone()));
//             channel.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
//             self.channel = Some(channel);
//         }
//         return self;
//     }

//     pub async fn setup_offer(&mut self) -> &Self {
//         if let Some(peer) = &self.peer {
//             if let Ok(offer) = JsFuture::from(peer.create_offer()).await {
//                 self.offer = Reflect::get(&offer, &JsValue::from_str("sdp")).ok()
//                     .and_then(|o| o.as_string())
//                     .take();
//             }
//         }
//         return self;
//     }

//     pub async fn dial(&self, offer: String) {
//         let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
//         offer_obj.sdp(&offer);
//         if let Some(peer) = &self.peer {
//             let srd_promise = peer.set_remote_description(&offer_obj);
//             match JsFuture::from(srd_promise).await {
//                 _ => {
//                 }
//             }
//         }
//     }

//     pub fn on_message(channel: RtcDataChannel) -> Box<dyn FnMut(MessageEvent)> {
//         box move |ev: MessageEvent| match ev.data().as_string() {
//             Some(message) => {
//                 info!("{:?}", message);
//                 channel.send_with_str("Pong from pc1.dc!").unwrap();
//             }
//             None => {}
//         }
//     }

//     pub fn on_channel() -> Box<dyn FnMut(RtcDataChannelEvent)> {
//         box move |ev: RtcDataChannelEvent| {
//             let cnn = ev.channel();
//             info!("onDataChannelEvent!");
//             match cnn.send_with_str("Greeting!") {
//                 Ok(_d) => {}
//                 Err(_e) => {
//                     panic!();
//                 }
//             };
//         }
//     }

//     pub fn on_open() -> Box<dyn FnMut(RtcDataChannelEvent)> {
//         box move |ev: RtcDataChannelEvent| {
//             info!("channel Open!");
//             let cnn = ev.channel();
//             match cnn.send_with_str("Greeting!") {
//                 Ok(_d) => {}
//                 Err(_e) => {
//                     panic!();
//                 }
//             };
//         }
//     }

//     pub fn on_icecandidate() -> Box<dyn FnMut(RtcPeerConnectionIceEvent)> {
//         box move |ev: RtcPeerConnectionIceEvent| {
//             info!("ice candidate");
//             match ev.candidate() {
//                 Some(candidate) => {
//                     info!("onicecandiate: {:#?}", candidate.candidate());
//                 }
//                 None => {}
//             }
//         }
//     }
// }

impl Component for P2p {
    type Message = P2pMsg;
    type Properties = Web3Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            address: ctx.props().address.clone(),
            transport: Arc::new(Mutex::new(WasmTransport::new(CbChannel::new(1)))),
            textarea_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            P2pMsg::ConnectChannel => {
                match &self.address {
                    Some(_addr) => {
                        let link = ctx.link().clone();
                        let trans = Arc::clone(&self.transport);
                        spawn_local(
                            async move {
                                // should not unwrap here
                                info!("try start trans");
                                trans.lock().unwrap().start().await.unwrap();
                                link.send_message(P2pMsg::UpdateP2p);
                            }
                        )
                    }
                    None => {
                         info!("cant get addr");
                    }
                }
                return true;
            },
            P2pMsg::ConnectPeer(offer) => {
                info!("Connection to peer {}", offer.clone());
                let trans = self.transport.clone();
                let offer = decode(offer).unwrap();
                spawn_local(
                    async move {
                        // should not unwrap here
                        info!("set remote desc");
                        trans.lock().unwrap().set_remote_description(SdpOfferStr::new(offer)).await.unwrap();
                    }
                );
                return true
            },
            P2pMsg::UpdateP2p => {
                return true;
            },
            P2pMsg::None => {
                return false;
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.address = ctx.props().address.clone();
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={"p2p"}>
                <a onclick={ctx.link().callback(|_| P2pMsg::ConnectChannel)}>{"GET SDP"}</a>
                <pre class="text">
            { match &self.transport.lock().unwrap().offer {
                Some(o) => {
                    let sdp = encode(o.sdp());
                    info!("{:?}", &sdp);
                    sdp
                },
                _ => "".to_string()
            }}
            </pre>
            <div>
                <textarea ref={self.textarea_ref.clone()}></textarea>
                <a onclick={
                    let text = self.textarea_ref.clone();
                    ctx.link().callback(move |_| {
                    match text.cast::<HtmlTextAreaElement>() {
                        Some(t) => P2pMsg::ConnectPeer(t.value()),
                        None => P2pMsg::None
                    }
                })}>{"Connect"}</a>
            </div>
            </div>
        }
    }
}

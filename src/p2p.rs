use crate::console_log;
use crate::log;
use js_sys::Reflect;
use serde_json::json;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::RtcConfiguration;
use web_sys::RtcDataChannel;
use web_sys::{MessageEvent, RtcDataChannelEvent, RtcPeerConnection, RtcPeerConnectionIceEvent};
use web_sys::{RtcSdpType, RtcSessionDescriptionInit};
use yew::prelude::*;
use yew::html::Scope;


// ref: https://rustwasm.github.io/wasm-bindgen/examples/webrtc_datachannel.html
// ref: https://mac-blog.org.ua/webrtc-one-to-one-without-signaling-server

#[derive(Properties, PartialEq, Clone)]
pub struct Web3Props {
    pub address: Option<Rc<String>>,
}

pub enum P2pMsg {
    ConnectChannel,
    UpdatePeer(RtcPeerConnection),
    UpdateOffer(String),
    ConnectPeer
}

pub struct P2p {
    pub address: Option<Rc<String>>,
    pub offer: Option<String>,
    pub peer: Option<RtcPeerConnection>
}

impl P2p {
    pub async fn start(addr: Rc<String>, link: Scope<Self>) -> RtcPeerConnection {
        let mut config = RtcConfiguration::new();
        config.ice_servers(
            &JsValue::from_serde(&json! {[{"urls":"stun:stun.l.google.com:19302"}]}).unwrap(),
        );
        let peer = RtcPeerConnection::new_with_configuration(&config).unwrap();
        console_log!("{:?} created: state {:?}", addr, peer.signaling_state());
        let channel = peer.create_data_channel("bns-channel");

        let onmessage_callback = Closure::wrap(Self::on_message(channel.clone()));
        channel.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));

        let onopen_callback = Closure::wrap(Self::on_open());
        peer.set_ondatachannel(Some(onopen_callback.as_ref().unchecked_ref()));

        if let Some(offer) = JsFuture::from(peer.create_offer()).await.ok() {
            let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp")).unwrap()
                .as_string()
                .unwrap();
            link.send_message(P2pMsg::UpdateOffer(offer_sdp.clone()));
        } else {
             console_log!("cannot get offer");

        }
        return peer;
    }

    pub async fn dial(self, offer: String) {
        if let Some(peer) = self.peer {
            let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
            offer_obj.sdp(&offer);
            let srd_promise = peer.set_remote_description(&offer_obj);
            match JsFuture::from(srd_promise).await {
                _ => {
                }
            }
        }
    }

    pub fn on_message(channel: RtcDataChannel) -> Box<dyn FnMut(MessageEvent)> {
        box move |ev: MessageEvent| match ev.data().as_string() {
            Some(message) => {
                console_log!("{:?}", message);
                channel.send_with_str("Pong from pc1.dc!").unwrap();
            }
            None => {}
        }
    }

    pub fn on_channel() -> Box<dyn FnMut(RtcDataChannelEvent)> {
        box move |ev: RtcDataChannelEvent| {
            let cnn = ev.channel();
            console_log!("onDataChannelEvent!");
            match cnn.send_with_str("Greeting!") {
                Ok(_d) => {}
                Err(_e) => {
                    panic!();
                }
            };
        }
    }

    pub fn on_open() -> Box<dyn FnMut(RtcDataChannelEvent)> {
        box move |ev: RtcDataChannelEvent| {
            console_log!("channel Open!");
            let cnn = ev.channel();
            match cnn.send_with_str("Greeting!") {
                Ok(_d) => {}
                Err(_e) => {
                    panic!();
                }
            };
        }
    }

    pub fn on_icecandidate() -> Box<dyn FnMut(RtcPeerConnectionIceEvent)> {
        box move |ev: RtcPeerConnectionIceEvent| {
            console_log!("ice candidate");
            match ev.candidate() {
                Some(candidate) => {
                    console_log!("onicecandiate: {:#?}", candidate.candidate());
                }
                None => {}
            }
        }
    }
}

impl Component for P2p {
    type Message = P2pMsg;
    type Properties = Web3Props;

    fn create(ctx: &Context<Self>) -> Self {
        return Self {
            address: ctx.props().address.clone(),
            peer: None,
            offer:None

        };
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            P2pMsg::ConnectChannel => {
                match &self.address {
                    Some(addr) => {
                        console_log!("try start p2p");
                        let addr = addr.clone();
                        let link = ctx.link().clone();
                        spawn_local(
                            async move {
                                let peer = Self::start(addr.clone(), link.clone()).await;
                                link.send_message(Self::Message::UpdatePeer(peer));
                            }
                        )
                    }
                    None => {
                         console_log!("cant get addr");
                    }
                }
                return true;
            },
            P2pMsg::UpdatePeer(peer) => {
                self.peer = Some(peer);
                return true;
            },
            P2pMsg::UpdateOffer(offer) => {
                self.offer = Some(offer);
                return true;
            },
            P2pMsg::ConnectPeer => {
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
                <a onclick={ctx.link().callback(|_| P2pMsg::ConnectChannel)}>{"Connect Channel"}</a>
                <div class="text">
            { match &self.offer {
                Some(o) => o,
                _ => ""
            }}
            </div>
            <a onclick={ctx.link().callback(|_| P2pMsg::ConnectPeer)}>{"Connect Peer"}</a>

           </div>
        }
    }
}

use crate::console_log;
use crate::log;


use serde_json::json;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;


use web_sys::RtcConfiguration;
use web_sys::RtcDataChannel;
use web_sys::{
    MessageEvent, RtcDataChannelEvent, RtcPeerConnection, RtcPeerConnectionIceEvent,
};
use yew::prelude::*;

// ref: https://rustwasm.github.io/wasm-bindgen/examples/webrtc_datachannel.html
// ref: https://mac-blog.org.ua/webrtc-one-to-one-without-signaling-server

#[derive(Properties, PartialEq, Clone)]
pub struct Web3Props {
    pub address: Option<Rc<String>>,
}

pub enum P2pMsg {
    ConnectChannel,
}

pub struct P2p {
    pub link: ComponentLink<Self>,
    pub address: Option<Rc<String>>,
}

impl P2p {
    pub fn start(addr: Rc<String>) -> () {
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        return Self {
            link: link,
            address: props.address,
        };
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            P2pMsg::ConnectChannel => {
                match &self.address {
                    Some(addr) => {
                        console_log!("try start p2p");
                        Self::start(addr.clone());
                    }
                    None => {}
                }
                return true;
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.address = props.address;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id={"p2p"}><a onclick={self.link.callback(|_| P2pMsg::ConnectChannel)}>{"Connect Channel"}</a></div>
        }
    }
}

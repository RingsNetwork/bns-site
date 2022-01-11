use crate::eips;
use crate::faq::FaqView;
use crate::footer::Footer;
use crate::nav::Nav;
use crate::search::Search;
use crate::slides::Slides;
use crate::p2p::P2p;
use crate::{console_log, log};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web3::api::Web3;
use web3::transports::eip_1193::Eip1193;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub address: Option<Rc<String>>,
    pub page: Pages,
}

pub enum Msg {
    ConnectWallet,
    UpdateAddr(String),
    SwitchPage(Pages),
}

#[derive(PartialEq, Clone, Copy)]
pub enum Pages {
    Index,
    FAQ,
}

pub struct View {
    pub web3: Option<Web3<Eip1193>>,
    pub address: Option<Rc<String>>,
    pub page: Pages,
}

impl Component for View {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        match eips::get_provider_js() {
            Ok(Some(p)) => Self {
                web3: Some(Web3::new(Eip1193::new(p))),
                address: None,
                page: Pages::Index,
            },
            Ok(None) | Err(_) => Self {
                web3: None,
                address: None,
                page: Pages::Index,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ConnectWallet => {
                let link = ctx.link().clone();
                match &self.web3 {
                    Some(w) => {
                        let w3 = w.clone();
                        spawn_local(async move {
                            match w3.eth().request_accounts().await {
                                Ok(s) => {
                                    link.send_message(Msg::UpdateAddr(s[0].to_string()));
                                    console_log!("Updating address 1");
                                }
                                Err(_e) => {
                                    console_log!("Failed to fetch address");
                                }
                            };
                        });
                    }
                    None => {
                        console_log!("Failed to get eth provider");
                    }
                }
                true
            }
            Msg::UpdateAddr(addr) => {
                self.address = Some(Rc::new(addr));
                console_log!("Updating address 2");
                true
            }
            Msg::SwitchPage(page) => {
                self.page = page;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <body>
                <Nav address = {self.address.clone()} page={self.page} />
                <P2p address = {self.address.clone()} />
            <Footer />
            </body>
        }
    }
}

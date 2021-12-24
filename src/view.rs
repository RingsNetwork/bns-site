use crate::eips;
use crate::faq::FaqView;
use crate::footer::Footer;
use crate::nav::Nav;
use crate::search::Search;
use crate::slides::Slides;
use std::default::Default;
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
    pub link: ComponentLink<Self>,
    pub web3: Option<Web3<Eip1193>>,
    pub address: Option<Rc<String>>,
    pub page: Pages,
}

impl Component for View {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        match eips::get_provider_js() {
            Ok(Some(p)) => Self {
                link: link,
                web3: Some(Web3::new(Eip1193::new(p))),
                address: None,
                page: Pages::Index,
            },
            Ok(None) | Err(_) => Self {
                link: link,
                web3: None,
                address: None,
                page: Pages::Index,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ConnectWallet => {
                let link = self.link.clone();
                match &self.web3 {
                    Some(w) => {
                        let w3 = w.clone();
                        spawn_local(async move {
                            match w3.eth().request_accounts().await {
                                Ok(s) => {
                                    link.send_message(Msg::UpdateAddr(s[0].to_string()));
                                }
                                Err(_e) => (),
                            };
                        });
                    }
                    None => (),
                }
                true
            }
            Msg::UpdateAddr(addr) => {
                self.address = Some(Rc::new(addr));
                true
            }
            Msg::SwitchPage(page) => {
                self.page = page;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <body>
                <Nav address = {self.address.clone()} page={self.page} />
            {
                match self.page {
                    Pages::Index => html! {
                        <>
                            <Search />
                            <Slides />
                        </>
                    },
                    Pages::FAQ => html! {
                        <>
                            <FaqView />
                        </>
                    },
                }
            }
            <Footer />
            </body>
        }
    }
}

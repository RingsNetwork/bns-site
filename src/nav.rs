use crate::view::{Msg, NavProps, Pages, View};

use yew::prelude::{html, Component, ComponentLink, ShouldRender};
use yew::Html;


pub enum NavMsg {
    ConnectWallet,
    SearchDomain,
    Register,
    UpdateAddr(String),
}

pub struct Nav {
    pub link: ComponentLink<Self>,
    pub props: NavProps,
}

impl Component for Nav {
    type Message = NavMsg;
    type Properties = NavProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: props,
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props == props {
            false
        } else {
            self.props = props;
            true
        }
    }

    fn view(&self) -> Html {
        let maybe_addr = match &self.props.address.as_ref() {
            Some(addr) => addr,
            None => "Connect",
        };

        html! {
            <nav>
                <a id="logo">
                    <img id="bns_logo" alt="BNS Logo" src="imgs/BNS-logo.png"/>
                </a>
                <ul>
                <li
                class={
                    if self.props.page == Pages::Index {
                        "selected"
                    } else {
                        ""
                    }
                }

                ><a onclick={
                    self.link.get_parent().unwrap().clone().downcast::<View>().callback(|_| {
                        Msg::SwitchPage(Pages::Index)
                    })
                }>{"Home"}</a></li>
                <li><a href="../docs/slide.pdf" type="application/pdf" target="_blank">{"Whitepaper"}</a></li>
                <li
                class={
                    if self.props.page == Pages::FAQ {
                        "selected"
                    } else {
                        ""
                    }
                }
                ><a onclick={
                    self.link.get_parent().unwrap().clone().downcast::<View>().callback(|_| {
                        Msg::SwitchPage(Pages::FAQ)
                    })
                }>{"FAQ"}</a></li>
                </ul>
                <div id="wallet">
                    <a onclick={self.link.get_parent().unwrap().clone().downcast::<View>().callback(|_| Msg::ConnectWallet)}>{maybe_addr}</a>
                </div>
            </nav>
        }
    }
}

use crate::view::{Msg, Pages, View};
use std::rc::Rc;
use yew::prelude::*;
use yew::Html;

#[derive(Properties, PartialEq, Clone)]
pub struct NavProps {
    pub address: Option<Rc<String>>,
    pub page: Pages,
}

pub enum NavMsg {
    ConnectWallet,
    SearchDomain,
    Register,
    UpdateAddr(String),
}

pub struct Nav {
    pub props: NavProps,
}

impl Component for Nav {
    type Message = NavMsg;
    type Properties = NavProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if &self.props != ctx.props() {
            self.props = ctx.props().clone();
            true
        } else {
            false
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let maybe_addr = match &self.props.address.as_ref() {
            Some(addr) => addr,
            None => "Connect",
        };

        html! {
            <nav>
                <a href="https://bns.org" id="logo">
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
                    ctx.link().get_parent().unwrap().clone().downcast::<View>().callback(|_| {
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
                    ctx.link().get_parent().unwrap().clone().downcast::<View>().callback(|_| {
                        Msg::SwitchPage(Pages::FAQ)
                    })
                }>{"FAQ"}</a></li>
                </ul>
                <div id="wallet">
                    <a onclick={ctx.link().get_parent().unwrap().clone().downcast::<View>().callback(|_| Msg::ConnectWallet)}>{maybe_addr}</a>
                </div>
            </nav>
        }
    }
}

use yew::prelude::*;

pub enum MainMsg {}

pub struct Search {}

impl Component for Search {
    type Message = MainMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {};
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="search">
                <div class="search-area">
                    <input type="search" placeholder="yourname.btc"/><a class="submit">{"Search"}</a>
                </div>
            </div>
        }
    }
}

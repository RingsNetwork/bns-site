use yew::prelude::*;

pub enum MainMsg {}

pub struct Search {}

impl Component for Search {
    type Message = MainMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        return Self {};
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="search">
                <div class="search-area">
                    <input type="search" placeholder="yourname.btc"/><a class="submit">{"Search"}</a>
                </div>
            </div>
        }
    }
}

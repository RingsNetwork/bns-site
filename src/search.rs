



use yew::prelude::*;



pub enum MainMsg {}

pub struct Search {}

impl Component for Search {
    type Message = MainMsg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        return Self {};
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="search">
                <div class="search-area">
                    <input type="search" placeholder="yourname.btc"/><a class="submit">{"Search"}</a>
                </div>
            </div>
        }
    }
}

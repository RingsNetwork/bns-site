



use yew::prelude::*;



pub enum MainMsg {}

pub struct Footer {}

impl Component for Footer {
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
            <footer>
                <span class="copyright">
            {"Copyright © 2021 BNS Fundation"}
                </span>

                <div class="links">
                <a class="twitter-icon"></a>
                <a class="github-icon"></a>
                <a class="discord-icon"></a>
                </div>
            </footer>
        }
    }
}

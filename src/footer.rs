use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
            <footer>
                <span class="copyright">
            {"Copyright © 2021 BNS Fundation"}
                </span>

                <div class="links">
                <a class="twitter-icon" href="https://twitter.com/btcnameservice" target="_blank"></a>
                <a class="github-icon" href="https://github.com/BNSnet" target="_blank"></a>
                <a class="discord-icon" href="https://discord.gg/xWrP5ptb" target="_blank"></a>
                </div>
            </footer>
    }
}

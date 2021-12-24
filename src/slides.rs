



use yew::prelude::*;



pub enum MainMsg {}

pub struct Slides {}

impl Component for Slides {
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
            <div id="viewport">
              <section>
        <div class="title">
          <span><p>{"."}</p>{"BTC Name Services"}</span>
        </div>
        <div class="body">
                <div class="desc-wrap">
                    <div class="icon">
                        <i class="awe-icon ng-icon"></i>
                    </div>
                    <div class="description">
                        <span class="desc-title">{"Ext. of Web2"}</span>
                        <span class="desc-body">{"An extension of current domain name system. Classic Web2 Urls, MultiAddr, Wallet Addresses, and your DID via our DHT."}</span>
                    </div>
                </div>

                <div class="desc-wrap">
                    <div class="icon">
                        <i class="awe-icon node-icon"></i>
                    </div>

                    <div class="description">
                        <span class="desc-title">{"Chord DHT"}</span>
                        <span class="desc-body">{"A Chord DHT, based on ICE protocol. BNS support browser based peepr to peer communication."}</span>
                    </div>

                </div>
                <div class="desc-wrap">
                    <div class="icon">
                        <i class="awe-icon key-pair-icon"></i>
                    </div>
                    <div class="description">
                       <span class="desc-title">{"Security"}</span>
                        <span class="desc-body">{"All traffics are encrypted with your onchain key-pair, Full anonymous services hosting is avaliable."}</span>
                    </div>
            </div>
                </div>
                </section>
                <section class="slide">
                <div class="desc">
                <p>{"BNS is a solution package targeting the next-generation Internet, which includes dWeb, anonymous network traffic and private data transaction, etc.
BNS provides a safe and DID-based information exchange spot for customers by bringing in the .btc domain name.
The domain name solution services of BNS cover not only Web3, IPFS  and blockchain wallets, but also traditional internet services."}</p>
                    </div>
             <div class="body">
              <ul>
            <li>
              {"BNS must be an extension of current domain name system."}
            </li>
            <li>
              {"BNS must be fully distributed and decentralized."}
            </li>
            <li>
              {"BNS must be self-organizing and not depend on administrators or centralized infrastructure."}
            </li>
            <li>
              {"BNS must be open and permit new peers to join."}
            </li>
              </ul>
            </div>
                </section>

                <section class="slide">
                    <div class="title">
                      <span>{"dDNS"}</span>
              <span class="sec-title">{"A BNS Record can be resolved to:"}</span>
            </div>
            <div class="body">
              <ul>
            <li>
              {"Classic internet URL"}
            </li>
            <li>
              {"Web3 URI, like IPFS, MultiAddr, ENS, etc."}
            </li>
            <li>
              {"Arbitrary on chain wallet address"}
            </li>
            <li>
              {"PeerId of BNS DHT which may pointed to your hidden services"}
            </li>
              </ul>
            </div>

                </section>
                <section class="slide">
                    <div class="title">
                      <span>{"DHT"}</span>
              <span class="sec-title">{"We use Chord DHT to support:"}</span>
            </div>

            <div class="body">
              <ul>
            <li>
              <span class="icon handshake-icon"></span>
              <span class="ctx">{"Ad-hoc message between browsers"}</span>
            </li>
            <li>
              <span class="icon anony-icon"></span>
              <span class="ctx">{"Anonymous hosting and traffic"}</span>
            </li>
            <li>
              <span class="icon mask-icon"></span>
              <span class="ctx">{"Zero Knowledge data transfer and trading "}</span>
            </li>
              </ul>
            </div>
                </section>
            </div>
        }
    }
}

use yew::{function_component, html};

#[function_component(FaqView)]
pub fn faq() -> Html {
    html! {
            <div id="faq">
                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b> {"What is BNS? Is it a new decentralized domain name system? What’s its difference to ENS, Unstoppable Domains?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"The ENS and Unstoppable Domains are excellent decentralized domain name system in the community. Unlike them, BNS is a solution package targeting the next-generation Internet, which includes dWeb, anonymous network traffic and private data transaction, etc. Granted, a decentralized domain name system is the major cornerstone. BNS provides a safe and DID-based information exchange spot for customers by bringing in the .btc domain name."}
                </p>
                </section>

                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b>
            {"BNS claims that it is an extension to the Internet, how does that work?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"The domain name solution services of BNS cover not only Web3, IPFS  and blockchain wallets, but also traditional internet services. Indeed, BNS can be used as a traditional DNS for resolving .btc,.com,.org as well. In a way, it is an enhanced version of openDNS or Google's 8.8.8.8."}
                </p>
                </section>

                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b>
            {"What is BNS's DHT and is it based on IPFS? What’s its difference from IPFS?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"No, BNS's DHT is not based on IPFS. IPFS is an excellent project, that’s the reason we inherited the underlying protocol of IPFS-MultiAddr. BNS will implement user node discovery based on the ICE protocol, where user nodes will include browser nodes and traditional nodes. We will implement Browser to Browser data exchange and information verification. Furthermore, we will build a distributed hash table (DHT) for users to store user data and redirect anonymous traffic."}
                </p>
                </section>


                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b>
            {"How to realize redirect anonymous traffic, like the Tor Project?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"We referred to the I2P network, and the implementation method and problems faced by the Tor network. The problems Tor Project faced were excessive DDOS attacks and the existence of insecure phishing nodes. The Tor Project community tried using Bitcoin's POW mechanism to solve the problems, however, the solutions were proven immature: it will greatly increase the burden on network nodes. The solution adopted by BNS is more relative to Ethereum 2.0. We will allow users to pledge tokens and set corresponding verification/penalty mechanisms to ensure the security of the network-yes, it is POS.
        "}
                </p>
                </section>


                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b>
            {"How does BNS perform traffic verification? Will it be a new side chain or public chain?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"The purpose of traffic verification is to verify the integrity of the traffic and to ensure that no space for arbitrage through false traffic. We use traffic fragmentation and zero-knowledge proof-based methods for vagrant verification. The protocol will slice one batch of data into a number of units, and pick random amounts of nodes from the DHT network for the transfer. Then the Receiver will use the Pedersen commitments method to verify if the information is the one sent by the Sender or not. If not, all relating nodes will receive corresponding punishment.
        "}
                </p>
                </section>

                <section class="faq-item">
                <p class="q">
                <b>{"Q:"}</b>
            {"How does BNS implement Hidden Services?"}
                </p>
                <p class="a">
                <b>{"A:"}</b>
            {"Frankly speaking, we referred to the implementation of the Tor Project. Hidden Services must host a DHT Peer, and this peer will be added to our DHT. Therefore, anyone who joins the BNS network can access this service through the .btc domain name. You can even use Nginx like other large Internet services, which is more feasible and safer."}
                </p>
                </section>
            </div>
    }
}

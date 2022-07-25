use yew::prelude::*;

pub struct AboutUs;
impl Component for AboutUs {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="container is-parent pt-5 pb-5">
                    <div class="tile is-child">
                        <div class="columns">
                            <div class="column is-three-quarters">
                                <div class="tile is-child box">
                                    <p class="title">{ "About Us" }</p>
                                    <div class="content">
                                        {r#"
                                        Whales are a widely distributed and diverse group of fully aquatic placental marine mammals. They are an informal grouping within the infraorder Cetacea, which usually excludes dolphins and porpoises. Whales, dolphins and porpoises belong to the order Cetartiodactyla, which consists of even-toed ungulates. Their closest non-cetacean living relatives are the hippopotamuses, from which they and other cetaceans diverged about 54 million years ago. The two parvorders of whales, baleen whales (Mysticeti) and toothed whales (Odontoceti), are thought to have had their last common ancestor around 34 million years ago. Whales consist of eight extant families: Balaenopteridae (the rorquals), Balaenidae (right whales), Cetotheriidae (the pygmy right whale), Eschrichtiidae (the grey whale), Monodontidae (belugas and narwhals), Physeteridae (the sperm whale), Kogiidae (the dwarf and pygmy sperm whale), and Ziphiidae (the beaked whales).
                                        "#}
                                    </div>
                                </div>
                            </div>
                            <div class="column">
                                <figure class="image is-400by400">
                                    <img src="../../static/img/fotw_icon.png"/>
                                </figure>
                            </div>
                        </div>
                    </div>
                    <div class="tile is-child">
                        <div class="columns">
                            <div class="column">
                                <figure class="image is-530by360">
                                    <img src="../../static/img/first_icon.png"/>
                                </figure>
                            </div>
                            <div class="column is-three-quarters">
                                <div class="tile is-child box">
                                    <p class="title">{ "FIRST" }</p>
                                    <div class="content">
                                        {r#"
                                        Whales are a widely distributed and diverse group of fully aquatic placental marine mammals. They are an informal grouping within the infraorder Cetacea, which usually excludes dolphins and porpoises. Whales, dolphins and porpoises belong to the order Cetartiodactyla, which consists of even-toed ungulates. Their closest non-cetacean living relatives are the hippopotamuses, from which they and other cetaceans diverged about 54 million years ago. The two parvorders of whales, baleen whales (Mysticeti) and toothed whales (Odontoceti), are thought to have had their last common ancestor around 34 million years ago. Whales consist of eight extant families: Balaenopteridae (the rorquals), Balaenidae (right whales), Cetotheriidae (the pygmy right whale), Eschrichtiidae (the grey whale), Monodontidae (belugas and narwhals), Physeteridae (the sperm whale), Kogiidae (the dwarf and pygmy sperm whale), and Ziphiidae (the beaked whales).
                                        "#}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

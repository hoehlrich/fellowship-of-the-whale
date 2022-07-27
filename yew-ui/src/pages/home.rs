use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img alt="A random image for the input term 'yew'." src="https://source.unsplash.com/random/1200x400/?whale" />
                    </figure>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "What is a whale?" }</p>
                        <p class="subtitle">{ "Everything you need to know!" }</p>

                        <div class="content">
                            {r#"
                            Whales are a widely distributed and diverse group of fully aquatic placental marine mammals. They are an informal grouping within the infraorder Cetacea, which usually excludes dolphins and porpoises. Whales, dolphins and porpoises belong to the order Cetartiodactyla, which consists of even-toed ungulates. Their closest non-cetacean living relatives are the hippopotamuses, from which they and other cetaceans diverged about 54 million years ago. The two parvorders of whales, baleen whales (Mysticeti) and toothed whales (Odontoceti), are thought to have had their last common ancestor around 34 million years ago. Whales consist of eight extant families: Balaenopteridae (the rorquals), Balaenidae (right whales), Cetotheriidae (the pygmy right whale), Eschrichtiidae (the grey whale), Monodontidae (belugas and narwhals), Physeteridae (the sperm whale), Kogiidae (the dwarf and pygmy sperm whale), and Ziphiidae (the beaked whales).
                            "#}
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "What is \"The Fellowship of the Ring\"?" }</p>
                        <p class="subtitle">{ "Everything you really need to know!" }</p>
                        <div class="content">
                            {r#"
                            The Fellowship of the Ring was formed as a brotherhood among members of the various Free Peoples of Middle-earth. Its purpose was to take the One Ring to Mordor so that it might be cast into the fires of Mount Doom, the mountain in which it was forged, so that it would be destroyed and ultimately eradicate the Dark Lord Sauron.
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}

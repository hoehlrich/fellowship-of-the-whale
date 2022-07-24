use yew::prelude::*;

pub struct Teams;
impl Component for Teams {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-child hero">
                <div class="hero-body container pb-auto">
                    <h1 class="title is-1">{ "Teams" }</h1>
                </div>
            </div>
        }
    }
}

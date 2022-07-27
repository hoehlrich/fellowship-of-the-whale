use yew::prelude::*;

pub struct Calendar;
impl Component for Calendar {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-child hero">
                <div class="hero-body container pb-auto">
                    <h1 class="title is-1">{ "Calendar" }</h1>
                </div>
            </div>
        }
    }
}

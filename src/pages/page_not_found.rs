use yew::prelude::*;

pub struct PageNotFound;

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <section class="hero is-medium">
                    <div class="hero-body ml-0">
                        <div class ="is-size-1 has-text-weight-bold">
                            { "ERROR 404" }
                        </div>
                        <div class="is-size-4 has-text-weight-bold">
                            { "\"NOT ALL THOSE WHO WANDER ARE LOST.\"" }
                        </div>
                        <div class="is-size-4">
                            { "-BUT YOU SEEM TO HAVE LOST YOUR WAY" }
                        </div>
                        <div class="is-size-5 pt-5">
                            { "It appears as if this page is missing. This could be because it has been deleted or moved but even more likely is that it has put on the one ring and disappeared. Sorry for any inconvenience." }
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}

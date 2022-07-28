use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub team: String,
    pub description: String,
    pub img_src: String,
}

pub struct Team {
    team: String,
    description: String,
    img_src: String,
}

impl Component for Team {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            team: (*ctx.props().team).to_string(),
            description: (*ctx.props().description).to_string(),
            img_src: (*ctx.props().img_src).to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self {
            team,
            description,
            img_src,
        } = self;
        html! {
            <div class="column">
                <figure class="image is-128by128 p-5">
                    <img class="is-rounded" src={ img_src.to_string() } loading="lazy" />
                </figure>
                <h1 class="subtitle has-text-centered">{ team }</h1>
                <div class="content is-size-6">
                    { description }
                </div>
            </div>
        }
    }
}

use yew::prelude::*;
use reqwasm::http::{Request, RequestMode};
use yew_router::components::Link;

use log;

use types::PostResponse;
use crate::{HOST, API_PORT};

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub id: String,
}

#[function_component(PostCard)] pub fn view(props: &Props) -> Html {
    let id = props.id.clone();

    let post_state = use_state(PostResponse::default);
    let first_load = use_state(|| true);

    let load = {
        let post_state = post_state.clone();

        Callback::from(move |_: bool| {
            let post_state = post_state.clone();
            let first_load= first_load.clone();

            if *first_load {
                let endpoint = format!("{}:{}/post/{}", HOST, API_PORT, &id);
                log::info!("Fetch: {}", &endpoint);

                wasm_bindgen_futures::spawn_local(async move {
                    let request = Request::get(&endpoint)
                        .header("Accept", "application/json")
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Request-Headers", "Content-Type")
                        .mode(RequestMode::Cors);
                    let fetched_response = request.send().await;

                    match fetched_response {
                        Ok(response) => {
                            let json: Result<PostResponse, reqwasm::Error> = response.json().await;
                            match json {
                                Ok(f) => {
                                    let mut post = (*post_state).clone();
                                    post.id = f.id.clone();
                                    post.title = f.title.clone();
                                    post.author = f.author.clone();
                                    post.categories = f.categories.clone();
                                    post.body = f.body.clone();
                                    post.time_created = f.time_created.clone();

                                    post_state.set(post);
                                    log::info!(
                                        "Fetch: Fetched post data with id of {:?}",
                                        f.id
                                    )
                                }
                                Err(e) => log::error!("Error parsing json: {:?}", e),
                            }
                        }
                        Err(e) => log::error!("Error parsing response: {:?}", e),
                    }
                });
                first_load.set(false);
            }
        })
    };

    load.emit(true);

    html! {
    <div class="card">
        <div class="card-image">
            <figure class="image is-2by1">
                <img alt="This post's image" src="https://this-person-does-not-exist.com/en" loading="lazy" />
            </figure>
        </div>
        <div class="card-content">
            <Link<Route> classes={classes!("title", "is-block")} to={Route::Post { id: post_state.id.clone() }}>
                { post_state.title.clone() }
            </Link<Route>>
            <p classes={classes!("subtitle", "is-block")}>
                { post_state.author.clone() }
            </p>
            <p classes={classes!("subtitle", "is-block")}>
                        { "id: " } { post_state.id.clone() }
            </p>
        </div>
    </div>
    }
}

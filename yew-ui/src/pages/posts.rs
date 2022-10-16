use yew::prelude::*;
use yew_router::prelude::*;

use reqwasm::http::{Request, RequestMode};

use crate::components::post_card::PostCard;

use types::PostResponse;
use crate::{HOST, API_PORT};

#[function_component(Posts)]
pub fn view() -> Html {
    log::info!("ON VIEW");

    let posts_state = use_state(Vec::new);
    let first_load = use_state(|| true);

    log::info!("Hello?!?!?");
    let load = {
        let posts_state = posts_state.clone();

        Callback::from(move |_: bool| {
            let posts_state = posts_state.clone();
            let first_load= first_load.clone();

            if *first_load {
                let endpoint = format!("{}/api/posts", HOST);
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
                            let json: Result<Vec<PostResponse>, reqwasm::Error> = response.json().await;
                            match json {
                                Ok(f) => {
                                    let posts: Vec<String> = f.iter().map(|v| v.id.clone()).collect();
                                    posts_state.set(posts.clone());

                                    log::info!(
                                        "Fetch: Fetched posts data with a value of {:?}",
                                        posts
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

    let ids = posts_state.to_vec();
    log::info!("Ids: {:?}", ids);
    let posts = {
        let mut cards = ids.iter().map(|id| {
            html! {
                <li class="list-item mb-5">
                    <PostCard id={id.clone()} />
                </li>
            }
        });
        html! {
            <div class="columns">
                <div class="column">
                    <ul class="list">
                        { for cards.by_ref().take(5) }
                    </ul>
                </div>
                <div class="column">
                    <ul class="list">
                        { for cards }
                    </ul>
                </div>
            </div>
        }
    };

    html! {
    <div class="section container">
        <h1 class="title">{ "Posts" }</h1>
        <h2 class="subtitle">{ "All of our quality writing in one place" }</h2>
        { posts }
    </div>
    }
}

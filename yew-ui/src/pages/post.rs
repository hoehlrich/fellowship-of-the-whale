use yew::prelude::*;
use reqwasm::http::{Request, RequestMode};

use log;

use types::PostResponse;
use crate::{HOST, API_PORT};

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub id: String,
}

#[function_component(Post)]
pub fn echo(PostProps { id }: &PostProps) -> Html {
    let id = id.clone();

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
        <div class="container">
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ &post_state.title }</p>
                    <p class="subtitle">{ "By: " } { &post_state.author }</p>
                    
                    <div class="content">
                        { &post_state.body }
                    </div>
                </div>
            </div>
        </div>
    }
}

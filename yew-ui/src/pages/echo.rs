use yew::prelude::*;
use reqwasm::http::{Request, RequestMode};

use log;

use types::EchoResponse;
use crate::{HOST, API_PORT};

#[derive(Properties, Clone, PartialEq)]
pub struct EchoProps {
    pub item: String,
}

#[function_component(Echo)]
pub fn echo(EchoProps { item }: &EchoProps) -> Html {
    let item = item.clone();

    let echo_state = use_state(EchoResponse::default);
    let first_load = use_state(|| true);

    let load = {
        let echo_state = echo_state.clone();

        Callback::from(move |_: bool| {
            let echo_state = echo_state.clone();
            let first_load= first_load.clone();

            if *first_load {
                let endpoint = format!("{}/api/echo/{}", HOST, &item);
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
                            let json: Result<EchoResponse, reqwasm::Error> = response.json().await;
                            match json {
                                Ok(f) => {
                                    let mut echo = (*echo_state).clone();
                                    echo.item = f.item.clone();
                                    echo_state.set(echo);
                                    log::info!(
                                        "Fetch: succeeded with value of {:?}",
                                        f.item
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
        <div>
            <p>{ "Echo: " } { &echo_state.item }</p>
        </div>
    }
}

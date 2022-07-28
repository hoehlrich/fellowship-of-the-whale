use yew::prelude::*;

use std::cell::RefCell;
use std::sync::Arc;

use reqwasm::http::{Request, RequestMode};

use log;

use crate::services::{echo::get_echo, future::handle_future};
use types::{EchoResponse, Status};

#[derive(Properties, Clone, PartialEq)]
pub struct EchoProps {
    pub item: String,
}

#[function_component(Echo)]
pub fn echo(EchoProps { item }: &EchoProps) -> Html {
    let item = item.clone();
    
    let echo = Arc::new(RefCell::new(EchoResponse::default()));
    let is_loading = Arc::new(RefCell::new(true));
    log::info!("Echo page loaded");
    {
        let mut is_loading = is_loading.clone();
        let mut echo = echo.clone();

        use_effect_with_deps(
            move |_| {
                log::info!("Update: request recieved at frontend");
                is_loading = Arc::new(RefCell::new(true));
                // let future = async move { get_echo(item.to_string()).await };
                // handle_future(future, move |data: Result<EchoResponse, Status>| {
                    // match data {
                        // Ok(data) => {
                            // log::info!("Update: data recieved with a value of {}", &data.item);
                            // echo = Arc::new(RefCell::new(data));
                              //      },
                        // Err(e) => {
                          //  log::error!("Error: {:?}", e);
                        //},
                    //};
                let endpoint = format!("http://127.0.0.1:8084/echo/{}", &item);
                is_loading = Arc::new(RefCell::new(false));
                wasm_bindgen_futures::spawn_local( async move {
                    let request = Request::get( &endpoint )
                        .header("Accept", "application/json")
                        .header("Content-Type", "application/json")
                        .header("Access-Control-Request-Headers", "Content-Type")
                        .mode(RequestMode::Cors);
                    let fetched_response = request.send().await;

                    match fetched_response {
                        Ok(response) => {
                            let json = response.json().await;
                            match json {
                                Ok(f) => {
                                    echo = Arc::new(RefCell::new(f));
                                    is_loading = Arc::new(RefCell::new(false));
                                    log::info!("Update: request succeeded with value of {:?}", echo)
                                },
                                Err(e) => log::error!("Error parsing json: {:?}", e),
                            }
                        },
                        Err(e) => log::error!("Error parsing response: {:?}", e),
                    }
                });
                || {}
            },
            (),
        );
    }

    let echo = echo.borrow_mut();
    let is_loading = is_loading.borrow_mut();
    log::info!("is_loading: {} echo: {}", is_loading, &echo.item);
    html! {
        {if *is_loading {
            html! { <p>{ "Loading..." }</p> }
        } else {
            html! { <p>{ "Echo: " } { &echo.item }</p> }
        }}
    }
}

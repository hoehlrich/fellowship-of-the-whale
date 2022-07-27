use yew::prelude::*;


use crate::services::{future::handle_future, echo::get_echo};
use types::{EchoResponse, Status};

#[derive(Properties, Clone, PartialEq)]
pub struct EchoProps {
    pub item: String,
}

#[function_component(Echo)]
pub fn echo(EchoProps { item }: &EchoProps) -> Html {
    let item = item.clone();
    
    let is_loading = use_state(|| false);
    let echo = use_state(|| EchoResponse::default());
    
    {
        let is_loading = is_loading.clone();
        let echo = echo.clone();

        use_effect_with_deps(
            move |_| {
                is_loading.set(true);
                let future = async move { get_echo(item.to_string()).await };
                handle_future(future, move |data: Result<EchoResponse, Status>| {
                    match data {
                        Ok(data) => echo.set(data),
                        Err(_) => (),
                    };
                is_loading.set(false)
                });
                || {}
            },
            (),
        );
    }
    html! {
        <div>
            <p> { "Echo: "} { &echo.item } </p>
        </div>
    }
}



use super::fetch::Fetch;
use types::{EchoResponse, Status};

use log;

pub async fn get_echo(item: String) -> Result<EchoResponse, Status> {
    log::info!("Update: request recieved at future gen");

    let url = format!("127.0.0.1:8084/echo/{}", item);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<EchoResponse>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

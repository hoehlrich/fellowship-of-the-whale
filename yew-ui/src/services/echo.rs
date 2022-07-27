use types::{EchoResponse, Status};
use super::fetch::Fetch;

pub async fn get_echo(item: String) -> Result<EchoResponse, Status> {
    let url = format!("127.0.0.1:8080/echo/{}", item);
    let json = Fetch::get(url).await;

    match json {
        Ok(json) => Ok(json.into_serde::<EchoResponse>().unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
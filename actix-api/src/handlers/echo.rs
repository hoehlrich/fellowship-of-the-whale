use actix_web::web::{self, Json};
use types::EchoResponse;

pub async fn get(item: web::Path<String>) -> Json<EchoResponse> {
    Json(EchoResponse {
        item: item.to_string(),
    })
}

use types::EchoResponse;
use actix_web::web::{self, Json};

pub async fn get(item: web::Path<String>) -> Json<EchoResponse> {
    Json(EchoResponse {
        item: item.to_string()
    })
}
use actix_cors::Cors;
use actix_web::{
    http,
    web::{self},
    App,
    HttpServer
};
use actix_files as fs;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ACTIX_PORT: &str = "8084";
    const UI_PORT: &str = "8080";
    const UI_HOST: &str = "127.0.0.1";

    let allowed_origin = format!("http://{}:{}", UI_HOST, UI_PORT);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(allowed_origin.as_str())
            .allowed_methods(["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        
            App::new()
                .wrap(cors)
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .route("/echo/{item}", web::get().to(handlers::echo::get))
    })
    .bind(("0.0.0.0", ACTIX_PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}
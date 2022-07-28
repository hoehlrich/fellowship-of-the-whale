use actix_cors::Cors;
use actix_web::{
    http,
    web,
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

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(["GET", "POST"])
            .max_age(3600);
        
            App::new()
                .wrap(cors)
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .route("/echo/{item}", web::get().to(handlers::echo::get))
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}

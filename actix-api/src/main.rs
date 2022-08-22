use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};

use mysql::prelude::*;
use mysql::*;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // MySQL Steup
    let url = "mysql://root:password@localhost:3306/fotw";
    let pool = Pool::new(url).expect("Error creating pool");

    let mut conn = pool.get_conn().expect("Error creating connection");

    match conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS posts (
            id text not null,
            title text not null,
            author text not null,
            categories text,
            body text not null,
            timeCreated text not null
            )",
    ) {
        Ok(_) => (),
        Err(e) => println!("Error creating table: {}", e),
    };

    const ACTIX_PORT: &str = "8084";

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(["GET", "POST"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/echo/{item}", web::get().to(handlers::echo::get))
            .route("/posts", web::get().to(handlers::posts::get_posts))
            .route("/post/{id}", web::get().to(handlers::posts::get_post))
            .route("/post", web::post().to(handlers::posts::add_post))
    })
    .bind(("127.0.0.1", ACTIX_PORT.parse::<u16>().unwrap()))?
    .run()
    .await
}

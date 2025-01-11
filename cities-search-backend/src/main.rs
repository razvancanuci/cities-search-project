mod routes;
mod handlers;
mod models;
mod qdrant;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(move || {

        let cors = Cors::default()
            .allowed_origin("https://localhost:7238")
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);
        
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes::init_routes)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

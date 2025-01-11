use actix_web::web;
use crate::handlers::search_handler::search_cities;

pub fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/search")
            .service(search_cities)
    );
}
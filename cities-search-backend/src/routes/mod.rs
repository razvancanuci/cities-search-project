use actix_web::web;

pub mod search_routes;
mod weather_routes;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    search_routes::add_routes(cfg);
    weather_routes::add_routes(cfg);
}
use actix_web::web;
use crate::handlers::weather_handler::get_weather;

pub fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/weather")
            .service(get_weather)
           
    );
}
use std::env;
use actix_web::{get, web, HttpResponse};
use crate::models::weather_models::{QueryWeather, WeatherModel};

#[get("")]
pub async fn get_weather(query: web::Query<QueryWeather>) -> HttpResponse {
    let weather_api_url = env::var("WEATHER_API").expect("WEATHER_API_URL should be set");
    let api = format!("{weather_api_url}/weather/{}", query.city);
    
    let response = reqwest::get(api).await;
    if response.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    
    let response = response.unwrap().json::<WeatherModel>().await;
    if  response.is_err() {
        return HttpResponse::InternalServerError().finish();
    } 
    
    HttpResponse::Ok().json(response.unwrap())
}
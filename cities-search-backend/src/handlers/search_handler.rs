use std::env;
use actix_web::{get, web, HttpResponse};
use crate::models::search_models::{QuerySearch, SearchApiResponse, SearchResponse};
use crate::qdrant::qdrant_client::QdrantClientWrapper;

#[get("")]
pub async fn search_cities(query: web::Query<QuerySearch>) -> HttpResponse {
    let city = query.input.clone();
    let client = reqwest::Client::new();
    
    let api_key = env::var("API_KEY").expect("API_KEY should be set");
    let embedding_api_url = env::var("EMBEDDING_API").expect("EMBEDDING_API_URL should be set");
    let request = client.get(format!("{embedding_api_url}/api/embedding/{city}")).header("X-Api-Key", api_key).build().unwrap();
    let embedding_response = client.execute(request).await;
    if embedding_response.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let embedding = embedding_response.unwrap().json::<SearchApiResponse>().await;
    if embedding.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let embedding = embedding.unwrap();
    
    let qdrant_client = QdrantClientWrapper::new();
    let qdrant_result = qdrant_client.search(embedding.result).await;
    if qdrant_result.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    let qdrant_result = qdrant_result.unwrap().into_iter().map(|point| SearchResponse::from(point)).collect::<Vec<SearchResponse>>();
    
    HttpResponse::Ok().json(qdrant_result)
}
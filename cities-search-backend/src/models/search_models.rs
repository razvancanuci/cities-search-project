use qdrant_client::qdrant::ScoredPoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
pub struct QuerySearch {
    pub input: String
}

#[derive(Default, Deserialize, Serialize)]
pub struct SearchApiResponse {
    pub result: Vec<f32>
}

#[derive(Deserialize, Serialize)]
pub struct SearchResponse {
    pub city: String,
    pub country: String,
    pub population: u32,
    pub latitude: f32,
    pub longitude: f32,
    pub confidence: f32
}

impl From<ScoredPoint> for SearchResponse {
    fn from(value: ScoredPoint) -> Self {
        Self {
            city: value.payload["city"].to_string().replace("\"", ""),
            country: value.payload["country"].to_string().replace("\"", ""),
            population: value.payload["population"].as_integer().unwrap() as u32,
            latitude: value.payload["latitude"].as_double().unwrap() as f32,
            longitude: value.payload["longitude"].as_double().unwrap() as f32,
            confidence: value.score
        }
    }
}

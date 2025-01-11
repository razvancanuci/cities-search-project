use qdrant_client::{Qdrant, QdrantError};
use qdrant_client::qdrant::{QueryPointsBuilder, ScoredPoint};

pub struct QdrantClientWrapper {
    client: Qdrant,
    collection_name: String
}

impl QdrantClientWrapper {
    pub fn new() -> Self {
        let client = Qdrant::from_url("http://localhost:6334").build().expect("Failed to create Qdrant client");
        QdrantClientWrapper { client, collection_name: "cities_collection".to_string() }
    }

    pub async fn search(&self, query_search: Vec<f32>) -> Result<Vec<ScoredPoint>,QdrantError> {
        let result = self.client
            .query(
                QueryPointsBuilder::new(&self.collection_name)
                    .query(query_search)
                    .limit(15)
                    .with_payload(true)
            )
            .await?;
        Ok(result.result)
    }
}
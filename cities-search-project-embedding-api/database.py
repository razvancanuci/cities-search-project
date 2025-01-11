import os

import pandas as pd
from qdrant_client import QdrantClient
from qdrant_client.http.models import VectorParams, Distance
from sentence_transformers import SentenceTransformer

def create_qdrant_db():
    qdrant_url = "http://localhost:6333"
    client = QdrantClient(qdrant_url)
    collection_name = "cities_collection"
    if client.collection_exists(collection_name):
        print('Collection already exists')
        return

    dataSet = os.getenv("DATASET")
    cities = pd.read_csv(dataSet)

    city_names = cities['city'].tolist()
    model_name = os.getenv("MODEL_NAME")
    model = SentenceTransformer(model_name)
    city_vectors = model.encode(city_names)


    client.create_collection(
        collection_name=collection_name,
        vectors_config=VectorParams(size=city_vectors.shape[1], distance=Distance.COSINE)
    )

    vectors = city_vectors.tolist()

    payload = cities.to_dict(orient="records")
    client.upload_collection(
        collection_name=collection_name,
        vectors=vectors,
        payload=payload,
        ids=list(range(len(city_names)))
    )
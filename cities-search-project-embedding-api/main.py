import os

from dotenv import load_dotenv
from fastapi import FastAPI, Request, HTTPException
from sentence_transformers import SentenceTransformer
from starlette.responses import JSONResponse

from database import create_qdrant_db

load_dotenv()

API_KEY = os.getenv("API_KEY")

create_qdrant_db()
app = FastAPI()


@app.middleware("http")
async def check_api_key(request: Request, call_next):
    api_key = request.headers.get("X-Api-Key")
    if not api_key or api_key != API_KEY:
        raise HTTPException(status_code=401, detail="Invalid or missing API key")

    response = await call_next(request)
    return response

@app.get("/api/embedding/{input}")
async def say_hello(input: str):
    model_name = os.getenv("MODEL_NAME")
    model = SentenceTransformer(model_name)
    city_vectors = model.encode(input)
    return JSONResponse(content={"result": city_vectors.tolist()})

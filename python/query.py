import hnswlib
import numpy as np
import time
import os
import pickle
from sentence_transformers import SentenceTransformer
model = SentenceTransformer('all-MiniLM-L6-v2')

#Given a search query, return the most similar sentence. Dependencies: embeddings and file names must already exist in a pkl file.
def search_for_query(query):
    embeddings_location, names_location = ['./data/embeddings_location.pkl', './data/names_location.pkl']
    with open(f"./{embeddings_location}",'rb') as file: embeddings = pickle.load(file)
    with open(f"./{names_location}",'rb') as file: file_names = pickle.load(file)

    dimension = embeddings.shape[1]
    p = hnswlib.Index(space='cosine', dim=dimension)
    p.init_index(max_elements=1000000, ef_construction=200, M=16)
    p.add_items(embeddings)
    p.set_ef(50) # Controlling the recall by setting ef: ef should always be > k

    new_embedding = model.encode([query])

    start_time = time.time()
    labels, distances = p.knn_query(new_embedding, k=1)
    end_time = time.time()
    return f"Most similar sentence is: {file_names[labels[0][0]]}. Elapsed search time: {end_time - start_time}"
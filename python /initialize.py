import hnswlib
import numpy as np
import time
import os
import pickle
from sentence_transformers import SentenceTransformer
model = SentenceTransformer('all-MiniLM-L6-v2')

#Generate embeddings and file names for all files in a folder and its subfolders. Save them in a pkl file.
def get_file_names(root_folder):
    embeddings_location = 'embeddings.pkl'
    names_location = 'file_names.pkl'

    endings = []
    for ending in open('./data/endings.txt', 'r'): endings.append('.' + ending.strip())
    excluded_folders = []
    for folder in open('./data/excluded_folders.txt', 'r'): excluded_folders.append('.' + folder.strip())

    file_names = []
    for root, dirs, files in os.walk(root_folder):
        for excluded_folder in excluded_folders:
            if excluded_folder in dirs: dirs.remove(excluded_folder)
        for file in files:
            if file.endswith(tuple(endings)): file_names.append(os.path.join(root, file))
    file_names = list(map(lambda x: x.replace(root_folder, ''), file_names))
    embeddings = model.encode(file_names)
    with open('./data/embeddings_location.pkl', 'wb') as f: pickle.dump(embeddings, f)
    with open('./data/names_location.pkl', 'wb') as f: pickle.dump(file_names, f)
    return embeddings_location, names_location
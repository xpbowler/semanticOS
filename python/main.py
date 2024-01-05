import hnswlib
import numpy as np
from sentence_transformers import SentenceTransformer
import time
import os
import pickle
from initialize import *
from query import *
model = SentenceTransformer('all-MiniLM-L6-v2')

# root_folder = '../'
# get_file_names(root_folder)

query = 'primary'
print(search_for_query(query))


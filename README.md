# SemanticOS

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.md)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/xpbowler/semanticOS.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/xpbowler/semanticOS)

SemanticOS is a lightweight desktop application, built in Rust and packaged by Tauri, for fast semantic file searching. Uses local vector embedding generation (BERT) and a K-dimensional Tree vector searching algorithm O(logn).

## 🔨 Installation

Requirements: 
* System: Rust (1.75.0), Tauri (1.5.4), React (18.2.0)
* Misc: permission to access/edit files (Windows)

Environment variables are :

|        Name         |               Description               | Required | Default value |                   Limitations                    |
|:-------------------:|:---------------------------------------:|:--------:|:-------------:|:------------------------------------------------:|
|`PROTOCOL_BUFFERS_PYTHON_IMPLEMENTATION`   |  Tensorflow environment variable for Mac |    ✅     |       ❌       |  Set to `python`  |
|   `DATABASE_HOST`   |        MongoDB database host URL        |    ✅     |       ❌       |              Can't be empty string               |

File structure:

```bash
.
├── R # R scripts used to pre-process raw RNA-seq data
├── c_images # processed RNA-seq dataset (2D-transformed)
├── client # React frontend directory
│   ├── database # express server connecting to MongoDB database
│   ├── public # React HTML renderer
│   ├── src # javascript files to run website
│   │  ├── components # various react pages for website
│   │  ├── sample_images # sample 2D-image inputs for CNN for testing purposes
├── ics4u_cnn2 # Weights for CNN
├── flask-server # Python Flask backend for handling predictions
├── package.json # project metadata
├── package-lock.json # project dependencies
└── README.md 

```

Instructions:
1. Clone the repository
```
$ git clone [https://github.com/xpbowler/semanticOS](https://github.com/xpbowler/semanticos)
```
2. Install required dependencies via ```npm install```
4. Run desktop application via ```npm run tauri dev```

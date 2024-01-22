# SemanticOS

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.md)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/xpbowler/semanticOS.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/xpbowler/semanticOS)

SemanticOS is a lightweight desktop application, built in Rust and packaged by Tauri, for fast semantic file searching. It employs local vector embedding generation (BERT) and a K-dimensional Tree vector searching algorithm O(logn).

Both the [all-MiniLM-L6-v2](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2) model from Hugging Face and [Embed-V3](https://txt.cohere.com/introducing-embed-v3/) model from Cohere are used for embedding generation. The embeddings and metadata are stored in binary using serialization / deserialization from [Rust Serde](https://serde.rs/). 



It features a file-name generator using a small scale transformer architecture, implementing multi-headed self attention with a bigram language model trained in PyTorch and ported to Rust.

Current implementation follows a few key papers:
- Bigram Language Model
- Transformers, following [Vaswani et al. 2017](https://arxiv.org/abs/1706.03762)
- Residual Connections, following [He et al. 2015](https://openaccess.thecvf.com/content_cvpr_2016/html/He_Deep_Residual_Learning_CVPR_2016_paper.html)

When using SemanticOS, the search scope can be specified inside the application. The default search scope is the parent directory of the SemanticOS folder (```./```)

## Benchmarks 

#### Windows file search time:
```
3min with native file explorer    -->    ~500ms inside directory with up to 2M files.
```

#### Initialization (Vector embedding generation):
```
~1min / 1M files
```

#### Disk,RAM space usage:
```
Vector embeddings: ~30MB / 1M files. 
Vector embedding model: 86.6MB
RAM usage: ~190MB during initialization, <70MB while searching/idle
```

## 🔨 Installation

Requirements :

|        Name         |               Description               | Required | Default value |                   Limitations                    |
|:-------------------:|:---------------------------------------:|:--------:|:-------------:|:------------------------------------------------:|
|`Rust`   |   Run Tauri backend  |    ✅     |       ❌       |  Recommended v1.75.0  |
|  `cargo`  | Load backend dependencies  |    ✅     |       ✅       |                          |
|   `Tauri`   |        Tauri packager        |    ✅     |       ❌       |              Recommended v1.5.4            |
|  `npm`  | Install frontend dependencies  |    ✅     |       ✅       |                          |

* Misc: Permission to access/edit files (Windows)
  
Github file structure:

```bash
.
├── R # R scripts used to pre-process raw RNA-seq data
├── c_images # processed RNA-seq dataset (2D-transformed)
├── src # React frontend desktop UI
├── public # Frontend resources
├── src-tauri # Rust backend
│   ├── data # Backend metadata
│   │  ├── all-MiniLM-L6-V2.onnx #BERT vector embedding model ported to Rust
│   ├── functions # Helper functions for search+embeddings
│   ├── icons # Tauri resources
│   ├── src # Main rust progam
│   │  ├── Cargo.toml # Specification of cargo dependencies
├── index.html 
├── package.json # project frontend metadata
├── package-lock.json # project frontend dependencies
└── README.md 

```

Instructions:

1. Clone the repository (<100MB including vector embedding model)
```
$ git clone https://github.com/xpbowler/semanticos
```
2. Install required dependencies
```
$ npm install
```
4. Run desktop application. This step can take a while for the first build, as Rust is loading all the necessary crates.
```
$ npm run tauri dev
```

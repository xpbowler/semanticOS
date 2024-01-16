# SemanticOS

[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE.md)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/xpbowler/semanticOS.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/xpbowler/semanticOS)
[![Build Status](https://github.com/xpbowler/semanticOS/workflows/Build/badge.svg?event=push)](https://github.com/xpbowler/semanticOS/actions)

SemanticOS is a lightweight desktop application, built in Rust and packaged by Tauri, for fast semantic file searching. It employs local vector embedding generation (BERT) and a K-dimensional Tree vector searching algorithm O(logn).

It features a file-name generator using a small scale transformer architecture, implementing multi-headed self attention with a bigram language model trained in PyTorch and ported to Rust.

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
Vector embeddings: ~30MB / 1M files
Vector embedding model: 86.6MB
RAM usage: ~2GB during search, 700MB while idle
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
  
File structure:

```bash
.
├── R # R scripts used to pre-process raw RNA-seq data
├── c_images # processed RNA-seq dataset (2D-transformed)
├── src # React frontend desktop UI
├── public # Frontend resources
├── src-tauri # Rust backend
│   ├── data # Backend metadata
│   ├── functions # Helper functions for search+embeddings
│   ├── icons # Tauri resources
│   ├── src # Main rust progam
│   │  ├── Cargo.toml # Specification of cargo dependencies
│   │  ├── all-MiniLM-L6-V2.onnx #BERT vector embedding model ported to Rust
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

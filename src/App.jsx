import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import {writeTextFile, BaseDirectory} from '@tauri-apps/api/fs';
import {downloadDir, desktopDir} from '@tauri-apps/api/path';
import {open} from '@tauri-apps/api/shell';

function App() {
  const [searchResult, setSearchResult] = useState("");
  const [x, setX] = useState("hmm");
  const [query, setQuery] = useState("");

  async function search() {
    const path = await desktopDir();
    setSearchResult(await invoke("search", { query }));
    //await open(path);
    //setX(path);
    //await writeTextFile('example.txt', 'Hello world!\n', { dir: BaseDirectory.Download });
  }

  async function get_embeddings(){
    setX(await invoke("initialize"));
  }

  return (
    <div className="container">
      <h1>SemanticOS</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          search();
        }}
      >
        <input
          id="query-input"
          onChange={(e) => setQuery(e.currentTarget.value)}
          placeholder="Query..."
        />
        <button type="submit">Find</button>
      </form>
      <p>{searchResult}</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_embeddings();
        }}
      >
        <button type="submit">Generate Embeddings</button>
      </form>
      <p>{x}</p>

    </div>
  );
}

export default App;
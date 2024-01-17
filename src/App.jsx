import { useState } from "react";
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
    let sub_dir = await invoke("search",{query});
    let absolute_path = await invoke("get_absolute_path");
    let path = absolute_path + sub_dir;
    setSearchResult(path);
    await open(searchResult);
  }

  async function get_embeddings(){
    setX(await invoke("initialized"));
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
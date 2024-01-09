import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [searchResult, setSearchResult] = useState("");
  const [x, setX] = useState("hmm");
  const [query, setQuery] = useState("");
  const folderPath = "C:\\Users\\rnqua\\Files\\Projects\\semanticOS\\src-tauri";

  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setSearchResult(await invoke("search", { query }));
    await invoke("open", { uri: "file://${folderPath}" })

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

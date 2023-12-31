import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [searchResult, setSearchResult] = useState("");
  const [query, setQuery] = useState("");

  async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setSearchResult(await invoke("search", { query }));
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
    </div>
  );
}

export default App;

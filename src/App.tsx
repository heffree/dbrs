import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [connectionUrl, setConnectionUrl] = useState("");
  const [sql, setSql] = useState("");
  const [output, setOutput] = useState("");

  async function connectToDb() {
    setOutput(await invoke("connect_to_db", { connectionUrl }));
  }

  async function executeSql() {
    console.log('hello');
    setOutput(await invoke("execute_sql", { sql }));
  }

  return (
    <main className="container">
      <h1>DBRS</h1>
      Connection URL: <input id="connection-url" type="text" onInput={(event) => setConnectionUrl(event.currentTarget.value)} value={connectionUrl} />
      <button id="connect-button" onClick={connectToDb}>Connect</button>
      <textarea id="input" onInput={(event) => setSql(event.currentTarget.value)}>{sql}</textarea>
      <button id="execute-button" onClick={executeSql}>Execute</button>
      <div id="output">{output}</div>
    </main>
  );
}

export default App;

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { ConnectionList } from "./components/ConnectionList";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

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
      <PanelGroup direction="horizontal">
        <Panel defaultSize={30} minSize={20} className="panel">
          <ConnectionList />
        </Panel>
        <PanelResizeHandle className="handle" />
        <Panel className="panel" minSize={70}>
          <PanelGroup direction="vertical">
            <Panel minSize={30} className="panel">
              <textarea id="input" onInput={(event) => setSql(event.currentTarget.value)}>{sql}</textarea>
              <button id="execute-button" onClick={executeSql}>Execute</button>
            </Panel>
            <PanelResizeHandle className="handle2" />
            <Panel defaultSize={30} minSize={20} className="panel">
              <div id="output">{output}</div>
            </Panel>
          </PanelGroup>
        </Panel>
      </PanelGroup>
    </main>
  );
}

export default App;

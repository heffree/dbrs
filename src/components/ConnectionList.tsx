import { useAppStore } from "../app-state";

export function ConnectionList() {
  const connections = useAppStore((state) => state.connections);
  return (
    <div id="connection-list">
      {connections.map((connection) =>
        <div>
          {connection}
        </div>
      )}
    </div>
  );
}

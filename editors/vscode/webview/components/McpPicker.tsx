import { useEffect } from "react";
import type { McpServer } from "../../src/protocol";
import { post } from "../lib/host";
import { redactSecrets } from "../lib/redact";
import { ChevronIcon } from "./icons";

/** The `/mcp` control panel: every configured server with its state. The switch
 *  flips `disabled` in whichever config file declares it; opening the row raises
 *  the server's own panel, with the actions a single server takes. It stays open
 *  across toggles, since turning two servers off is one errand. */
export function McpPicker({
  servers,
  onToggle,
  onOpen,
}: {
  servers: McpServer[];
  onToggle: (name: string, disabled: boolean) => void;
  onOpen: (name: string) => void;
}) {
  // The config can change under us between openings, so the list is re-read
  // rather than cached from whenever the panel last loaded.
  useEffect(() => {
    post({ type: "listMcp" });
  }, []);

  return (
    <div className="picker" role="dialog" aria-label="MCP servers">
      <div className="picker-head">MCP servers</div>
      {servers.length === 0 && (
        <div className="picker-empty">
          No servers configured. Add them to .mcp.json or `mcp:` in aster.yaml, or run `aster mcp
          import`.
        </div>
      )}
      {servers.map((server) => (
        <div key={server.name} className="picker-row mcp-row" data-selected={!server.disabled}>
          <button
            type="button"
            role="switch"
            aria-checked={!server.disabled}
            aria-label={`${server.disabled ? "Enable" : "Disable"} ${server.name}`}
            className="mcp-switch"
            onClick={() => onToggle(server.name, !server.disabled)}
          >
            <span className="mcp-switch-knob" />
          </button>
          <button
            type="button"
            className="mcp-row-open"
            aria-label={`Actions for ${server.name}`}
            onClick={() => onOpen(server.name)}
          >
            <span className="picker-body">
              <span className="picker-label">{server.name}</span>
              <span className="picker-detail">{describe(server)}</span>
            </span>
            <span className="mcp-chev" aria-hidden="true">
              <ChevronIcon open={false} />
            </span>
          </button>
        </div>
      ))}
    </div>
  );
}

function describe(server: McpServer): string {
  if (server.url) return redactSecrets(server.url);
  const command = [server.command, ...server.args].join(" ").trim();
  return command ? redactSecrets(command) : (server.transport ?? "unconfigured");
}

import type { McpServer } from "../../src/protocol";
import { post } from "../lib/host";
import { redactSecrets } from "../lib/redact";
import { Modal } from "./Modal";

/** One server's own panel, raised from the `/mcp` picker: what it is, whether
 *  it is on, and the actions a single server takes. Reauth runs `aster mcp
 *  login` in a terminal, since the OAuth flow opens a browser and waits. */
export function McpServerModal({
  server,
  onToggle,
  onClose,
}: {
  server: McpServer | null;
  onToggle: (name: string, disabled: boolean) => void;
  onClose: () => void;
}) {
  if (!server) return null;
  const remote = Boolean(server.url);
  return (
    <Modal label={`${server.name} server`} className="mcp-server-modal" align="center" onClose={onClose}>
      <span className="info-title">{server.name}</span>
      <p className="mcp-server-target">{describe(server)}</p>

      <div className="mcp-action">
        <div className="mcp-action-text">
          <span className="picker-label">{server.disabled ? "Disabled" : "Enabled"}</span>
          <span className="picker-detail">
            {server.disabled
              ? "Its tools are left out of new turns."
              : "Its tools are offered on every new turn."}
          </span>
        </div>
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
      </div>

      {remote && (
        <button
          type="button"
          className="mcp-action-btn"
          onClick={() => {
            post({ type: "mcpLogin", name: server.name });
            onClose();
          }}
        >
          <span className="picker-label">Sign in again</span>
          <span className="picker-detail">
            Opens a terminal and runs the login step for this server.
          </span>
        </button>
      )}
    </Modal>
  );
}

function describe(server: McpServer): string {
  if (server.url) return redactSecrets(server.url);
  const command = [server.command, ...server.args].join(" ").trim();
  return command ? redactSecrets(command) : (server.transport ?? "unconfigured");
}

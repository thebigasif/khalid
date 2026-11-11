Added
- The speed and uptime watch has an API route and an MCP tool. `GET /api/admin/v1/watch/` answers, for a key with `settings:write`, how fast and how reliably the blog answered at its public address over the last 24 hours and the last 30 days, in the shape `khalid watch --json` prints. Agents read the same report through `khalid mcp` with the `uptime_report` tool.

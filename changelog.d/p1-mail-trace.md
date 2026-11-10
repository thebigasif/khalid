Added
- `khalid serve` now writes a log to standard error: plain lines, or JSON with `--json` or `[log] format = "json"`, at the level `[log] level` names, `info` unless set. An email that could not be sent, such as a form answer, a welcome or an alert, leaves a warning there with the reason. No password, API key or token reaches the log (G38).

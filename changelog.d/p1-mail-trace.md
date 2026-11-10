Added
- `khalid doctor` checks mail: it says which way mail leaves and whether the password, API key or token it needs is set, without showing it, and `--send-test-mail` sends one email through it and says why if that fails (G38).
- `khalid serve` now writes a log to standard error: plain lines, or JSON with `--json` or `[log] format = "json"`, at the level `[log] level` names, `info` unless set. An email that could not be sent, such as a form answer, a welcome or an alert, leaves a warning there with the reason. No password, API key or token reaches the log (G38).

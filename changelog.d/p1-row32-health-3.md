Added
- `khalid serve` audits the blog once a day, the way `khalid audit --offline` does, and keeps the health score in `data/audit/runs.jsonl`. The first audit runs an hour after the server starts.
- `GET /api/admin/v1/site/health/` sends the last audit's health score, its counts and up to the last 30 audits as the trend.
- The admin dashboard shows the site health score out of 100, what the last audit found and how the score moved.

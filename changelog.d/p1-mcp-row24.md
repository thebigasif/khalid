Added
- `khalid mcp` now works on the site's real comments, media library and stored redirects. Before, those tools answered that the site had none. A running `khalid serve` sees what an agent changes, and what `khalid redirects import` stores, within about 5 seconds, without a restart.
- `khalid mcp` tools now say what they do to the site: each one carries the Model Context Protocol's read-only, destructive and idempotent hints, taken from its route's method, so an agent's client can ask a person before a call that overwrites or removes something.

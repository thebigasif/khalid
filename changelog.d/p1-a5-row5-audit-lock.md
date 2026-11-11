Fixed
- `khalid audit` and the nightly audit in `khalid serve` no longer build into `data/audit/site/` at the same time. The audit that runs keeps a lock in `data/audit/running`. An audit started meanwhile stops and says why, and the server tries again an hour later. A lock six hours old is taken over.

Added
- The performance budgets now measure the full site audit of 1,000 posts. `khalid-perf audit` times three `khalid audit --offline` runs on the generated 1,000-post site and reports the median, so the 60 second budget is checked on every run of `scripts/perf.sh` instead of showing "not measured yet".

Fixed
- `docs/standards/performance.md` no longer speaks of a 55 MB idle memory budget: the budget, the table and the gate have always said 50 MB.

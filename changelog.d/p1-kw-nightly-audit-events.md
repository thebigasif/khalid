Added
The nightly site audit in `khalid serve` now fires the `audit.finished` webhook and raises the blog's broken links to the owner, as `khalid audit` does. Stopping the server no longer waits for an audit build: the audit lock goes at once and the unfinished audit keeps no score.

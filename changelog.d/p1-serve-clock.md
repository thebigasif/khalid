Changed
- A new test runs the real `khalid serve` across two restarts and proves the welcome sequence keeps its days: a reader who joins gets `welcome-day-3.md` from the hourly round once three days have passed, not on day two, and only once. The newsletter now reads the same server clock as the scheduler and preview links, which only a test build can move. No change to how the server behaves.

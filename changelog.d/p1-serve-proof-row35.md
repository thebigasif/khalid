Changed
- Two new tests run the real `khalid serve` across a restart. A post scheduled for a time that passes while the server is stopped goes live at the next start, fires `post.published` once and tells IndexNow. A preview link still opens after a restart and stops working once its seven days are up, showing the "This preview link has expired" page. No change to how the server behaves.

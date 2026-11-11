Changed
- The 404 log no longer stops recording once it holds 10,000 addresses. A new address now takes the place of the one asked for least, and of those the one seen longest ago, so real missing pages and their spike alerts still show up after a scanner has filled the log.
- `khalid serve` now cleans its database at start and once an hour. It deletes addresses in the 404 log nobody asked for in 90 days, webhook deliveries that finished more than 30 days ago, and sessions and sign-in links that can no longer be used. Deliveries still being tried and everything in use stay.

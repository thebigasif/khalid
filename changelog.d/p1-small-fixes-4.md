Fixed
- The slow-site alert now works when the speed and uptime watch looks every 30 or 60 minutes. It judges the last hour, or the last three looks when the watch looks less often than every 20 minutes. Before, a rare watch could never collect the three answers the alert needs.
- When the blog's address in `site.toml` is a private address, the server log now says in plain words that the watch did not look at it, why, and what to change.

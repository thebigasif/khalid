Fixed
- Checking a post before it is published is about three times faster on a large site: 14 ms instead of 45 ms for one post on a 1,000-post site. The check for posts that copy each other now reads each other post once and skips the ones that cannot be copies, and it finds the same copies as before.

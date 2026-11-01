Fixed
- Checking a post before it is published is about three times faster on a large site: 14 ms instead of 45 ms for one post on a 1,000-post site. The check for posts that copy each other now reads each other post once and skips the ones that cannot be copies, and it finds the same copies as before.
- A post saved while `khalid serve` is still starting now shows at once. On a 10,000-post site it used to wait about 40 seconds, for two or three renders of the whole site. The saved page now shows in about 15 ms, and the rest of the site follows when its render ends.

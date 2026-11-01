Fixed
- `khalid redirects explain` now knows the old addresses listed in a post's `aliases`. It names the post, and a stored redirect that points at an alias ends at the post in one step, as `khalid serve` sends it. An alias of a post the linter blocks is shown as waiting for that post (KH-SEO-8).
- `khalid import` and `khalid redirects import` say "1 redirect" and "1 file" instead of "1 redirects" and "1 files".

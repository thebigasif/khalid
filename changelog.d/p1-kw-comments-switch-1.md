Added
Comments can be closed or turned off. A `[comments]` table in `site.toml` sets `state = "open"`, `"closed"` or `"off"` for the whole site, and a post's own `comments:` field wins over it. A closed post keeps its approved comments and shows "Comments are closed on this post." in place of the form; a post with comments off shows none. The comment form refuses a post that is not open.

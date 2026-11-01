Fixed
- `khalid new` leaves the filler words and years out of a post's address, such as "that" or "2026", so its own linter no longer warns about the address it just made. A title made only of filler words keeps them.
- The "links to none of your other posts" block mentions the home page only when the post really is the only one in its language. Otherwise it asks for a link to a related post and lists the best ones to try.
- `khalid audit` says which pages it counts, the ones readers and search engines reach, since `build` counts every page it writes. It also says the health score is the share of pages with no error, so warnings do not lower it.
- `khalid new` says a post shows after the next build, or at once while `khalid serve` runs.
- The `routes.toml` that `khalid init` writes explains the default addresses and holds an example to uncomment, instead of pointing at an internal planning file.

Fixed
- A post with no `slug:` line now works with `khalid serve` and `khalid db rebuild` as it already did with `khalid build`: its address comes from its title, the same in all three. Before, `serve` stopped and `db rebuild` refused the post with "has no slug". An author with no `slug:` takes one from their name the same way.

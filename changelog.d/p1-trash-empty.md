Added
- `khalid serve` now empties the trash on its own: a deleted post is kept for 30 days, then deleted for good. Set `trash_days` in `site.toml` to keep it longer or shorter, or `trash_days = 0` to keep every deleted post until you delete it yourself.

Changed
- The staff toolbar on public pages remembers what the linter said about a post for 5 minutes, or until the post is saved, so moving between pages no longer checks the post again on every view.

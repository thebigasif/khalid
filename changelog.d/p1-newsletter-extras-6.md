Added
- A newsletter's welcome can now be a short sequence. Beside `welcome.md`, a file such as `content/newsletters/weekly/welcome-day-3.md` goes out three days after a reader joins. `khalid serve` looks once an hour and sends each email once.
- A reader who already confirmed and signs up for one more newsletter now gets that newsletter's welcome.
- Newsletter emails can count clicks on their links. Add `count_clicks = true` under `[mail]` in `site.toml`; `khalid newsletter numbers` then shows the clicks on each email. Every reader's copy has the same links, so clicks are never tied to a person. Off unless you set it.

Changed
- `khalid serve` reads the welcome files each time a reader is due one, so an edited `welcome.md` needs no restart.

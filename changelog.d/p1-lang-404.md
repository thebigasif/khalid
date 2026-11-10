Fixed
- `khalid serve` answers a missing address under a language's own folder, such as `/de/`, with that language's "page not found" page: its own language tag, words and head. Before, every missing address got the default language's page. `khalid build` already wrote one per language.
- A render that started before a save and ends after it no longer brings back the old list of password posts or the old search page.

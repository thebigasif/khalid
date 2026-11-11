### Fixed
- A misspelt category under `[reviewers]` in `lint.toml`, such as `required-in = ["heath"]`, no longer switches the reviewer rule off without a word. `khalid lint`, `khalid build` and publishing stop, name the line, and list the site's real categories. A category counts when `categories.toml` lists it or a post uses it.

Fixed
A site's own words in `theme/strings/<lang>.toml` now reach every reader-facing word Khalid writes outside its templates: the month names, page line and "Written by" and reading time labels in titles and X and Slack previews, the imprint link in the footer, the opening hours and day names on location cards, and the locations page.

Added
The same file may hold `[comments]`, `[newsletter]` (with `[newsletter.page]`) and `[notes]` tables, which word the page a refused comment shows, the newsletter's emails and sign-up pages, and the notes a static site shows where a form was. A key those pages never show is refused with a message naming the table.

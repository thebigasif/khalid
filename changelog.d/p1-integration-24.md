Changed
- The Look screen's options now include one-line text, such as Ledger's tagline, drawn as a single text field. The Admin API lists it as `line` with the most characters it takes.
- Templates share one set of words: "Latest posts" over the newest posts, "All posts" for the link to every post, and "All" as the first of a row of topics or categories.
- Every template keeps its links, stylesheets, newsletter form and share links under a blog's folder, such as `/blog/`, and the Look screen calls the Admin API there too. A test builds the demo in each template and fails on any address that leaves the folder.

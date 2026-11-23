Added
The Admin API lists of redirects, tags, pages, authors and snippets take `page`, `limit` and `filter`, and answer `meta.pagination` beside the list. A request without them still gets the whole list, so a big WordPress import no longer has to come back in one answer, and older callers keep working.

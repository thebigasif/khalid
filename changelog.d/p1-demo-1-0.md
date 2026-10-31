Fixed
- A glossary term's page now has a meta description, taken from the first sentences of its definition, so search results show the definition and Lighthouse's SEO check passes.
- `khalid serve` answers the browser's request for `/favicon.ico` with "no content" when the site has no icon, so browsers stop logging an error on every page.
- The demo check (`scripts/demo-check.sh`) finishes well inside CI's 30 minutes: the HTML validator runs again, the not-found pages skip Lighthouse, which cannot score a 404, and the browser checks load each page once and check four at a time.

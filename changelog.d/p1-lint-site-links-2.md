Added
`khalid lint` now checks ordinary markdown links to the site's own addresses, not only `[[wikilinks]]`. A link to an address beside your posts and pages that has no page gets the new `broken-internal-link` warning, and a link to an address you moved with a stored redirect gets `redirecting-link` with the new address to use. Links to the main site beside a blog in a folder are left alone.

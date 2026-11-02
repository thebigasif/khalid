Added
- **Every demo is also on serpdad.com.** `scripts/demos-bundle.sh` builds the showcase, the template gallery with its pictures, each template's demo and the six use case examples for https://serpdad.com/khalid/ and packs them as `khalid-demos.tar.gz`, which the Pages workflow publishes for the serpdad.com build to download.
- **A whole site can stay out of search.** `[seo.index] site = "noindex"` gives every page `noindex, follow`, over a post's own `robots`, and empties the sitemaps, for a staging copy or a sample blog. The demos use it; the template gallery stays indexable.

Changed
- The GitHub Pages copy at thebigasif.github.io/khalid/ names the matching serpdad.com address as each page's canonical one, and its header and hero link to https://serpdad.com/khalid/.
- Each card in the template gallery, Classic too, links to its demo at `/khalid/templates/<name>/`.

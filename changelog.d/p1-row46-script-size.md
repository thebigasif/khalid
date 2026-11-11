Changed
- The public site's scripts now have a size check. `reader.js` and `consent.js` must each stay under 5 KB gzipped, so readers never download a heavy script. Today `reader.js` is 4.0 KB and `consent.js` 1.6 KB. A change that makes one bigger fails the tests and says how to fix it.

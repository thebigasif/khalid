Added
- An owner guide to cookie consent, `docs/privacy/consent.md`: what Khalid does with no setup, the five categories, how to tag your own scripts, every privacy setting, proof of consent, static sites and outside consent tools.
- Every template's demo now shows the consent banner, and the layout check fails when any demo page loads anything from another site before a reader agrees, with scripts on or off. The served demo check does the same.

Fixed
- On a static site the consent banner now sits at the top of the page, after the skip link, as it does on a server, instead of a bar stuck to the bottom of the window that covered the text as a reader scrolled. It appears with the page, so nothing moves.
- A page framed from a domain allowed under `[media] embed_domains`, such as a map, now waits behind a card until the reader allows external media, as videos do. Before, it loaded for every reader.

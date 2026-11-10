Added
- Each post page `khalid serve` shows names its oEmbed card in its head (`<link rel="alternate" type="application/json+oembed">`), so another site given only the post's address finds the card. Pages `khalid build` writes leave it out, since a static host has nothing to answer it.

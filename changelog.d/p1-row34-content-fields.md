Added
- The Content API answers `fields` on every list: `/api/content/v1/posts/?fields=title,url` sends each post with only its title and address, so a front end fetches less. A name the resource does not have is a 400 that lists the names it does have. `include` is still refused, because posts already name their tags and authors.

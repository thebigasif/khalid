# Example: a publication in three languages

This is Bridgeword, a small publication about learning a language as an adult. It is written in English, German and Spanish. The publication and its editors are made up. The advice is real.

## Who it is for

A magazine, a company blog or a person who writes for readers in more than one language. Each reader finds the post in their own language, and search engines show each country the right version.

## What it shows

| Feature | Where to look |
|---|---|
| Three languages on one site | `site.toml`: `languages = ["en", "de", "es"]` |
| Each post in all three languages, linked as one post | the `translation:` key, the same in all three versions, such as `translation: exchange` |
| Search engines told which version is for whom | Khalid writes the `hreflang` links from those keys; nothing to set by hand |
| Addresses in each language's own words | `slug:`, such as `/blog/de/sprachtandem/` and `/blog/es/intercambio-de-idiomas/` |
| Menus and categories named in each language | `nav.primary` entries with `language = "..."`, and `categories.toml` |
| Authors and an about page in each language | `content/<language>/authors/` and `content/<language>/pages/` |
| Pull quotes | the `:::pullquote` block |
| Tables that compare words across languages | `false-friends` and its translations |

It uses the **Wire** template, made for publications: a lead story on the home page, a subtitle under each title and pull quotes in the text.

## Try it

```
khalid --site fixtures/examples/multilingual lint
khalid --site fixtures/examples/multilingual build --out public
```

The site needs the Wire template. Until Wire is built into your copy of Khalid, change `name = "wire"` to `name = "classic"` in `site.toml` to try it.

## Two things Khalid does not do yet

- **The home pages share one title.** Each language's home page takes the site's name and tagline, which are the same in every language. `khalid audit` warns about it.
- **Spanish pages that Khalid makes itself use English words,** such as the list of authors. Khalid has built-in interface words for English and German only so far.

Your posts and pages are not affected: they use the words you write.

## Make it yours

List your languages in `site.toml`. Write a post in one language, then copy it into each other language's folder, translate it and keep the same `translation:` key. Translate the slug too, so the address reads well in that language.

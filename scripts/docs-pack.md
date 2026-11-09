# The Khalid docs pack

`khalid-docs.tar.gz` holds Khalid's user docs for <https://serpdad.com/khalid/docs/>. `scripts/docs-bundle.sh` in the Khalid repository makes it, and the website build reads it. The docs stay written in one place, the Khalid repository.

## What is in it

```text
README.md          this file
docs.json          the doc list, in sidebar order
pages/<slug>.md    one doc, such as pages/getting-started.md or pages/deploy/cloudflare.md
pictures/<path>    each picture a doc shows, served at /khalid/docs/<path>
```

## The doc list

`docs.json` has the base address and one entry per doc, in the order the sidebar shows them: each top doc by its order, then the guides under it by theirs.

```json
{
  "base": "/khalid/docs/",
  "docs": [
    {"slug": "deploy", "title": "Running Khalid on a server", "order": 20, "parent": null, "url": "/khalid/docs/deploy/", "file": "pages/deploy.md", "pictures": []},
    {"slug": "deploy/cloudflare", "title": "A free Khalid blog on Cloudflare", "order": 10, "parent": "deploy", "url": "/khalid/docs/deploy/cloudflare/", "file": "pages/deploy/cloudflare.md", "pictures": []}
  ]
}
```

- `slug`: the doc's path under `docs/` without `.md`. A guide's slug starts with its parent's.
- `title`: the page title. It matches the doc's one `#` heading.
- `order`: the place among the docs that share a parent, smallest first.
- `parent`: the slug of the doc a guide sits under, or `null` for a top doc.
- `url`: where the page is served. Each doc is in the sitemap and `llms-full.txt` at this address.
- `file`: the page in the pack.
- `pictures`: the files in the pack that the page shows.

## A page

Each page is Markdown with front matter between two `---` lines:

```yaml
title: "A free Khalid blog on Cloudflare"
order: 10
slug: deploy/cloudflare
parent: deploy
source: docs/deploy/cloudflare.md
```

`source` is the file in the Khalid repository, for an "Edit this page" link. The body opens with its one `#` heading, so the page needs no other `h1`. Code fences may sit indented inside lists, and tables use the GitHub form.

## Links and pictures

The pack rewrites every link outside code, so none needs the repository:

- a link to another doc is its address, such as `/khalid/docs/routes/` or `/khalid/docs/templates/#your-call-to-action`;
- a picture is `/khalid/docs/<path>`, and the file is `pictures/<path>`;
- a link to another file of the repository is its GitHub address;
- full addresses and `#` links on the same page stay as they are.

A link or picture that resolves to nothing stops the pack, so a pack that exists has none.

## How a doc joins

A Markdown file under `docs/` joins the pack when its front matter has `title: "<words>"` and `order: <whole number>`. A guide in `docs/<parent>/` needs `docs/<parent>.md` packed too.

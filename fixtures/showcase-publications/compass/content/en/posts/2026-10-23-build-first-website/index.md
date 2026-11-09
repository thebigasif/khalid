---
title: "Your first website, step 2: build the pages with HTML and CSS"
slug: build-first-website
date: 2026-10-23
author: ruben-achterberg
reviewed_by: desmond-vale
category: first-project
description: Build your first website from your paper plan. Make a shared header and footer, link the pages, add one stylesheet and commit after each page.
focus_keyword: build
tags: [projects, html, css]
series:
  name: Your first website
  order: 2
image:
  src: lead.jpg
  alt: A pair of thin-rimmed glasses and two pens resting on an open ring binder of handwritten notes on a wooden desk
---

To build your first website from your plan, make the home page first with a header, a main section and a footer, link one shared stylesheet, and then copy that page as the starting point for every other page. Commit each page with Git as soon as it works, so you always have a version to return to.

This is step 2 of 4. You need the plan from [step 1](/blog/plan-first-website/) and a folder with Git set up, as in [Git for beginners](/blog/git-first-steps/).

::toc

:::callout{kind=summary}
- **Build the home page completely first.** It becomes the template for the rest.
- **Every page shares a header, a menu and a footer.**
- **One stylesheet for the whole site** keeps every page looking the same.
- **Commit after each page.**
:::

## Build the home page

Start with this structure. It uses a few HTML tags you have not met yet, `<header>`, `<main>` and `<footer>`, which label the big parts of a page so browsers and screen readers understand its layout. Tags that say what something is are called [semantic HTML](https://developer.mozilla.org/en-US/docs/Glossary/Semantics).

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Kestrel Bay footpaths</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <header>
      <a class="site-name" href="index.html">Kestrel Bay footpaths</a>
      <nav>
        <a href="index.html">Walks</a>
        <a href="about.html">About</a>
      </nav>
    </header>

    <main>
      <h1>Pick a walk for the weekend</h1>
      <p>Four footpaths around Kestrel Bay, from a flat hour by the harbour to a half day on the cliffs.</p>

      <ul class="walks">
        <li><a href="cliff-path.html">The cliff path</a>: 9 km, steep in places</li>
        <li><a href="harbour-loop.html">The harbour loop</a>: 4 km, flat</li>
      </ul>
    </main>

    <footer>
      <p>Made by Ines Duarte. Suggest a walk on the <a href="about.html">about page</a>.</p>
    </footer>
  </body>
</html>
```

:::callout{kind=note}
The `viewport` line in the `<head>` tells phones to show the page at their real width instead of shrinking a desktop page. Every page needs it. Step 3 explains why.
:::

## Build the site, step by step

::::howto{name="Build your first website from a plan" time="2:00:00"}
:::step{name="Write the home page"}
Type the structure above into `index.html`, with your own words from your plan. Open it in the browser.
:::
:::step{name="Add the shared stylesheet"}
Write `style.css` with your body font, column width and heading colour, as in [the CSS guide](/blog/css-basics/). Lay out the header with Flexbox, as in [the Flexbox guide](/blog/flexbox-layout/).
:::
:::step{name="Commit the home page"}
In the terminal: `git add .` then `git commit -m "Build the home page"`.
:::
:::step{name="Copy it for the next page"}
Run `cp index.html about.html`. Open `about.html`, keep the header and footer, and replace only what is inside `<main>`.
:::
:::step{name="Check every link"}
Click every link on every page. A link that goes nowhere shows an error page in the browser: fix the file name in its `href`.
:::
:::step{name="Commit again"}
`git add .` and `git commit -m "Add the about page"`. Repeat steps 4 to 6 for each page in your plan.
:::
::::

:::figure{src="notebook.jpg" alt="An open exercise book with blank ruled pages, one corner folded over, lying on a dark brown cloth"}
Each new page starts from a copy of the home page, with a blank space in the middle for its own content.
:::

## The stylesheet for the whole site

This is the starting stylesheet our project groups use. Change the colours and font to suit your site.

```css
body {
  font-family: system-ui, sans-serif;
  line-height: 1.6;
  color: #1a1a1a;
  max-width: 42rem;
  margin: 0 auto;
  padding: 0 16px;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid #ccc;
}

nav {
  display: flex;
  gap: 16px;
}

.site-name {
  font-weight: bold;
  text-decoration: none;
}

footer {
  margin-top: 48px;
  padding: 16px 0;
  border-top: 1px solid #ccc;
  font-size: 0.9rem;
}

img {
  max-width: 100%;
  height: auto;
}
```

## A checklist before you move on

| Check | How |
|---|---|
| Every page has a `<title>` that names it | Look at the browser tab on each page |
| Every picture has `alt` text | Search your HTML for `<img` and read each one |
| Every link works | Click them all, on every page |
| Every page shares the same header and footer | Click between pages: nothing should jump |
| Your work is committed | `git status` says "nothing to commit" |

:::callout{kind=warning}
When you change the header on one page, change it on every page. With plain HTML, each page has its own copy. A forgotten page with an old menu is the most common bug in our groups' first sites.
:::

:::callout{kind=read}
Next in the series: [step 3, make it work on phones](/blog/responsive-website/). More than half your readers will open your site on one.
:::

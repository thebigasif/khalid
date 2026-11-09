---
title: "Style a page with CSS: colours, fonts and spacing"
slug: css-basics
date: 2026-08-04
author: desmond-vale
reviewed_by: ruben-achterberg
category: the-web
description: Style a page with CSS in a separate stylesheet. Link it, choose colours and fonts, add spacing with margin and padding, and read the box model.
focus_keyword: CSS
tags: [beginners, css]
image:
  src: lead.jpg
  alt: A row of coloured pencils laid side by side on dark cloth, their round painted ends running from red through yellow, green and blue to black
---

To style a page with CSS, write your rules in a separate file called `style.css`, link it from the `<head>` of your HTML page, and give each rule a selector, which says what to style, and declarations, which say how. CSS stands for Cascading Style Sheets, and it controls everything about how a page looks: colours, fonts, sizes, spacing and layout.

This guide picks up the allotment page from [your first HTML page](/blog/first-html-page/) and makes it pleasant to read in about forty-five minutes.

::toc

:::callout{kind=summary}
- **Keep CSS in its own file** and link it with one line in the `<head>`.
- **A rule is a selector and some declarations:** `h1 { color: darkgreen; }`.
- **Every element is a box** with content, padding, a border and a margin.
- **Change one value at a time** and reload, so you see what each one does.
:::

## Link a stylesheet

Make a new file called `style.css` in the same folder as `index.html`. Then add this line inside the `<head>` of your page, just after the `<title>`:

```html
<link rel="stylesheet" href="style.css">
```

Now put one rule in `style.css` to check that the link works:

```css
body {
  background-color: lightyellow;
}
```

Save both files and reload the page. If the background turns pale yellow, the link works. If not, check that the file names match exactly.

## How a CSS rule is built

```css
h1 {
  color: darkgreen;
  font-size: 40px;
}
```

| Part | In the example | What it does |
|---|---|---|
| Selector | `h1` | Chooses which elements the rule applies to |
| Curly brackets | `{ }` | Hold the rule's declarations |
| Property | `color` | The thing to change |
| Value | `darkgreen` | What to change it to |
| Semicolon | `;` | Ends each declaration. Forgetting it is the most common CSS mistake. |

## Style the allotment page

::::howto{name="Style a page with CSS" time="0:45:00"}
:::step{name="Set a readable font and size"}
In the `body` rule, add `font-family: Georgia, serif;`, `font-size: 18px;` and `line-height: 1.6;`. The list of fonts is a fallback: if the reader's computer lacks the first, it uses the next.
:::
:::step{name="Limit the line length"}
Add `max-width: 40rem;` and `margin: 0 auto;` to the `body` rule. Lines of about 70 letters are easiest to read, and `auto` margins centre the column.
:::
:::step{name="Colour the headings"}
Add a rule for `h1, h2` with a `color`. A comma in a selector applies one rule to several elements.
:::
:::step{name="Space the list"}
Add a rule for `li` with `margin-bottom: 8px;` so the list items breathe.
:::
:::step{name="Round the picture"}
Add a rule for `img` with `max-width: 100%;` and `border-radius: 8px;`. The first line stops a large photo spilling out of the column.
:::
::::

Your finished `style.css`:

```css
body {
  background-color: lightyellow;
  font-family: Georgia, serif;
  font-size: 18px;
  line-height: 1.6;
  max-width: 40rem;
  margin: 0 auto;
  padding: 0 16px;
}

h1,
h2 {
  color: darkgreen;
}

li {
  margin-bottom: 8px;
}

img {
  max-width: 100%;
  border-radius: 8px;
}
```

:::figure{src="spiral-binding.jpg" alt="Close view of the steel spiral binding of an open squared notebook, the grid pages fanning out on either side"}
Every element sits on an invisible grid of boxes, like the squares of a notebook. CSS decides how big each box is and how far apart they sit.
:::

## Every element is a box

The one idea that makes CSS click is the box model, which the [Mozilla developer site explains with diagrams](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_box_model/Introduction_to_the_CSS_box_model). The browser draws every element, a heading, a paragraph, a picture, as a rectangle with four layers, from the inside out:

1. **Content:** the text or picture itself.
2. **Padding:** space inside the box, between the content and the border.
3. **Border:** a line around the box. Invisible unless you give it a width and colour.
4. **Margin:** space outside the box, pushing other boxes away.

Try it. Add this rule, reload, then change each number and watch what moves:

```css
p {
  padding: 12px;
  border: 2px solid darkgreen;
  margin: 24px 0;
}
```

:::callout{kind=tip}
Open the developer tools, click a paragraph in the **Elements** or **Inspector** panel, and look at the coloured diagram of nested rectangles. It shows the exact padding, border and margin of that element. The [developer tools guide](/blog/browser-developer-tools/) explains the rest of that panel.
:::

## Colours you can use

CSS understands colours written in several ways. All four of these are the same dark green:

| Written as | Example |
|---|---|
| A name | `darkgreen` |
| Hex code | `#006400` |
| RGB | `rgb(0 100 0)` |
| HSL | `hsl(120 100% 20%)` |

Names are easiest to start with. Hex codes are what designers usually hand you. Whatever you choose, check that your text colour is dark enough against your background to read comfortably.

:::callout{kind=warning}
Pale grey text on white looks elegant and is hard to read for many people, especially on a phone in sunlight. Keep body text close to black.
:::

## What to try next

Change the font, the colours and the spacing until the page feels like yours. Then move on to the next stop, [lay out a page with Flexbox](/blog/flexbox-layout/), where boxes stop stacking and start sitting side by side.

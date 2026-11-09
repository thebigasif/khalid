---
title: "Lay out a page with Flexbox"
slug: flexbox-layout
date: 2026-08-18
author: desmond-vale
reviewed_by: imke-sato
category: the-web
description: Lay out a page with Flexbox. Put boxes side by side with one line of CSS, space and align them, and let them wrap on a narrow screen.
focus_keyword: Flexbox
tags: [css, layout]
image:
  src: lead.jpg
  alt: A wooden printer's type case on a slanted stand, its many small compartments arranged in a grid against a deep blue wall
---

To lay out a page with Flexbox, give a parent element `display: flex;` in your CSS, and its children line up side by side in a row. Two more properties, `justify-content` and `align-items`, decide how they are spaced and lined up, and `flex-wrap: wrap;` lets them drop onto a new line when the screen is too narrow.

Flexbox is the layout tool we teach first, because it solves the most common beginner layouts, a menu in a row, cards side by side, a footer with two halves, with very little code.

::toc

:::callout{kind=summary}
- **Flexbox works on a parent and its children.** You style the parent; the children move.
- **`display: flex` puts children in a row.**
- **`justify-content` spaces them along the row; `align-items` lines them up across it.**
- **`flex-wrap: wrap` and `gap`** make a row of cards that works on a phone.
:::

## Make a row in one line

Start with three cards in your HTML, inside one parent:

```html
<div class="cards">
  <div class="card">Beans</div>
  <div class="card">Beetroot</div>
  <div class="card">Potatoes</div>
</div>
```

Without any CSS, the three cards stack one under another, because a `<div>` takes the full width by default. Now add this to `style.css`:

```css
.cards {
  display: flex;
}

.card {
  padding: 16px;
  border: 1px solid darkgreen;
}
```

Reload. The three cards sit side by side. A dot before a name, as in `.cards`, selects every element with that `class`.

## The two directions

Flexbox thinks in two directions. The **main axis** runs along the row, left to right. The **cross axis** runs across it, top to bottom. Two properties control them:

| Property | Direction | Common values | What they do |
|---|---|---|---|
| `justify-content` | Along the row | `flex-start`, `center`, `space-between` | Push the children to the start, gather them in the middle, or spread them out with equal gaps |
| `align-items` | Across the row | `stretch`, `center`, `flex-start` | Make them all the same height, centre them vertically, or line up their tops |
| `flex-direction` | Turns the row | `row`, `column` | `column` turns the main axis downwards |
| `flex-wrap` | Allows new lines | `nowrap`, `wrap` | `wrap` lets children move to a new line when there is no room |
| `gap` | Between children | `16px`, `1rem` | Space between children, without margins on each one |

:::callout{kind=tip}
Most layout problems in Flexbox come from styling the wrong element. If nothing moves, check that `display: flex` is on the parent, not on the cards.
:::

## Build a menu and a row of cards with Flexbox

::::howto{name="Lay out a page with Flexbox" time="0:40:00"}
:::step{name="Make a menu bar"}
Put three links inside a `<nav>`. Give the `nav` a rule with `display: flex;` and `gap: 24px;`. The links sit in a row with even gaps.
:::
:::step{name="Push the logo left and the menu right"}
Wrap your site name and the `nav` in a `<header>`. Give the `header` `display: flex;` and `justify-content: space-between;`. The name moves to the left edge and the menu to the right.
:::
:::step{name="Centre them vertically"}
Add `align-items: center;` to the `header`, so a large name and small links share one centre line.
:::
:::step{name="Let the cards share the row"}
Give each `.card` the rule `flex: 1 1 200px;`. Each card starts at 200 pixels wide, then grows or shrinks to fill the row evenly.
:::
:::step{name="Let them wrap"}
Add `flex-wrap: wrap;` and `gap: 16px;` to `.cards`. Make the browser window narrow: the cards drop onto new lines instead of squeezing.
:::
::::

The finished CSS for the header and cards:

```css
header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

nav {
  display: flex;
  gap: 24px;
}

.cards {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.card {
  flex: 1 1 200px;
  padding: 16px;
  border: 1px solid darkgreen;
  border-radius: 8px;
}
```

## What `flex: 1 1 200px` means

The `flex` shorthand holds three numbers. They look odd at first, so here they are one by one.

| Part | Value here | Meaning |
|---|---|---|
| Grow | `1` | Take a share of any spare room in the row |
| Shrink | `1` | Give up room when the row is too small |
| Basis | `200px` | The width to start from before growing or shrinking |

Three cards with the same three numbers end up the same width. Give one card `flex: 2 1 200px;` and it takes twice as much of the spare room as the others.

:::callout{kind=note}
CSS has a second layout tool called Grid, which is better when you need rows and columns that line up in both directions, such as a photo gallery. Flexbox handles one direction at a time. Learn Flexbox first; Grid will make more sense afterwards. The [basic concepts of Flexbox on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_flexible_box_layout/Basic_concepts_of_flexbox) cover every property in this guide and a few more.
:::

## A practice exercise

Build a footer with your name on the left and three links on the right, using only what this guide covers. Then make the window narrow and check that nothing overlaps. If something does, add `flex-wrap: wrap;` to the parent.

You now have a page that looks designed. The last stop in "The web" makes it react: [add a little JavaScript](/blog/first-javascript/).

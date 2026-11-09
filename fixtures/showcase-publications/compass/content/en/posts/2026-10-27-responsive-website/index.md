---
title: "Your first website, step 3: make it responsive for phones"
slug: responsive-website
date: 2026-10-27
author: ruben-achterberg
reviewed_by: desmond-vale
category: first-project
description: Make your first website responsive so it works on phones. Add the viewport line, let pictures shrink, wrap your rows and add one media query for small screens.
focus_keyword: responsive
tags: [projects, css, layout]
series:
  name: Your first website
  order: 3
image:
  src: lead.jpg
  alt: A phone in a black leather folding case lying on a polished wooden table, seen close up at table height
---

To make your website responsive, so it works on phones, add the viewport line to every page, let pictures shrink with `max-width: 100%`, let rows of boxes wrap with Flexbox, and add one media query that adjusts the layout below a set width. Then test at a phone's width in your browser's developer tools and on a real phone.

This is step 3 of 4. You need the pages you built in [step 2](/blog/build-first-website/). A page that works on a narrow screen and grows to fill a wide one is called responsive.

::toc

:::callout{kind=summary}
- **The viewport line** stops phones shrinking your page to a tiny desktop copy.
- **Flexible widths** let text and pictures fit any screen.
- **One media query** is usually enough for a small site.
- **Test on a real phone** before you call it done.
:::

## Why phones need extra care

A phone screen is about 360 to 430 pixels wide, a third of a laptop's. A page designed only on a laptop can overflow sideways on a phone, show tiny text or squeeze a row of cards into thin strips. Most of your readers will arrive on a phone, so this step matters more than any colour you choose.

:::callout{kind=stat}
**6 of 10** visits to the sites our autumn 2026 project groups put online came from phones in their first month.
:::

## Make the site responsive, step by step

::::howto{name="Make your first website work on phones" time="1:00:00"}
:::step{name="Check the viewport line"}
Every page needs `<meta name="viewport" content="width=device-width, initial-scale=1">` in its `<head>`. The [viewport page on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/HTML/Viewport_meta_tag) explains each part. Without it, phones pretend to be about 980 pixels wide and shrink everything.
:::
:::step{name="Let pictures shrink"}
Make sure your stylesheet has `img { max-width: 100%; height: auto; }`. A picture can then never be wider than its column.
:::
:::step{name="Let rows wrap"}
Any element with `display: flex` that holds several boxes also needs `flex-wrap: wrap;`, so the boxes move to a new line instead of squeezing.
:::
:::step{name="Add a media query"}
Add the media query below to the end of `style.css`. It changes the header only on narrow screens.
:::
:::step{name="Test at phone width"}
Open the developer tools, switch on device mode and set the width to 360. Click every page and scroll to the bottom of each.
:::
:::step{name="Test on a real phone"}
Copy your site to a real phone, or wait until it is online in step 4, and read every page with your thumb, not a mouse.
:::
::::

## Your first media query

A media query is the main tool of responsive design: a CSS block that applies only when a condition is true, such as the screen being narrower than a width you choose. This one stacks the site name above the menu on small screens:

```css
@media (max-width: 600px) {
  header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  h1 {
    font-size: 1.75rem;
  }
}
```

Read it as: "When the screen is 600 pixels wide or less, stack the header's contents in a column and make the main heading a little smaller." Above 600 pixels, the browser ignores the whole block.

:::callout{kind=tip}
Choose the width for your media query by looking, not by device lists. Drag the device mode width slowly narrower and note the width where your layout starts to look cramped. Put the media query a little above that number.
:::

## Common phone problems and their fixes

| What you see on a phone | Usual cause | Fix |
|---|---|---|
| The page scrolls sideways | Something is wider than the screen, often a picture or a long link | `max-width: 100%` on images; `overflow-wrap: anywhere;` on long links |
| Everything is tiny | The viewport line is missing | Add it to that page's `<head>` |
| Cards are thin strips | A flex row cannot wrap | Add `flex-wrap: wrap;` to the parent |
| Links are hard to tap | They are small and close together | Add `padding: 8px;` to menu links |
| Text runs to the very edge | No side padding | Add `padding: 0 16px;` to `body` |

## Find what overflows

When a page scrolls sideways and you cannot see why, paste this rule at the end of your stylesheet for a minute:

```css
* {
  outline: 1px solid red;
}
```

Every box on the page gets a red outline. Scroll right and look for the outline that sticks out past the others. That is your culprit. Remove the rule when you have found it.

:::callout{kind=warning}
Do not fix sideways scrolling by adding `overflow-x: hidden` to the `body`. It hides the symptom, and part of your content becomes impossible to reach on a phone.
:::

## Larger text helps everyone

Some phone readers set their text size larger in their phone's settings. Use `rem` for font sizes, as in the media query above, rather than fixed pixels, and the page grows with their setting. Then check that a page still reads well with the text one size larger.

When every page passes the tests, your site is responsive. Commit your work: `git add .` and `git commit -m "Make the site work on phones"`.

:::callout{kind=read}
Next in the series: [step 4, put your website online](/blog/publish-website/).
:::

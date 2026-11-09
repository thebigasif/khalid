---
title: "Use your browser's developer tools to see how a page works"
slug: browser-developer-tools
date: 2026-10-13
author: imke-sato
reviewed_by: desmond-vale
category: tools
description: Use your browser's developer tools to inspect any element, try CSS changes live, read console errors and spot missing files in the Network panel.
focus_keyword: developer tools
tags: [browsers, css, tools]
image:
  src: lead.jpg
  alt: A white desk seen from above with an open blank notebook, a yellow pencil, reading glasses, a phone and the corner of a laptop
---

Your browser's developer tools are a set of panels built into every desktop browser that show how a page is made: its HTML, the CSS on each element, any errors and every file it loaded. Open them with F12, or Cmd + Option + I on a Mac, and you can inspect and even change any page you are looking at, including your own.

Nothing you change in the developer tools is saved. That makes them the safest place to experiment: try a colour, a margin or a layout, see the result at once, then copy the change you like into your own CSS file.

::toc

:::callout{kind=summary}
- **Elements** shows the page's HTML and the CSS on each part.
- **Console** shows errors and lets you run JavaScript.
- **Network** lists every file the page asked for, and which ones failed.
- **Changes are temporary.** Reload and they are gone.
:::

## Open the developer tools

| Browser | Windows and Linux | Mac |
|---|---|---|
| Chrome, Edge and Brave | F12 or Ctrl + Shift + I | Cmd + Option + I |
| Firefox | F12 or Ctrl + Shift + I | Cmd + Option + I |
| Safari | Not available | Turn on "Show features for web developers" in Settings, then Cmd + Option + I |

The [Mozilla developer site has a longer tour of the developer tools](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Tools_and_setup/What_are_browser_developer_tools) in each browser. You can also right-click any part of a page and choose **Inspect**. The tools open with that part already selected, which is usually what you want.

## The four panels you will use

| Panel | Chrome name | Firefox name | Use it to |
|---|---|---|---|
| Elements | Elements | Inspector | See the HTML and the CSS rules on any element |
| Console | Console | Console | Read errors and run JavaScript |
| Network | Network | Network | See every file the page loaded, and spot the ones that failed |
| Device mode | Toggle device toolbar | Responsive design mode | Preview the page at a phone's width |

## Inspect and change your own page

Open your allotment page from the earlier guides and follow these steps.

::::howto{name="Inspect and change a page with the developer tools" time="0:25:00"}
:::step{name="Inspect the main heading"}
Right-click your `h1` and choose **Inspect**. The Elements panel opens with `<h1>` highlighted, and the Styles pane beside it lists every CSS rule that applies.
:::
:::step{name="Change a colour live"}
In the Styles pane, click the value of `color` and type a new one, such as `teal`. The heading changes at once. Press Enter to keep it until you reload.
:::
:::step{name="Turn a rule off"}
Hover over a declaration and untick the box beside it. The page shows what it looks like without that line. Tick it again to bring it back.
:::
:::step{name="Read the box"}
Scroll the Styles pane to the coloured box diagram. It shows the element's content, padding, border and margin in pixels. Hover over each layer to see it outlined on the page.
:::
:::step{name="Copy what you like"}
When a change looks right, type the same line into your own `style.css`, save and reload. Now it is permanent.
:::
::::

:::callout{kind=tip}
A rule with a line through it has been overruled by another rule further down or with a more specific selector. When your CSS "does nothing", look for your line with a strike through it. The rule above it in the pane is the one that won.
:::

## Find a missing picture with the Network panel

When a picture does not appear, the Network panel tells you why in seconds.

1. Open the developer tools and click **Network**.
2. Reload the page. A row appears for each file.
3. Look for a row in red, or with `404` in the **Status** column.
4. Read its name. Compare it, letter by letter, with the file name in your folder.

Nine times out of ten the difference is a capital letter, a missing folder name or a `.jpeg` that should be `.jpg`.

:::callout{kind=note}
If the Network panel is empty, it was not open when the page loaded. Leave it open and reload.
:::

## Preview your page on a phone

Click the device mode button, which looks like a small phone and tablet, near the top left of the tools. The page shrinks to a phone's width, and a bar at the top lets you choose a device size or drag the width yourself. Drag it slowly from wide to narrow and watch for the moment something overlaps or runs off the edge. That is where your layout needs work, which is exactly what step 3 of the first website series covers: [make your website work on phones](/blog/responsive-website/).

## Look inside other people's pages

The developer tools work on any page, not just yours. When you see a layout you like on another site, inspect it. Find the element, read its CSS, and notice which properties do the work. Most of what working developers know, they learned by inspecting pages that did something they wanted to copy.

:::callout{kind=warning}
Changing another site in the developer tools changes only what your own browser shows, and only until you reload. It does not affect the real site. Never paste code into the console because a message or a stranger asks you to: that is a common way to steal accounts.
:::

That completes the tools. You now have everything you need for the last stop on the path: [plan your first website](/blog/plan-first-website/).

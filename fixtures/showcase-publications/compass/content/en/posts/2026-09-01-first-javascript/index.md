---
title: "Add a little JavaScript to a web page"
slug: first-javascript
date: 2026-09-01
author: desmond-vale
reviewed_by: hanna-ostrow
category: the-web
description: Add a little JavaScript to a web page. Link a script, find an element, react to a click and change the page, with every line explained.
focus_keyword: JavaScript
tags: [javascript, beginners]
image:
  src: lead.jpg
  alt: An open silver laptop, a black coffee, two pencils and a small pad of paper on a pale wooden table seen from above
---

To add JavaScript to a web page, write your code in a file called `script.js` and link it from your HTML with a `<script>` tag at the end of the `<body>`. Your script can then find an element on the page, wait for the reader to do something, such as click a button, and change the page in response.

You already wrote a few lines of JavaScript in the console in [your first program](/blog/first-program/). This guide puts JavaScript into a real page. By the end, a button on your allotment page will show and hide a list.

::toc

:::callout{kind=summary}
- **Keep JavaScript in its own file** and link it just before `</body>`.
- **`document.querySelector`** finds an element on the page.
- **`addEventListener`** waits for something to happen, such as a click.
- **Change a class, not the style.** Let CSS decide how things look.
:::

## Link a script

Make a file called `script.js` in your site folder. Then, in `index.html`, add this line just before the closing `</body>` tag:

```html
<script src="script.js"></script>
```

Put one line in `script.js` to check it works:

```javascript
console.log("script.js has loaded");
```

Reload the page and open the console. If you see the message, the link works.

:::callout{kind=note}
The script goes at the end of the body because the browser reads the page from top to bottom. A script at the very top would run before the elements it looks for exist. Placing it last means everything is ready.
:::

## Show and hide a list

Here is the HTML to add to your page. The button and the list each have an `id`, a name that is unique on the page:

```html
<button id="toggle-tips" type="button">Show growing tips</button>

<ul id="tips" class="hidden">
  <li>Water beans at the roots, not the leaves.</li>
  <li>Pull beetroot when it is the size of a golf ball.</li>
</ul>
```

Add one rule to `style.css`:

```css
.hidden {
  display: none;
}
```

And replace the contents of `script.js` with this:

```javascript
const button = document.querySelector("#toggle-tips");
const tips = document.querySelector("#tips");

button.addEventListener("click", function () {
  tips.classList.toggle("hidden");

  if (tips.classList.contains("hidden")) {
    button.textContent = "Show growing tips";
  } else {
    button.textContent = "Hide growing tips";
  }
});
```

Reload and click the button. The list appears, and the button's words change. Click again and it hides.

## What each line does

::::howto{name="Make a button show and hide a list" time="0:30:00"}
:::step{name="Find the elements"}
`document.querySelector("#toggle-tips")` searches the page for the element whose `id` is `toggle-tips`. The `#` means "id". We keep it in a `const`, a variable whose value will not be replaced.
:::
:::step{name="Wait for a click"}
`button.addEventListener("click", function () { ... })` tells the browser: when this button is clicked, run the code between the curly brackets. The [addEventListener page on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener) lists every other event you can listen for.
:::
:::step{name="Toggle a class"}
`tips.classList.toggle("hidden")` adds the class `hidden` if it is missing, and removes it if it is there. CSS does the hiding.
:::
:::step{name="Update the button"}
The `if` checks whether the list is now hidden and sets the button's words to match, so the button always says what it will do next.
:::
::::

## Words worth knowing

| Word | Meaning |
|---|---|
| `document` | The whole page, as JavaScript sees it |
| Element | One tag on the page, such as a button or a list |
| Event | Something that happens: a click, a key press, the page finishing loading |
| Function | A named or unnamed block of code that runs when called |
| `const` and `let` | Make a variable. `const` cannot be given a new value; `let` can. |

:::callout{kind=tip}
If the button does nothing, open the console. A message such as `Cannot read properties of null (reading 'addEventListener')` means `querySelector` found nothing: the `id` in your HTML and in your script do not match. Our guide to [reading error messages](/blog/read-error-messages/) explains the rest.
:::

## Why we change a class instead of a style

JavaScript can change styles directly, for example with `tips.style.display = "none"`. We teach the class approach instead. It keeps the decision about how things look in your CSS, where you will look for it later, and it means one line of JavaScript can change many styles at once.

## How much JavaScript you need for now

For a first website, very little. A menu that opens on a phone, a button that shows more text, a form that checks an email address is filled in: that is plenty. JavaScript is a whole language, and you can spend years on it later. For now you have completed "The web". The next stop on the path is the tools: start with [the terminal](/blog/terminal-basics/).

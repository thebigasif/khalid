---
title: "Write your first HTML page"
slug: first-html-page
date: 2026-07-21
updated: 2026-10-02
updated_note: We added the lang attribute to the starting file and a note on saving files with the right ending on Windows.
author: desmond-vale
reviewed_by: hanna-ostrow
category: the-web
description: Write your first HTML page in a plain text file, open it in your browser and add headings, paragraphs, a list, a link and a picture.
focus_keyword: first HTML page
tags: [beginners, html]
image:
  src: lead.jpg
  alt: Two hands typing on a silver laptop keyboard at a dark desk, with a green plant blurred behind the screen
---

To write your first HTML page, open a plain text editor, type a short skeleton of tags, save the file as `index.html` and double-click it to open it in your browser. HTML is not a programming language with logic in it; it is a way of labelling text so the browser knows which part is a heading, which is a paragraph and which is a link.

You will finish this guide with your first HTML page on your own computer: a title, a short introduction, a list and a link. It takes about forty minutes.

::toc

:::callout{kind=summary}
- **HTML is text with labels**, called tags, around each part of the page.
- **Most tags come in pairs:** `<p>` opens a paragraph and `</p>` closes it.
- **Save the file as `index.html`**, then open it in any browser.
- **Indent nested tags** so you can see which tag sits inside which.
:::

## What you need

- A plain text editor. Notepad on Windows and TextEdit on a Mac will do for today; the [code editor guide](/blog/code-editor-setup/) shows you a better one later.
- A folder for your work. Make one called `my-first-site` on your desktop.
- Any web browser.

:::callout{kind=warning}
TextEdit on a Mac saves formatted text by default, which breaks HTML. Before you start, open TextEdit's settings and choose **Plain text** under **Format**. On Windows, make sure the file ends in `.html` and not `.html.txt`: turn on **File name extensions** in File Explorer's **View** menu to check.
:::

## The skeleton of your first HTML page

Every HTML page, including your first HTML page, has the same outer frame. Type this into your editor exactly:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>My allotment</title>
  </head>
  <body>
    <h1>My allotment</h1>
    <p>I grow vegetables on plot 14 at the Kestrel Bay allotments.</p>
  </body>
</html>
```

Each part has a job:

| Part | What it does |
|---|---|
| `<!DOCTYPE html>` | Tells the browser this is a modern HTML page |
| `<html lang="en">` | Wraps the whole page and says it is written in English |
| `<head>` | Holds information about the page that is not shown in the page itself |
| `<meta charset="utf-8">` | Lets the page show any letter or symbol correctly |
| `<title>` | The text on the browser tab and in search results |
| `<body>` | Everything the reader sees |
| `<h1>` | The main heading. Use one per page. |
| `<p>` | A paragraph |

## Build your first HTML page, step by step

::::howto{name="Write your first HTML page" time="0:40:00"}
:::step{name="Save the skeleton"}
Save the file as `index.html` in your `my-first-site` folder. Double-click it. Your browser opens it and shows a heading and a sentence.
:::
:::step{name="Add a second heading and a list"}
Under the paragraph, add `<h2>What I grow</h2>`, then a list: `<ul>` with one `<li>` for each vegetable, then `</ul>`.
:::
:::step{name="Add a link"}
Add a paragraph with a link: `<a href="https://www.example.com/">our allotment society</a>`. The `href` says where the link goes.
:::
:::step{name="Add a picture"}
Copy a photo into the folder, name it `plot.jpg`, and add `<img src="plot.jpg" alt="My plot in June, with beans on canes">`. The `alt` text describes the picture for people who cannot see it.
:::
:::step{name="Save and reload"}
Save the file, switch to the browser and press F5 or Cmd + R to reload. Every change you save shows up after a reload.
:::
::::

Here is the body after all five steps:

```html
<body>
  <h1>My allotment</h1>
  <p>I grow vegetables on plot 14 at the Kestrel Bay allotments.</p>

  <h2>What I grow</h2>
  <ul>
    <li>Runner beans</li>
    <li>Beetroot</li>
    <li>Two kinds of potato</li>
  </ul>

  <p>I am a member of <a href="https://www.example.com/">our allotment society</a>.</p>

  <img src="plot.jpg" alt="My plot in June, with beans on canes">
</body>
```

:::callout{kind=tip}
If the picture does not show, the file name in `src` and the real file name do not match. Check capital letters too: on most web servers `Plot.jpg` and `plot.jpg` are different files.
:::

## Why the indentation matters

The browser ignores the spaces at the start of each line. You do not. Indenting each tag inside its parent shows you the shape of the page at a glance: the `<li>` items sit inside the `<ul>`, which sits inside the `<body>`. When a tag is not closed, the indentation looks wrong, and you spot the mistake before the browser does.

## Tags you will use most

| Tag | Use it for |
|---|---|
| `<h1>` to `<h6>` | Headings, from the most important to the least |
| `<p>` | Paragraphs |
| `<a href="...">` | Links |
| `<img src="..." alt="...">` | Pictures. It has no closing tag. |
| `<ul>` and `<li>` | A bulleted list |
| `<ol>` and `<li>` | A numbered list |
| `<strong>` | Words that matter most in a sentence |

With these seven you can write most of a simple website. The [HTML element reference on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/HTML/Element) lists every other tag when you need one. Your first HTML page looks plain for now, in black serif text on white. Giving it colours and spacing is the next stop: [style a page with CSS](/blog/css-basics/).

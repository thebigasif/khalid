---
title: "How the web works: what happens when you open a page"
slug: how-web-works
date: 2026-07-07
author: desmond-vale
reviewed_by: imke-sato
category: the-web
description: "How the web works, in six steps: the address, the name lookup, the request, the server's answer, and how your browser builds the page."
focus_keyword: web works
tags: [beginners, browsers]
image:
  src: lead.jpg
  alt: The cut end of a thick network cable on a white surface, its foil-wrapped pairs of coloured copper wires fanned out
---

Here is how the web works: by request and answer. Your browser asks another computer, called a server, for a file, and the server sends it back. The browser then reads that file, which is written in HTML, fetches any pictures, styles and scripts it mentions, and draws the page on your screen.

Everything else in how the web works is detail around those two moves. This guide walks through the detail in the order it happens, because knowing it makes the rest of the path, HTML, CSS and JavaScript, make sense.

::toc

:::callout{kind=summary}
- **A web page is a file**, usually HTML, kept on a computer called a server.
- **Your browser asks for it** using the page's address, and the server answers.
- **The HTML lists other files**, such as styles and pictures, and the browser asks for each one.
- **The browser builds the page** from all of them, in a fraction of a second.
:::

## The words you need

| Word | What it means |
|---|---|
| Browser | The program you use to view web pages. It asks for files and draws them. |
| Server | A computer, often in a data centre, that keeps website files and sends them when asked. |
| URL | The address of a page, such as `https://www.example.com/about/`. |
| Domain name | The readable part of the address, such as `example.com`. |
| IP address | The number that identifies a computer on the internet, such as `203.0.113.24`. |
| HTTP | The set of rules browsers and servers use to ask and answer. HTTPS is the same, sent in code so nobody can read it on the way. |

## How the web works, step by step

Say you type `https://www.example.com/about/` and press Enter. Here is what happens, step by step.

::::howto{name="Follow a web page from address to screen" time="0:00:01"}
:::step{name="The browser reads the address"}
It splits it into parts: `https` is the set of rules to use, `www.example.com` is the computer to ask, and `/about/` is the page on that computer.
:::
:::step{name="It looks up the name"}
Computers find each other by number, not by name. The browser asks a directory service called DNS, "What is the number for `www.example.com`?" and gets back an IP address.
:::
:::step{name="It connects and asks"}
The browser opens a connection to that number and sends a short request: "Please send me /about/."
:::
:::step{name="The server answers"}
The server finds the file and sends it back with a status code. `200` means "here it is". `404` means "no such page". The [list of status codes on the Mozilla developer site](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status) explains the rest.
:::
:::step{name="The browser reads the HTML"}
The file is HTML: text with tags that say what each part is. The browser reads it from top to bottom.
:::
:::step{name="It fetches the rest and draws the page"}
The HTML mentions a stylesheet, some pictures and perhaps a script. The browser asks the server for each one, then draws the finished page.
:::
::::

All six steps usually take less than a second. That, in short, is how the web works for every page you open.

:::figure{src="connector.jpg" alt="Close view of the clear plastic plug on the end of a grey network cable, with eight gold contacts and coloured wires inside"}
Much of the journey runs through plugs and cables like this one, from your home router to the server's building and back.
:::

## See how the web works for yourself

You can watch this happen. Open your browser's developer tools (F12 on most computers, or Cmd + Option + I on a Mac), click the **Network** tab, then reload the page. Each row is one file the browser asked for. Click a row to see its status code and how long it took.

If you are comfortable with a terminal, the `curl` command makes a request and shows the raw answer. It is built into Windows 10 and 11, macOS and most Linux systems:

```bash
curl -I https://www.example.com/
```

The `-I` asks only for the headers, the short note that comes before the page. The first line of the answer reads something like `HTTP/2 200`, which means the server found the page. Our [terminal guide](/blog/terminal-basics/) explains how to open a terminal.

## The three languages of a web page

Every page you have ever visited is built from three kinds of file, each with one job.

| Language | Its job | A tiny example |
|---|---|---|
| HTML | Says what each part of the page is: a heading, a paragraph, a link | `<h1>My allotment</h1>` |
| CSS | Says how each part looks: colours, sizes, spacing, layout | `h1 { color: darkgreen; }` |
| JavaScript | Makes the page react: open a menu, check a form | `button.onclick = showMenu;` |

You will learn them in that order on this path, because each one builds on the one before.

:::callout{kind=note}
Some websites build their HTML on the server each time you ask, for example to show your own shopping basket. The browser cannot tell the difference: it still receives HTML. Your first website will be a set of ready-made files, which is the simplest kind and is called a static site.
:::

## Why this matters for a beginner

Knowing how the web works saves you hours later. When a picture does not show on your page, you will know the browser asked for a file and did not find it, so you will check the file name and path. When a page looks unstyled, you will know the stylesheet did not load. Many beginner problems are a missing file, and the Network tab shows them at once.

:::callout{kind=read}
Next on the path: [write your first HTML page](/blog/first-html-page/), the file at the heart of every request.
:::

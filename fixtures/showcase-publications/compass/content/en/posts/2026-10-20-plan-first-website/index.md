---
title: "Your first website, step 1: plan it on paper"
slug: plan-first-website
date: 2026-10-20
author: ruben-achterberg
reviewed_by: hanna-ostrow
category: first-project
description: Plan your first website on paper before you write any code. Choose one reader and one purpose, list three pages and sketch each one.
focus_keyword: first website
tags: [projects, planning, html]
series:
  name: Your first website
  order: 1
image:
  src: lead.jpg
  alt: An open diary with blank lined pages on a brown wooden desk, beside red, white and black pencils, a pair of glasses and a pot of red ink
---

Plan your first website on paper before you write any code: decide who it is for and what it should help them do, then list no more than three pages and sketch each one in a few boxes. An hour of planning saves you several evenings of building pages you later throw away.

This is the first of four guides in which you build and put online a small website of your own. You will use everything from the earlier stops on the path. If you have not followed them, start with [how to start coding as an adult](/blog/start-coding/).

:::callout{kind=summary}
- **One reader, one purpose.** Write both down in a sentence each.
- **Three pages at most.** You can always add more after it is online.
- **Sketch boxes, not pictures.** Each box is a heading, a paragraph, a list or a photo. Designers call these sketches [wireframes](https://en.wikipedia.org/wiki/Website_wireframe).
- **Write the real words first.** Content decides the design, not the other way round.
:::

## Pick a first website you will finish

Your first website should be small, personal and useful to one real person. Our project groups have built a page for a choir's concert dates, a CV for a career change, a menu for a market stall and a guide to a village's footpaths. The sites that got finished all had one thing in common: the builder knew exactly who would read it.

:::callout{kind=tip}
If you cannot choose, build an online CV. Everyone has the content already, it needs only one or two pages, and you will use it.
:::

## Plan your first website, step by step

::::howto{name="Plan your first website on paper" time="1:00:00"}
:::step{name="Write the reader and the purpose"}
Finish two sentences: "This site is for..." and "After reading it, they will...". For example: "This site is for people who walk in Kestrel Bay. After reading it, they will pick a footpath for the weekend."
:::
:::step{name="List what the reader needs"}
Write every question that reader might have, one per line. Do not judge them yet.
:::
:::step{name="Group the questions into pages"}
Put related questions together. Aim for three groups or fewer. Each group is a page, and its biggest question is that page's heading.
:::
:::step{name="Sketch each page"}
On a sheet of paper per page, draw a box for each part, from top to bottom: header and menu, main heading, then the content, then a footer. Label each box with what goes in it.
:::
:::step{name="Write the words"}
Write the real text for the home page, even roughly. You will paste it into your HTML in step 2.
:::
::::

:::figure{src="orienteering-map.jpg" alt="A hand holding a clear baseplate compass on a printed orienteering map with numbered control circles, against snow"}
A plan is a map of the site before you walk it. The numbered stops on this map work like the pages on your list.
:::

## An example plan

Here is the plan one student, Ines, made for her first website, a footpath guide. It fitted on one side of paper.

| Page | Its main question | What goes on it |
|---|---|---|
| Home (`index.html`) | Which walk should I do? | A short welcome, a list of four walks with distance and difficulty, a photo |
| A walk (`cliff-path.html`) | What is this walk like? | A description, a numbered list of directions, where to park |
| About (`about.html`) | Who made this, and how do I suggest a walk? | Two sentences about Ines, an email link |

She built the home page and one walk page first, and added the other three walks after the site was online.

## Name your files now

Decide your file names while you plan, so links between pages are easy to write later. Three rules keep you out of trouble:

1. Use only lowercase letters, numbers and hyphens: `cliff-path.html`, not `Cliff Path.html`.
2. Call the home page `index.html`. Web servers open it automatically when someone visits your site's address.
3. Put photos in a folder called `images`.

Your project folder will look like this:

```text
my-first-site/
  index.html
  cliff-path.html
  about.html
  style.css
  images/
    cliff-path.jpg
```

## What not to plan yet

Leave colours, fonts and clever effects for later in your first website. Beginners often spend their first evenings choosing a colour scheme and never write a page. The colours will be easier to choose once the words are on the screen. Keep a note of ideas for "version 2" and come back to it after step 4, when [what to learn next](/blog/learn-next/) suggests where to take them.

:::callout{kind=read}
Next in the series: [step 2, build the pages](/blog/build-first-website/), where your paper plan becomes HTML and CSS.
:::

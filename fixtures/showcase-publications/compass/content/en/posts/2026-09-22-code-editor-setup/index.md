---
title: "Set up a code editor for your first website"
slug: code-editor-setup
date: 2026-09-22
author: imke-sato
reviewed_by: desmond-vale
category: tools
description: Set up a code editor for your first website. What to look for in a free editor, the five settings to change and the shortcuts worth learning first.
focus_keyword: code editor
tags: [tools, beginners]
image:
  src: lead.jpg
  alt: An open laptop on a light wooden desk with a drawer, a pile of colourful books against the wall and a yellow chair pulled up
---

A code editor is a text editor made for writing code: it colours your code so the parts are easy to tell apart, closes brackets and tags for you, and shows mistakes as you type. Set one up before your first real project, change five settings, and learn six shortcuts, and you will write HTML and CSS faster and with fewer errors than in a plain text editor.

Every code editor we mention in class is free. Wikipedia keeps a [general description of source-code editors](https://en.wikipedia.org/wiki/Source-code_editor) if you want to know how they differ from word processors. We do not name one here, because the good ones all do what a beginner needs, and the best one is whichever you will open every evening.

## What to look for in a code editor

Any code editor with these features will serve you well for years:

- **Syntax highlighting:** tags, text and values in different colours.
- **Auto-closing:** type `<p>` and it adds `</p>`; type `(` and it adds `)`.
- **A file tree:** your project's folder shown down the side, so you can open any file with one click.
- **A built-in terminal:** a terminal panel inside the editor, already in your project folder.
- **Free and kept up to date:** check that the last update was within the past few months.

:::callout{kind=tip}
Pick one code editor and stay with it for at least three months. Switching editors every few weeks feels like progress and is not. The time is better spent on the guides.
:::

## Set up your code editor, step by step

::::howto{name="Set up a code editor" time="0:30:00"}
:::step{name="Install it from the maker's own site"}
Download the editor from its official website, not from a download page that bundles other programs. Install it like any other program.
:::
:::step{name="Open your project folder"}
Use **File** then **Open Folder** and choose your `my-first-site` folder. Always open the folder, not a single file, so the file tree shows everything.
:::
:::step{name="Turn on the five settings"}
Change the settings in the table below. Most editors have a search box in their settings page; type the name to find each one.
:::
:::step{name="Open the built-in terminal"}
Find **Terminal** in the editor's menus and open a new one. Type `pwd` and check it is in your project folder.
:::
:::step{name="Test it"}
Open `index.html`, type `<h2>` and check that the closing tag appears. Save with Ctrl + S or Cmd + S.
:::
::::

## The five code editor settings to change

| Setting | Change it to | Why |
|---|---|---|
| Auto save | After a short delay | You forget to save less often, and the browser shows your latest work |
| Tab size | 2 spaces | Keeps nested HTML from drifting off the right edge |
| Format on save | On, if the editor offers it | Tidies your indentation every time you save |
| Word wrap | On | Long lines of text wrap instead of running off the screen |
| Font size | Large enough to read without leaning in | You will look at this text for hours |

:::callout{kind=note}
Many editors offer extensions, small add-ons that add features. Install none for your first month. Each one adds menus and warnings that make it harder to learn what the editor itself does. When you hit a real need, add one extension for it.
:::

## The six shortcuts worth learning first

| Action | Windows and Linux | Mac |
|---|---|---|
| Save | Ctrl + S | Cmd + S |
| Undo | Ctrl + Z | Cmd + Z |
| Find in this file | Ctrl + F | Cmd + F |
| Comment out a line | Ctrl + / | Cmd + / |
| Move a line up or down | Alt + Up or Down | Option + Up or Down |
| Open a file by name | Ctrl + P | Cmd + P |

These six cover most of what you do in a day. Learn one a week. Write it on a sticky note on your screen until your fingers know it.

## Comments: notes to yourself

Every language lets you leave notes the computer ignores. They are called comments, and the comment shortcut adds the right kind for the file you are in:

```html
<!-- The menu. Links go in the same order as the pages. -->
<nav>
  <a href="index.html">Home</a>
  <a href="about.html">About</a>
</nav>
```

```css
/* Body text: keep it close to black so it reads well on a phone. */
body {
  color: #1a1a1a;
}
```

Write a comment whenever you do something you might not understand next month. Your future self is the person most likely to read your code.

## A warning about code editor suggestions

Many editors now suggest whole lines of code as you type. While you are learning, turn these suggestions off, or at least type the code yourself before you accept one. Accepting code you could not have written is the fastest way to end up with a page you cannot fix.

With an editor ready, you are set for the tool that keeps every version of your work: [Git for beginners](/blog/git-first-steps/).

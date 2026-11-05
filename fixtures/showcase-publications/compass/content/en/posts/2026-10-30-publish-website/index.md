---
title: "Your first website, step 4: publish it online"
slug: publish-website
date: 2026-10-30
updated: 2026-11-03
updated_note: We added the local test server for students whose pages worked from the folder but not online.
author: ruben-achterberg
reviewed_by: imke-sato
category: first-project
description: Publish your first website online with a free static host. Test it locally, choose how to upload and check every page on a real phone.
focus_keyword: publish
tags: [projects, git, terminal]
series:
  name: Your first website
  order: 4
image:
  src: lead.jpg
  alt: A small folded paper boat sitting in a rain puddle on red earth, with blades of grass and fallen leaves around it
---

To publish your website online, upload your project folder to a static hosting service, which stores your files on a server and gives them a web address. Many such services have a free plan that is plenty for a small personal site, and you can either drag your folder onto their upload page or connect them to a Git repository so every commit goes live.

This is the last of four steps. You need the finished, phone-tested site from [step 3](/blog/responsive-website/). The whole step takes most people one evening.

::toc

:::callout{kind=summary}
- **Test with a local server first**, so you see the site the way a real server shows it.
- **A static host** is all a site of HTML, CSS and pictures needs.
- **Upload by drag and drop or by Git.** Both work; Git makes updates easier.
- **Check every page on a real phone** at the new address.
:::

## Test with a local server

Opening `index.html` by double-clicking works for building, but a real server behaves slightly differently. Links that work from your folder can break online, usually because of a capital letter or a path that only works on your computer. Run a small server on your own machine to catch these before anyone else sees them.

If Python is installed, as it is on most Mac and Linux computers, open a terminal in your project folder and run the [built-in Python web server](https://docs.python.org/3/library/http.server.html):

```bash
python3 -m http.server 8000
```

Then open `http://localhost:8000` in your browser. Your site loads from the little server instead of from the folder. Click every link. Press Ctrl + C in the terminal to stop the server when you are done.

:::callout{kind=note}
On Windows, the command may be `python -m http.server 8000`, or Python may not be installed. If so, skip this test and check every link carefully once the site is online instead.
:::

## Choose how to upload

| Way | How it works | Best for |
|---|---|---|
| Drag and drop | You drag your project folder onto the host's upload page | Your very first site, or one you rarely change |
| Connected to Git | The host watches your repository and publishes each commit you push | Sites you will keep improving |
| File upload to web hosting | You copy files to space you rent, with a file transfer program | When you already pay for web hosting, for example with a domain |

Our project groups start with drag and drop, then move to Git once their site is online and they want to make changes.

:::callout{kind=tip}
When you compare hosts, check three things on their pricing page: that the free plan allows static sites, that it gives you HTTPS (the padlock) at no cost, and that you can add your own domain name later.
:::

## Publish your website, step by step

::::howto{name="Put your first website online" time="1:00:00"}
:::step{name="Check your files"}
Run `ls` in your project folder. You should see `index.html` at the top level, not inside another folder. Hosts look for `index.html` first.
:::
:::step{name="Shrink large photos"}
Run `ls -lh images` to see each photo's size. Any photo over 500 KB will be slow on a phone: resize it to about 1600 pixels wide in a photo editor and save it again.
:::
:::step{name="Commit the final version"}
`git add .` then `git commit -m "Ready to publish"`. You publish exactly what is in this commit.
:::
:::step{name="Create a site on the host"}
Sign up with a static host, choose to create a new site, and either drag your folder onto the page or connect your repository.
:::
:::step{name="Visit your address"}
The host gives you an address, often a made-up word followed by the host's own domain. Open it. Your site is on the internet.
:::
:::step{name="Test it everywhere"}
Open every page on your laptop, then on your phone over mobile data rather than home wifi. Send the address to one friend and ask them to tell you what confuses them.
:::
::::

## If a page or picture is missing online

Most problems at this stage are file names. Servers on the internet treat `Photo.jpg` and `photo.jpg` as two different files, even when your own computer does not.

| What you see | Usual cause | Fix |
|---|---|---|
| A 404 page instead of your home page | `index.html` is inside a subfolder, or named `Index.html` | Move it to the top level and use lowercase |
| A broken picture | The `src` and the file name differ in a capital letter or extension | Make them match exactly, then upload again |
| No styling at all | The stylesheet link has a wrong path | Check `href="style.css"` against the real name |
| Old version still showing | Your browser kept a copy | Reload with Ctrl + Shift + R, or Cmd + Shift + R on a Mac |

:::callout{kind=warning}
Everything you upload is public, including photos and anything left in comments in your HTML. Before you publish, check that your pages do not show your home address, other people's photos without permission or any password you used while testing.
:::

## Celebrate, then keep going

You built a website and put it on the internet. Send the address to the person you planned it for in [step 1](/blog/plan-first-website/). Then write down three things you would like to change. Those are your version 2, and making them is how you keep learning.

:::callout{kind=read}
Wondering what to learn now? Read [what to learn after your first website](/blog/learn-next/).
:::

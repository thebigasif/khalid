---
title: "Video notes: debugging INP with field data"
slug: inp-video-notes
date: 2026-10-16
author: tomas-lindqvist
reviewed_by: dalia-moreno
category: technical-seo
description: Our notes on debugging INP on a blog. Read the field data, split a slow tap into its three phases, and find the cause in Chrome DevTools.
translation: inp-video-notes
focus_keyword: INP
cluster:
  name: technical-seo
  role: member
related: [core-web-vitals, technical-seo-audit]
tags: [page speed]
image:
  src: cover.svg
  alt: A video player frame with a finger tapping a button, and a stopwatch beside it split into three coloured parts
---

Debugging INP starts with field data, not with a lab test. Field data tells you which pages feel slow to real readers and on which devices. Then you reproduce one slow tap in Chrome DevTools, split it into its three phases, and fix the phase that takes longest. These are our notes, next to a talk from the Chrome team.

::embed{src="https://www.youtube.com/watch?v=xfjumh8ySRY" title="New field insights for debugging INP"}

:::callout{kind=note}
The video is "New field insights for debugging INP", a Google I/O 2024 talk by Jeremy Wagner, published on the Chrome for Developers channel. Its listing says it covers new Chrome tools that explain the causes of slow interactions, and a release of the web-vitals JavaScript library that gathers those details from real users. The notes below are our own advice, not a summary of the talk.
:::

::toc

## What INP measures

[[interaction-to-next-paint|Interaction to Next Paint]], or INP, measures how quickly a page responds when someone clicks, taps or types. The browser times each of those interactions during a visit, from the input to the next frame painted on screen. The page's score is close to the slowest one. web.dev's [guide to INP](https://web.dev/articles/inp) sets the bands: 200 milliseconds or less is good, and over 500 milliseconds is poor.

On a blog, the slow interactions are rarely in the article. They hide in the menu button, the search box, the cookie banner and the comment form. Our [Core Web Vitals guide for writers](/blog/core-web-vitals/) covers the other two metrics.

## Read the field data first

Field data comes from real visits. Lab data comes from one test on one machine. For debugging INP you need both, but in that order, because a lab test only shows what you thought to click.

- **The Chrome UX Report (CrUX).** Google's public set of real Chrome user data. [Its documentation](https://developer.chrome.com/docs/crux) explains what it covers. PageSpeed Insights shows it at the top of each report, for the page and for the whole site.
- **Search Console's Core Web Vitals report.** It groups your URLs into good, needs improvement and poor, and names the metric at fault. Google's page on [Core Web Vitals and search](https://developers.google.com/search/docs/appearance/core-web-vitals) links to it.
- **Your own field data.** The web-vitals library can send each visit's INP to your analytics, with details on the element that was tapped.

Look for a pattern before you open DevTools. Is it every template, or only pages with comments? Phones only, or desktop too? A group of slow URLs that share one template usually share one cause.

## The three phases of an interaction

Every interaction has three parts. Your fix depends on which one is long.

:::figure{src="inp-phases.svg" alt="A timeline of one tap split into the three INP phases: input delay, processing time and presentation delay, ending at the next paint"}
One tap, from finger to frame. Look for the longest of the three bars: it tells you where to start.
:::

1. **Input delay.** The time before the page can even start to respond. It is long when the main thread is busy with other work, such as a script still loading.
2. **Processing time.** The time the page's own code takes to handle the event. It is long when a click handler does too much at once.
3. **Presentation delay.** The time the browser needs to work out the layout and paint the next frame. It is long when the page is huge or a change forces the whole layout to be worked out again.

## Common causes on blogs

| Cause | Phase it hurts | What to try |
|---|---|---|
| Heavy third-party scripts: ads, chat widgets, tag managers | Input delay | Load them later, or drop the ones nobody uses |
| A large DOM: long comment threads, mega menus | Presentation delay | Paginate comments, simplify the menu |
| Long tasks from theme scripts | Input delay and processing time | Split work into smaller pieces, defer what can wait |
| A search box that filters on every key press | Processing time | Wait until the reader pauses before filtering |

The DOM is the tree of elements the browser builds from your HTML. Every element in it adds work when the layout changes.

## Find a slow interaction with Chrome DevTools

::::howto{name="Find a slow interaction in Chrome DevTools" time="20:00"}
:::step{name="Pick the page and the element"}
Use the field data to choose a slow page. Guess the likely element: menu, search, banner or form.
:::
:::step{name="Slow the CPU down"}
Open DevTools, go to the Performance panel and turn on CPU throttling. Your laptop is faster than most readers' phones.
:::
:::step{name="Record the interaction"}
Start a recording, click the element once, then stop.
:::
:::step{name="Read the Interactions track"}
Find your click in the Interactions track. Hover over it to see the three phases and which one is longest.
:::
:::step{name="Look at the main thread"}
Under the interaction, find the long tasks. DevTools marks them with a red corner. Open one to see which script ran.
:::
:::step{name="Fix, record again, then wait"}
Make one change and record again. Then watch the field data, which takes weeks to catch up.
:::
::::

## Keep it fixed

Most INP regressions on blogs arrive with a new widget, not with new writing. Add a check for new third-party scripts to your [technical SEO audit](/blog/technical-seo-audit/), and test the menu and search on a phone after every theme update. Good speed also supports the [[structured-data|structured data on your pages]] and content work you have already done, because a page that responds fast keeps the reader you earned.

::newsletter{label="Your email address" button="Get our technical SEO notes"}

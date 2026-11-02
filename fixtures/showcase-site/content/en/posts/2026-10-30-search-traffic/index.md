---
title: "Search traffic in 2026: what our client charts show"
slug: search-traffic
date: 2026-10-30
reviewed: 2026-10-31
author: priya-raman
reviewed_by: tomas-lindqvist
category: content-strategy
description: How search traffic moved for the blogs we look after this year, where the clicks came from, and the three charts we now send every client each month.
focus_keyword: search traffic
tags: [research, measurement]
image:
  src: cover.svg
  alt: A rising line over a grid of month columns, the shape of a year of search traffic
---

Every month we send each client the same three charts about their search traffic. They answer the questions owners ask most: is the blog growing, which pages bring readers in, and how much of it now comes from AI search. This post shows the three charts for a typical client in 2026 and how to read them.

The numbers below are from one anonymised client, a software company with a blog of about two hundred posts, and come from its own [Search Console](https://search.google.com/search-console/about) and analytics, read on 31 October 2026.

## Search traffic month by month

The first chart is the one owners look at first: visits from search engines each month, split into Google and every other engine. The numbers sit in a CSV file next to this post, so we update the file and the chart redraws itself when the site is built.

:::chart{type="line" title="Visits from search in 2026, in thousands" unit="k" src="visits.csv"}
:::

Search traffic grew steadily after a slow start. The dip in August is the summer, not a problem: it happens in the same weeks every year. Read a chart like this one against the same months last year, never against last month alone.

## Which pages bring readers in

The second chart lists the pages that brought the most visits from search in October 2026. Long guides and comparisons lead, and the home page is not even in the top five.

:::chart{type="bar" title="Visits from search by page in October 2026, in thousands" unit="k"}
Page, Visits
Pricing guide, 4.2
Comparison with rivals, 3.6
Setup checklist, 2.9
Glossary of terms, 1.8
Home page, 1.1
:::

If most of your search traffic lands on a few guides, give them the most care: update them first, link to them from new posts and keep their titles sharp. Our [playbook for posts that slipped](/blog/content-refresh/) shows how.

## How much comes from AI search

The third chart is new this year. It shows the share of visits that came from AI search tools, such as answers that cite the blog, in each quarter of 2026 beside the same quarter of 2025.

:::chart{type="column" title="Share of visits from AI search by quarter, 2025 and 2026" unit="%"}
Quarter, 2025, 2026
Q1, 0.6, 2.1
Q2, 0.9, 3.4
Q3, 1.4, 4.8
Q4, 2.0, 6.5
:::

The share of search traffic from AI tools is still small, but it tripled in a year. Pages that state a fact plainly and show its source get quoted most, as our post on [how AI search picks which pages to quote](/blog/ai-search-sources/) explains.

## Where our clients are

Most of the blogs in this report belong to companies near our Bristol office. The map is a live one: drag it, zoom in and out, it works right here in the post.

::embed{src="https://www.openstreetmap.org/export/embed.html?bbox=-2.65%2C51.42%2C-2.52%2C51.49&layer=mapnik" title="A map of Bristol, where most of the companies in this report are" height="360"}

## Build these charts for your own blog

You do not need a dashboard tool to chart your search traffic. Export the months from your analytics as a CSV file, put it next to the post and name it in the chart. Our own planning tool, [Larkspur Planner](https://planner.example.com/reports/), makes the same three charts from your Search Console data and keeps the file up to date for you.

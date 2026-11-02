---
title: "The 2026 content benchmark data, free to download"
slug: content-benchmark-data
date: 2026-10-09
author: priya-raman
reviewed_by: tomas-lindqvist
category: content-strategy
description: Download sample rows of our 2026 content benchmark data as a CSV file, with a data dictionary, our method, the licence and how to cite it.
translation: content-benchmark-data
focus_keyword: benchmark data
cluster:
  name: content-strategy
  role: member
related: [content-benchmark, content-refresh, content-calendar]
tags: [research, measurement]
image:
  src: cover.svg
  alt: A spreadsheet grid with a download arrow pointing into a folder, beside a small bar chart
schema: '{"@context":"https://schema.org","@type":"Dataset","name":"Larkspur content benchmark 2026, sample rows","description":"Illustrative sample rows in the shape of the fictional Larkspur content benchmark of 140 small business blogs, read in August 2026. One row per blog: sector, posts in twelve months, author names, blank meta descriptions, updates, shared calendar and sourced numbers. Made for the Khalid demo; the values are not real measurements.","url":"https://www.example.com/blog/content-benchmark-data/","keywords":["content benchmark","blog audit","content marketing","small business blogs","sample data"],"creator":{"@type":"Organization","name":"Larkspur Studio","url":"https://www.example.com/blog/","logo":"https://www.example.com/blog/about/logo.svg"},"license":"https://creativecommons.org/licenses/by/4.0/","isAccessibleForFree":true,"temporalCoverage":"2026-08","variableMeasured":["blog_id","sector","posts_published_12m","has_author_names","blank_meta_share","updated_in_12m","shared_calendar","sourced_numbers"],"distribution":{"@type":"DataDownload","encodingFormat":"text/csv","contentUrl":"https://www.example.com/blog/content-benchmark-data/benchmark-2026-sample.csv"}}'
---

This page shares the benchmark data behind our 2026 content benchmark, in the shape we used to count it. You get a CSV file with one row per blog, a data dictionary that explains each column, our method, and the terms for reuse. Open it in any spreadsheet and run your own counts.

:::callout{kind=warning}
Larkspur Studio and this benchmark are fictional. The rows are illustrative sample data made for the Khalid demo.
:::

::toc

## Download the benchmark data

::file{src="benchmark-2026-sample.csv" title="Content benchmark 2026, sample rows (CSV)"}

The file holds twenty sample rows, not every blog we read in 2026. Each row is one blog, named only by a short code. We never publish the names of the blogs we read. Our [news post on opening the data](/blog/news/benchmark-data-open/) explains why we share it. The full write-up of the findings lives on its own page, [the 2026 content benchmark](/blog/content-benchmark/), with five findings and what to fix first.

## What the benchmark measured

We read, in 2026, 140 small business blogs, post by post, all in August. We counted only what any reader can see on the page. The headline findings, all from that 2026 read:

| Finding | Share of blogs |
|---|---|
| No author name on posts | 61% in 2026 |
| Meta description blank on at least one post in five | 54% in 2026 |
| No post updated in twelve months | 38% in 2026 |
| A content calendar shared with the whole team | 23% in 2026 |
| Numbers in posts linked to a source | 19% in 2026 |

Posts updated within the year earned, in 2026, 1.6 times the search clicks of untouched posts at the median. The sample rows do not hold click data, so you cannot rebuild that figure from this file.

## Data dictionary

Each column in the benchmark data, what it holds and the values it can take.

:::table{fold=3 more="Show the other columns"}
| Column | What it means | Values |
|---|---|---|
| `blog_id` | A code for the blog, so rows can be cited without naming anyone | A short code, one per blog |
| `sector` | The kind of business behind the blog | software, professional services, manufacturing, logistics |
| `posts_published_12m`, counted in 2026 | Posts published in the year before the read | A whole number |
| `has_author_names` | Whether most posts show the writer's name | yes, no |
| `blank_meta_share` | The share of posts we checked with no meta description | A decimal, from none to all |
| `updated_in_12m`, checked in 2026 | Whether any post shows an update in the past twelve months | yes, no |
| `shared_calendar` | Whether the team said it keeps one calendar everyone can see | yes, no |
| `sourced_numbers` | Whether most numbers in the posts link to a source | yes, no |
:::

The column on updates is our simplest sign of [[content-decay|content decay]]: an archive nobody has touched in a year tends to drift out of date.

## Work out a median yourself

A median is the middle value once you sort a column. It suits blog counts better than an average, because one very busy blog can drag an average up. Sort the column of posts published from low to high. With twenty rows there is no single middle row, so take the two middle values and halve their sum.

:::maths{label="Median"}
\text{median} = \frac{x_{10} + x_{11}}{2} = \frac{24 + 24}{2} = 24
:::

So the median sample blog published 24 posts (2026), about two a month. That matches the median we reported for the full 2026 read.

:::callout{kind=tip}
In a spreadsheet, the MEDIAN function does the same sum in one step. Try it on `blank_meta_share`, then split the result by sector with a filter.
:::

## Method notes

::snippet{slug="benchmark-method"}

- **Who we read.** Public blogs of small companies that sell to other businesses, in four sectors. We picked them from client enquiries and public directories, so they are not a random sample.
- **When.** Every blog was read during August 2026. A post counts as updated only if the page shows a date of change.
- **How.** Two editors read each blog. Where they disagreed, a third read it and decided.
- **One column we asked about.** A calendar is not visible on a blog, so `shared_calendar` comes from a one-line question we sent each team.
- **What we did not measure.** Traffic, rankings and links. Those need access to each site's own tools, which we did not have.

The benchmark data has limits you should keep in mind. Our blogs are small and mostly British. The patterns may not hold for large publishers or other markets.

## Licence and reuse

You may share and adapt the benchmark data for any purpose under the [Creative Commons Attribution 4.0 licence](https://creativecommons.org/licenses/by/4.0/). The one condition is credit: name Larkspur Studio and link to this page.

## Credit us in your own work

Copy this line into your notes or reference list:

```text
Larkspur Studio (2026). Content benchmark 2026, sample rows. Larkspur Ledger. https://www.example.com/blog/content-benchmark-data/
```

## How this page is marked up

This page carries [[structured-data|structured data for a dataset]]. It names the file, its licence, the month it covers and the columns it holds, so dataset search tools can list it. Khalid writes it from the post's settings.

If the numbers prompt you to act, our guides on [refreshing posts that slipped](/blog/content-refresh/) and [planning a calendar your team will keep](/blog/content-calendar/) are good next steps. The wider [content strategy guide](/blog/content-strategy-guide/) puts both in context.

::newsletter{label="Your email address" button="Get next year's data first"}

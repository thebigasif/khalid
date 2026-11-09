---
title: "How to run a technical SEO audit: a complete guide"
slug: technical-seo-audit
date: 2026-05-26
updated: 2026-10-19
updated_note: We added the click depth chart, the hreflang checks and a fuller audit report template.
reviewed: 2026-10-19
author: tomas-lindqvist
reviewed_by: dalia-moreno
category: technical-seo
description: A step-by-step technical SEO audit for small sites. Check crawling, indexing, speed, structure and markup, then rank the fixes by impact and effort.
translation: technical-seo-audit
focus_keyword: technical SEO audit
secondary_keywords: [redirect chains, click depth, page speed]
cluster:
  name: technical-seo
  role: pillar
featured: true
related: [core-web-vitals, internal-linking, xml-sitemaps, structured-data]
tags: [crawling, page speed]
image:
  src: cover.svg
  alt: A site drawn as a tree of pages for a technical SEO audit, with one broken link crossed out and its page marked in orange
---

A technical SEO audit is a check of everything a search engine has to get right before it can rank your pages: can it reach them, can it read them, does it know which version to show, and do they load well for real people. The writing can be superb. If a crawler cannot reach the page, nobody will find it.

This guide is the hub of our technical series. It walks through a full technical SEO audit in the order we run it for clients, with the commands we type and the report we hand over. Each step links to a shorter post when it needs more depth.

:::callout{kind=summary}
- **Work in order.** Crawl first, then indexing, then speed and markup. A fix further down is wasted if the page cannot be crawled.
- **Use two views.** A crawler shows what your site says. Search Console shows what Google did with it. You need both.
- **Fix by impact, not by count.** One wrong noindex on a key template beats a thousand missing alt texts.
- **Write it down.** A short report with an owner per fix is what turns an audit into results.
:::

::toc

## What a technical SEO audit covers

Think of a search engine's work as four stages. It finds a URL, fetches it, decides whether to keep it in its index, then decides where to rank it. A technical SEO audit checks the first three stages and the page experience that feeds the fourth. Content quality and links from other sites matter a great deal, but they are separate jobs.

:::figure{src="audit-flow.svg" alt="A technical SEO audit drawn as a flow of four stages, crawl, index, render and rank, with the checks for each listed beneath"}
The four stages a page passes through, and the checks this guide runs at each one. Notice that a failure on the left blocks everything to its right.
:::

The short animation below shows the same journey in motion: a post is written, a crawler follows links to it, and only then can it climb the results page. It has no sound.

::video{src="crawl-index-rank.mp4" title="How a post reaches the results page" poster="crawl-index-rank-poster.svg" description="A silent animation: a page is written, a crawler follows the links between pages to reach it, and the page then climbs a list of search results." duration="0:12"}

The table below is the map for the rest of the guide. It lists each area, the tool we use, what a healthy site looks like and the fix we make most often.

| Area | Main tool | What good looks like | Most common fix |
|---|---|---|---|
| Crawling | A site crawler | Every key page loads without errors | Remove blocking rules |
| Redirects | Crawler, curl | One hop to the final URL | Point links at the target |
| Indexing | Search Console | Key pages indexed | Remove a stray noindex |
| Duplicates | Crawler | One version per page | Add a canonical tag |
| Sitemaps | Search Console | Only live, indexable URLs | Regenerate the file |
| Structure | Crawler | Key pages within three clicks | Add hub links |
| Speed | PageSpeed Insights | Field data passes all three vitals | Shrink the hero image |
| Markup | Rich Results Test | No errors on key templates | Fill missing fields |
| Languages | Crawler | Hreflang pairs link both ways | Add return links |
| Security | Browser, curl | HTTPS everywhere, no mixed content | Redirect HTTP to HTTPS |

## Before you start a technical SEO audit

Three things make the rest of the work fast.

1. **Access to Search Console.** Google's free console is the only place that shows what Google itself crawled and indexed. If you have not set it up, our guide to [setting up Google Search Console properly](/blog/search-console-setup/) takes about twenty minutes.
2. **A crawler.** Any desktop or cloud crawler that lists every URL with its status code, its canonical tag, its indexing rules and its depth will do. Our [buyer's guide to SEO tools for small teams](/blog/choosing-seo-tools/) compares the kinds on offer.
3. **A list of templates.** Most sites have only a handful of page types: home, category, post, product, author. A fault in a template repeats on every page built from it, so note one sample URL of each.

:::callout{kind=tip}
Run the crawler with a normal browser user agent first, then again as a search engine bot. If the two crawls differ, your server or firewall treats bots differently, and that is your first finding.
:::

:::header{image="cover.svg" alt="The audit checklist and magnifier from the cover, shown again as the banner for part one"}
## Part one: can search engines reach and read your pages

Crawling, status codes, redirects, indexing and duplicates. Most of the serious findings in a technical SEO audit live here.
:::

## Step 1: crawl the site the way a bot does

Every technical SEO audit starts with a crawl, because nothing else matters if bots cannot fetch the page.

### Read the robots.txt file

The [[robots-txt|robots.txt]] file sits at the root of your domain and tells crawlers which paths they may fetch. It is the first file a bot reads, and one wrong line in it can hide a whole site. Open it in a browser and read every rule.

```text
User-agent: *
Disallow: /admin/
Disallow: /cart/
Disallow: /*?sort=

Sitemap: https://www.example.com/sitemap.xml
```

This file is healthy. It keeps bots out of the admin, the basket and sorted copies of list pages, and it points to the sitemap. The faults we find most often are a leftover `Disallow: /` from a staging site, and rules that block the CSS or script files a page needs to render.

:::callout{kind=warning}
Robots.txt controls crawling, not indexing. A blocked page can still appear in search if other pages link to it, just without a description. To keep a page out of the index, let bots crawl it and use a noindex rule instead. Google explains the difference in its [introduction to robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro).
:::

### Check every status code

A status code is the three-digit answer a server gives for each URL. In your crawl, sort by it and read the groups.

- **[200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200).** The page loaded. This is what you want for every page you hope to rank.
- **[301 and 308](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/301).** Permanent redirects. Fine in moderation, but internal links should point at the final URL.
- **[302 and 307](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/302).** Temporary redirects. Search engines may keep the old URL indexed, so check each one is truly temporary.
- **[404 and 410](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404).** Not found or gone. Fine for pages you removed on purpose, a problem when your own links still point at them.
- **[5xx server errors](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500).** Even a few are urgent, because a crawler that meets them again and again slows down.

:::callout{kind=note}
A [soft 404](https://developers.google.com/search/docs/crawling-indexing/http-network-errors#soft-404-errors) is a page that says "not found" to people but tells bots it loaded fine. Search Console lists them in its indexing report. Fix them by returning a real not found or gone status.
:::

### Find redirect chains and loops

Redirect chains happen when URL A redirects to B, which redirects to C. Each hop adds a round trip for the visitor and one more fetch for the crawler. Google follows a limited number of hops before it gives up[^1], and long redirect chains also waste [[crawl-budget|crawl budget]] on large sites. A loop, where C points back to A, takes the page down for everyone.

To read the headers of one URL without downloading the page, use `curl` with the silent and head flags:

```bash
curl -sI https://www.example.com/old-guide/
```

To follow every hop and print only the status lines and destinations, add `-L` and filter:

```bash
curl -sIL https://www.example.com/old-guide/ | grep -iE "^(HTTP|location)"
```

A healthy result shows one permanent redirect and then a page that loads. If you see three or more status lines before the final page, you have a chain. Update the first redirect to point straight at the final URL, then update your internal links so they skip the redirect entirely.

## Step 2: check what is actually indexed

This is where a technical SEO audit most often finds the costly faults: pages you want in search that Google has left out.

### Look for stray noindex rules

A [[noindex|noindex rule]] tells search engines not to keep a page in their index. It lives in a meta robots tag in the page's head or in an `X-Robots-Tag` HTTP header. It is the single most damaging line in technical SEO when it lands on the wrong template, and it often arrives with a theme update or a staging copy pushed live.

In your crawl, filter for pages marked noindex and read the list. Every URL on it should be one you chose to hide: search results pages, thank-you pages, thin tag archives. If a post template appears, stop the audit and fix it today.

### Read the page indexing report

Search Console's page indexing report shows how many of your URLs Google has indexed and, for the rest, why not. The reasons are the real value. Each one points to a different fix.

:::figure{src="indexing-report.svg" alt="An annotated mock of a page indexing report with three numbered callouts on the totals, the reason list and the example URLs"}
A mock page indexing report with example data. The first callout marks the totals, the second the list of reasons, the third the sample URLs you open to diagnose each reason. Start with the reason that covers your most important pages, not the longest row.
:::

The reasons we act on most often during a technical SEO audit:

- **Excluded by noindex tag.** Confirm each page is meant to be hidden.
- **Duplicate without user-selected canonical.** Google found copies and chose one itself. Add a canonical tag so you choose.
- **Page with redirect.** Usually fine. It turns into a problem only if your sitemap or internal links point at these URLs.
- **Not found.** Pages that are gone. Check whether anything still links to them.
- **Crawled or discovered but not indexed.** Google saw the page and decided it was not worth keeping, or has not fetched it yet. This is often a quality or internal linking signal, not a technical fault.

## Step 3: tidy up duplicate pages

Most duplicate content is accidental. The same page answers at several addresses: with and without a trailing slash, with tracking parameters, under HTTP and HTTPS, or in a print view. Search engines then have to guess which one to show, and links to the page split their value across the copies.

The fix is a [[canonical-url|canonical URL]]: a tag in the head of each copy that names the preferred version.

```html
<link rel="canonical" href="https://www.example.com/blog/technical-seo-audit/">
```

In a technical SEO audit, check three things for every template. The canonical uses an absolute URL with the right protocol and host. It points to a page that returns 200 and is indexable. And every page carries one canonical tag, not two. Google's guide to [consolidating duplicate URLs](https://developers.google.com/search/docs/crawling-indexing/consolidate-duplicate-urls) lists the other signals it weighs, such as redirects and sitemap entries.

:::callout{kind=tip}
A self-referencing canonical, where a page names itself, is a cheap insurance policy. It stops tracking parameters and scrapers from creating copies that compete with the original.
:::

## Step 4: check the XML sitemap

An [[xml-sitemap|XML sitemap]] is a file that lists the URLs you want search engines to find. It does not force anything into the index, but it helps new and deep pages get found faster. During a technical SEO audit, compare the sitemap with your crawl.

- **Every URL in the sitemap should load without errors** and be indexable, with a canonical pointing at itself.
- **Every important page should be in it.** Pages that are in your crawl but missing from the sitemap are a gap.
- **Pages in the sitemap but not in your crawl** are often orphans, pages with no internal links.
- **Dates should be honest.** A `lastmod` date that changes on every build teaches search engines to ignore it.

Our [field guide to XML sitemaps](/blog/xml-sitemaps/) covers index files, size limits and how to submit one.

:::header{image="click-depth.svg" alt="The click depth chart, shown again as the banner for part two"}
## Part two: structure, speed and markup

Once bots can reach and index your pages, the rest of the technical SEO audit asks whether the site is easy to move through, fast to use and clear to machines.
:::

## Step 5: map site structure and click depth

Click depth is the number of clicks it takes to reach a page from the home page. Pages buried deep get crawled less often and receive less internal link value. As a rule of thumb, keep every page you care about within three clicks.

Your crawler reports depth for each URL. Export it and count pages per level. The chart below shows the shape we often see on a blog that has grown for a few years without a plan.

:::figure{src="click-depth.svg" alt="Bar chart of example data showing how many of 1,200 pages sit at each click depth, with a long tail at depth four and beyond"}
Example data from a made-up blog of twelve hundred pages. More than a quarter of the pages sit four or more clicks deep, mostly old posts reachable only through paginated archives. That tail is what hub pages and related links fix.
:::

Look for three patterns. Deep pages that matter, such as a popular guide stuck on page nine of an archive. An [[orphan-page|orphan page]], one with no internal links at all, which crawlers find only through the sitemap. And hubs that do not link to their members. Every topic deserves a [[pillar-page|pillar page]] like this one that links to each post in its cluster, and each post should link back.

Our guide to [planning internal links that crawlers follow](/blog/internal-linking/) shows how to fix all three, including how to choose clear [[anchor-text|anchor text]] for each link.

## Step 6: measure page speed and Core Web Vitals

Page speed matters to readers first and rankings second. Google measures it with three Core Web Vitals, each with a "good" threshold that at least 75% of real visits should meet. The thresholds below come from [web.dev's guide to the vitals](https://web.dev/articles/vitals).

- **Largest Contentful Paint ([[largest-contentful-paint|LCP]]):** how long the main content takes to appear. Good is 2.5 seconds or less.
- **[[interaction-to-next-paint|Interaction to Next Paint]] (INP):** how quickly the page responds when someone taps or clicks. Good is 200 milliseconds or less.
- **[[cumulative-layout-shift|Cumulative Layout Shift]] (CLS):** how much the layout jumps while it loads. Good is 0.1 or less.

:::callout{kind=stat}
**2.5 seconds** is the longest a page's main content should take to appear for a good Largest Contentful Paint, according to [web.dev's LCP guide](https://web.dev/articles/lcp).
:::

In a technical SEO audit, use field data, from real visitors, before lab data from a single test. Search Console's Core Web Vitals report groups URLs by template, which matches how you will fix them. [PageSpeed Insights](https://pagespeed.web.dev/) shows both kinds of data for a single URL and lists the causes.

The causes we find most often are an oversized hero image, web fonts that block text, third-party scripts that hog the main thread, and images or embeds without set dimensions. Our [practical guide to Core Web Vitals](/blog/core-web-vitals/) works through each one, and our [notes from the INP video session](/blog/inp-video-notes/) dig into slow interactions.

:::pullquote{by="Tomas Lindqvist, technical SEO lead"}
Fix speed one template at a time. One change to the post template can lift every article on the site in a single afternoon.
:::

## Step 7: check the mobile experience

Google crawls and indexes the mobile version of your pages, so for a technical SEO audit the mobile page is the real page. Open each template on a phone, not just in a narrow browser window, and check four things.

1. **The same content.** Text, images, links and structured data on mobile should match desktop. Content hidden from mobile may not count.
2. **Readable text** without zooming, and tap targets far enough apart.
3. **No horizontal scrolling** at any width, down to the narrowest phone screen.
4. **No pop-ups** that cover the content as soon as the page opens.

## Step 8: validate structured data

Structured data is code that describes a page in a vocabulary machines read, usually [schema.org](https://schema.org/) in JSON-LD. It can earn rich results, such as review stars or breadcrumbs, and it helps machines understand what a page is about.

In a technical SEO audit, test one URL per template in Google's [Rich Results Test](https://search.google.com/test/rich-results). Fix errors first, because they can stop a rich result from showing. Then check warnings, which flag optional fields you may want to fill. Finally, make sure the markup matches what the page shows. Markup that describes content a reader cannot see breaks Google's guidelines. Our [structured data guide for bloggers](/blog/structured-data/) covers the types worth adding.

## Step 9: check hreflang on multilingual sites

If your site has versions in several languages or for several countries, [[hreflang]] annotations tell search engines which version to show to whom. They live in the head, in HTTP headers or in the sitemap. A technical SEO audit of a multilingual site always checks them.

Hreflang breaks in predictable ways, and a crawler finds all of them:

- **Missing return links.** If the English page names the German one, the German page must name the English one back.
- **Wrong codes.** Use a two-letter language code, then an optional two-letter region code. British English is `en-GB`, not `en-UK`.
- **Links to non-canonical URLs** or to pages that redirect.
- **No self-reference.** Each page should list itself as well as its alternates.

Google documents the rules in its guide to [localised versions of a page](https://developers.google.com/search/docs/specialty/international/localized-versions), and our [hreflang guide for small sites](/blog/hreflang-guide/) shows a working setup.

## Step 10: confirm HTTPS everywhere

This is the quickest check in a technical SEO audit. Every page should load over HTTPS, and the HTTP version of every URL should redirect to it in one hop. Run the redirect check from the first step against `http://` versions of your home page and a few deep pages. Then check three details: the certificate is valid and covers both the bare domain and `www`, no page loads images or scripts over HTTP (mixed content), and canonical tags and sitemaps use the HTTPS addresses.

## Prioritise the fixes by impact and effort

A crawler can report thousands of issues. Most are noise. The point of a technical SEO audit is to choose the handful of fixes that will move results, and do those first.

We score each finding on two questions. How much traffic or revenue sits on the pages it affects? And how many hours will the fix take, counting the developer, testing and release? Then we place it on a simple grid.

:::figure{src="priority-matrix.svg" alt="A two by two grid of impact against effort, with five example findings placed in the quick wins, big projects, fill-ins and skip quadrants"}
The impact and effort grid with example findings. Do the top left first: high impact, low effort. Plan the top right as projects. Batch the bottom left. Question anything bottom right.
:::

::::columns
:::column{title="Fix this week"}
A noindex on a live template. A robots.txt rule that blocks key paths. Server errors on important pages. Canonicals pointing at the wrong host.
:::
:::column{title="Plan for this quarter"}
Rebuilding navigation to cut click depth. Template changes for page speed. Cleaning up years of redirect chains. Adding hreflang to a new language.
:::
::::

Before you commit to a full technical SEO audit of a large site, weigh it against a focused one that checks only your top templates.

:::proscons
+ A full crawl catches rare faults on forgotten sections of the site
+ It gives you a baseline to compare against next time
+ Findings come with counts, which helps make the case for developer time
- It takes days on a large site and buries the key issues in noise
- Many findings repeat the same template fault hundreds of times
- Small teams rarely have the hours to act on everything it finds
:::

For most small sites we start focused: one sample URL per template, the indexing report, the sitemap and the vitals. Then we run a full crawl once a quarter to catch what the samples miss.

## Write the audit report

A technical SEO audit nobody acts on is wasted time. The report is the part that makes the work count, and it should be short enough to read in ten minutes. We use the same template for every client.

```markdown
# Technical SEO audit: example.com, October 2026

## Summary
Three sentences: overall health, the biggest risk, the first fix.

## Findings
| Finding | Pages affected | Impact | Effort | Owner | Due |
|---------|----------------|--------|--------|-------|-----|
| Post template carries noindex | All posts | High | Low | Dev | This week |
| Redirect chains from old URLs | Most old links | Medium | Low | Dev | This month |
| Hero image delays LCP on mobile | All posts | High | Medium | Dev | This quarter |

## Evidence
One screenshot or command output per finding.

## Next check
Date of the follow-up crawl and who runs it.
```

:::toggle{summary="See one finding written up in full"}
**Finding: redirect chains from old URLs.** Nearly a hundred internal links point at URLs that redirect two or three times before they reach the final page. Each hop slows visitors and wastes crawler requests. Fix: update the first redirect of each chain to point at the final URL, then change the internal links to point there directly. Check: rerun the crawl and confirm every redirect has one hop. Owner: the developer. Effort: about two hours.
:::

## Run the audit in a week

Here is the order we follow for a site of up to a few thousand pages. Spread it over a week, or do it in two long days.

::::howto{name="Run a technical SEO audit on a small site" time="8:00:00"}
:::step{name="Gather access and templates"}
Get Search Console access, set up your crawler and list one sample URL per template. Read the robots.txt file before anything else.
:::
:::step{name="Crawl the whole site"}
Run a full crawl and export status codes, canonicals, indexing rules and depth. Note any URL that does not load cleanly.
:::
:::step{name="Compare with the index"}
Open the page indexing report. Match each reason to the URLs in your crawl and note which templates they come from.
:::
:::step{name="Check duplicates and the sitemap"}
Confirm each template has one correct canonical tag. Compare the sitemap with the crawl and list gaps in both directions.
:::
:::step{name="Measure structure and speed"}
Count pages by click depth, list orphans, and read the Core Web Vitals report by template.
:::
:::step{name="Test markup, languages and HTTPS"}
Run one URL per template through the Rich Results Test, check hreflang return links, and test the HTTP to HTTPS redirect.
:::
:::step{name="Score and write the report"}
Place each finding on the impact and effort grid, then write the report with an owner and a date for every fix.
:::
::::

::cta

## Keep the site healthy after the audit

An audit is a snapshot. Sites drift: a plugin update adds a noindex, a redesign moves URLs, a new script slows every page. Three habits keep the gains.

- **A monthly look at Search Console.** Ten minutes on the indexing and vitals reports catches most new faults within weeks.
- **A crawl before every big release.** Crawl the staging site and compare it with live before the change goes out.
- **A quarterly technical SEO audit,** lighter than the first, using the same report so you can see what changed.

:::callout{kind=read}
Our [plain guide to the free SEO tools worth using](/blog/best-free-seo-tools/) lists free crawlers and checkers for each step. For content that slips after the technical fixes, our [refresh playbook](/blog/content-refresh/) shows what to update first.
:::

::bookmark{href="https://developers.google.com/search/docs" title="Google Search Central documentation" description="Google's own guides to crawling, indexing and ranking, the reference behind every check in this audit."}

## Questions about technical SEO audits

::::faq
:::question{text="How often should I run a technical SEO audit?"}
Run a full audit once when you take over a site or before a redesign, then a lighter one each quarter. Check Search Console's indexing and vitals reports monthly in between. Run an extra crawl before any big release, because most serious faults arrive with a change to templates or URLs.
:::
:::question{text="Can I run a technical SEO audit without paying for tools?"}
Yes, for a small site. Search Console is free and shows indexing and speed from Google's side. Several crawlers have free versions that cover a few hundred URLs, and curl, PageSpeed Insights and the Rich Results Test cost nothing. Paid tools save time on large sites and scheduled crawls.
:::
:::question{text="Which finding should I fix first?"}
Fix anything that stops key pages from being crawled or indexed: a blocking robots.txt rule, a stray noindex, server errors or canonicals pointing at the wrong pages. Those cost you traffic today. Speed, markup and structure come next, ranked by how much traffic sits on the pages they affect.
:::
:::question{text="Do I need a developer for a technical SEO audit?"}
You can run the audit yourself with the tools in this guide. You will usually need a developer for the fixes, because most live in templates, server settings or redirects. Write each finding with the evidence and the exact change, so the developer can act without a meeting.
:::
:::question{text="How long does it take to see results?"}
Fixes to crawling and indexing can show within days, once Google recrawls the pages. Speed changes take about a month to show in field data, because the reports use a [rolling 28-day window](https://developer.chrome.com/docs/crux). Structural changes such as better internal links usually take several weeks to settle.
:::
::::

[^1]: Google explains how many redirect hops Googlebot follows in its page on [how HTTP status codes and network errors affect Google Search](https://developers.google.com/search/docs/crawling-indexing/http-network-errors).

::newsletter{label="Your email address" button="Get one technical fix a week"}

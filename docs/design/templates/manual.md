# Manual: design brief

Manual is an original Khalid template for a help centre or product documentation. A reader arrives with a question. The template helps them find the answer fast, read it in numbered sections, and know where to go next when the answer is not enough.

## Who it is for

- A company that answers its customers' questions in a help centre, such as how to set up, charge or return a product.
- A software team that writes product documentation as short how-to articles.
- A club or a council that keeps a set of "how do I" pages for its members.

## The content model

Manual adds no front matter. It reads what Khalid already has:

- **An article is a post.** Its title is the question or the task, such as "Charge the battery".
- **A topic is a category,** in the order `categories.toml` gives. The front page numbers the topics 01, 02, 03 in that order.
- **A guide is a series.** Articles that a reader follows in order, such as the three steps of setting up, form a series. An article in a guide says "Step 2 of 3" and lists the guide's steps beside the text.
- **Facts are "Before you start":** up to four label and value pairs, such as the time it takes, the tools, the models it applies to and how hard it is.
- **Featured posts** are the popular articles on the front page.
- **Tags** are the products or models an article applies to.

## The front page

1. **Hero:** "Help centre" as a small label, the site's title large, the tagline, and a wide search field when the site answers searches. Under it, up to four popular articles as quick links.
2. **Browse by topic:** a grid of topic tiles, each with its number (01, 02), its name, its newest articles as links, and a link to every article in it.
3. **Popular articles:** the featured posts in two columns, each with its topic and summary.
4. **Recently updated:** a dated feed of the newest articles (the shared feed list).
5. Numbered page links, then the call to action band, which a help centre uses for "Contact us".

## The article page

On a wide screen the article sits in three columns:

1. **Start column:** the guide's steps when the article is in a series, with this step marked; otherwise more articles in the same topic.
2. **Middle column:** the topic above the title, the title, the summary (dek), "Updated" with the date and the reading time. "Step 2 of 3" when it is in a guide. Then the "Before you start" box (the facts), the lead picture and the text.
3. **End column:** "On this page", the contents, sticky as the reader scrolls.

Under the text comes the **next steps panel**, styled like a "Was this helpful?" box but with links only, no script: "Did this answer your question?" with two halves. "Yes" leads to the next step of the guide, or the next article to read. "Not quite" leads to the search page, every article in the topic, and the call to action, such as "Contact us". Then tags, the author box and related articles.

On a phone the columns stack: the title and facts, the contents folded, the text, the next steps, then the guide's steps.

## Other pages

- **A topic** (category): "Topic" as a small label, the name, its intro, how many articles it holds, then the articles as rows with a title and summary.
- **A guide** (series page): the same, numbered as steps.
- **Tag, archive, authors, an author and search:** the shared pages in the same type and colours.

## Type pair

- Titles: **Schibsted Grotesk** 700. It is firm and compact, so short questions read at a glance.
- Text: **Albert Sans** 400 and 600. It is open and plain at 18px, for long instructions.
- Section numbers and step counts: **Spline Sans Mono** 500.

All three are SIL OFL, downloaded from the Google Fonts repository on GitHub with their OFL files and self-hosted in the sample brand. The pack's own default uses the system sans and monospace.

## Palette

The pack's own default is a calm cobalt on white with a pale blue surface:

| Token | Value | Check |
|---|---|---|
| background | `#ffffff` | |
| surface | `#eef3f8` | |
| text | `#15202e` | 16.4:1 on white, 14.7:1 on surface |
| muted | `#4b5768` | 7.3:1 on white, 6.6:1 on surface |
| link | `#1a56b8` | 6.9:1 on white, 6.1:1 on surface |
| accent | `#1a56b8` | 6.9:1 on white |
| border | `#d5dde7` | |

The demo's sample brand, Brindle, a made-up maker of electric bikes, is a deep teal link (`#0e6458`, 6.9:1 on its warm white `#fdfcf9`) with a burnt orange accent (`#c4501a`, 4.5:1) for the numbers.

## Signature device

**Numbered like a printed manual.** Every article numbers its sections: 1, 2, 3 for each main heading and 2.1, 2.2 for the parts under it. The numbers hang in the margin at the start of each heading on a wide screen, in the monospace and the accent colour, and the contents column carries the same numbers, so "see 3.2" means the same thing in both places. The front page numbers the topics 01 to 06 in the same monospace.

## How it differs from the existing templates

- **Almanac** and **Compass** are guides for teaching: long reads and step-by-step how-tos. Manual is a help centre: search first, topics as a grid, three columns on each article, and a next steps panel for the reader who is still stuck.
- **Syllabus** is a course that a learner works through from lesson 1. Manual is for a reader who arrives with one question and leaves when it is answered. Guides are short series inside it, not the whole site.
- **Versions** is a changelog. Manual has no version rail or change types.

No new shared section part comes with it. The front page uses the hero and the feed list.

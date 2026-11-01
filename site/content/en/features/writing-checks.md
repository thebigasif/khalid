---
title: "Writing checks: a second reader for every post"
slug: writing-checks
date: 2026-11-01
author: khalid-team
description: Khalid's writing checks read every post before it goes live. They stop the mistakes that hurt a post in search and explain each one plainly.
focus_keyword: writing checks
---

Khalid's writing checks read every post before it goes live, the way a careful editor would. They stop the few mistakes that really hurt a post. They warn about the rest and say how to fix each one.

## What the writing checks do for you

- **They stop the serious mistakes.** A post with no title, no date, no author, no link to another page of your site, two main headlines or broken structured data does not go live.
- **They explain every finding.** Each message says what is wrong, why it matters and what to do, in one short sentence each.
- **They give each post a score.** A post with no findings gets full marks, so you see at a glance which posts need work.
- **They help AI search quote you.** They check that a question in a heading gets a short, clear answer right below it, and that a number names its source.
- **They teach as they go.** After a few posts, you write the way search engines and readers like without thinking about it.

## How the checks work

The part of Khalid that runs the writing checks is called the linter. It runs when you publish, when you build the site, and after every save in the editor. You can also run it yourself, inside your blog's folder:

```sh
khalid lint
```

Most checks only warn, so they never stop you. In a file called `lint.toml` you can turn any check off, or make a warning block publishing instead. A few more checks are there when you want them:

- **A facts file** holds numbers you use in many posts, such as a price. A post that still uses an old value gets a warning.
- **A brief** lists the questions, words and links a post should have. The linter says which ones are still missing.
- **Keyword checks** look for your main phrase in the title, the address, the description and the first paragraph.

Every check, with its full explanation, is in the [rule catalogue on GitHub](https://github.com/thebigasif/khalid/blob/main/crates/khalid-lint/catalogue.toml). This website passes every one of them too.

## See it live

- [The demo blog](/khalid/demo/) passes every check, with full marks on every post.
- [The editing checklist post](/khalid/demo/editing-checklist/) shows the habits the checks reward, such as an answer under each question.
- [How AI search chooses the pages it quotes](/khalid/demo/ai-search-sources/) shows short answers and sourced numbers.

Next: [SEO and AI search, built in](/khalid/features/seo-ai-search/). Also see [scheduled publishing](/khalid/features/schedule-posts/), which runs these checks first.

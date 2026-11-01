---
title: About Khalid
slug: about
description: Who makes Khalid, how it is built, and where to ask a question or report a problem.
---

Khalid is an open-source blog engine made by SerpDad. You can read, change and share its code under the AGPL licence, which means changes you share must stay open too.

## Who makes it

SerpDad is a small team that builds SEO tools. The founder decides what Khalid should do. Claude, an AI model from Anthropic, writes the code. Other agents review each change against a written checklist, and tests must pass before it lands.

## How it is built

Every rule the code follows, every test and every decision is public in [the repository on GitHub](https://github.com/thebigasif/khalid). The plan says what Khalid will and will not do, and the changelog says what changed in each release.

Speed has written budgets, and a change that makes Khalid slower or heavier than its budget does not land. Every page works with scripts turned off, and every message Khalid shows says what happened, why it matters and what to do, in plain words.

## About this website

This website is a Khalid site too. Its pages are text files in the `site` folder of the repository, and the same writing checks and site audit that run on your blog run on it in the project's own tests. If a page here says something Khalid does not do, that is a bug: please tell us.

## How to reach us

- To ask a question or report a problem, [open an issue on GitHub](https://github.com/thebigasif/khalid/issues).
- To report a security problem, follow [the security policy](https://github.com/thebigasif/khalid/blob/main/SECURITY.md) and do not open a public issue.

Want to try it? Go to [Start](/khalid/start/).

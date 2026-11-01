---
title: "The blog API, SDK and MCP: connect your other tools"
slug: blog-api
date: 2026-10-24
author: khalid-team
description: Read and write your blog from other tools, from code and from AI agents, through one blog API with keys you control.
focus_keyword: blog API
---

Your blog should work with your other tools. Khalid has one blog API, a way for programs to talk to it, and everything else uses it: the admin, the command line and AI agents.

## What the blog API does for you

- **Show posts anywhere.** The Content API gives your posts to a website, an app or another site builder.
- **Automate the work.** The Admin API creates, edits and publishes posts from your own scripts.
- **Let AI agents help.** The MCP server lets an AI assistant that speaks MCP, the Model Context Protocol, use your blog through the same API, with the same checks.
- **React when things happen.** Webhooks tell your other tools when a post is published, a form is sent or a subscriber joins.
- **Stay in control.** Each API key has a scope, keys are stored only in a scrambled form, and you can revoke a key at any time.

## How it works

Make a key on the command line, inside your blog's folder, or in the admin:

```sh
khalid keys create
```

The blog API is described in an OpenAPI file, a standard description that many tools read. A TypeScript SDK, a ready-made library for JavaScript and TypeScript code, comes with Khalid. For AI agents, run `khalid mcp`, and run `khalid agents` to print a guide written for them.

There are also starter projects for showing Khalid posts inside an Astro or a Next.js site, for teams whose main website is built with one of those.

## See it live

The blog API needs Khalid running as a server, so the demo has none. You can read every part on GitHub:

- [The OpenAPI description](https://github.com/thebigasif/khalid/blob/main/sdk/typescript/openapi.json) lists every request.
- [The TypeScript SDK](https://github.com/thebigasif/khalid/tree/main/sdk/typescript) shows how code calls it.
- [The starter projects](https://github.com/thebigasif/khalid/tree/main/starters) show posts in other site builders.

Next: [blog hosting and speed](/khalid/features/blog-hosting/). Also see [theme sync and languages](/khalid/features/theme-sync/).

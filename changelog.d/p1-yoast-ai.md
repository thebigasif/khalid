Added
- Writing suggestions in the editor, off until you turn them on: an `[ai]` table in `site.toml` names SerpDad, Anthropic or any OpenAI-compatible service, and the key comes from `KHALID_AI_KEY`, never the file.
- Suggest buttons, marked AI, beside the SEO title, the description and the X and Facebook titles and descriptions: five options, each checked against the length rules. Suggest a fix under checks about words, Summarize for key takeaways, and Suggest alt text in media details. Nothing changes until you pick one.
- `POST /api/admin/v1/ai/suggestions/` for programs and the bulk SEO editor, and `GET /api/admin/v1/ai/` to see whether suggestions are on.
- A key takeaways block, `:::takeaways`, drawn as a box in every template.
- WordPress imports now carry Yoast, Rank Math and All in One SEO share cards for X and Facebook, cornerstone posts as the main post of a topic group, the focus keyword, and related keyphrases and synonyms as secondary keywords. Breadcrumb titles and schema types are listed in the report.

Added
- The Admin API answers each post's Search Console numbers at `GET /api/admin/v1/search-console/report/`: clicks, impressions, click-through rate and position for the last 28 days with final numbers, posts whose clicks fell three months in a row, pages that answer 404 or `noindex` but still get search traffic, and the week's winners and losers. It needs a key with `settings:write`, like the follow-ups route.
- Each post in the Search Console answers, from the report and the follow-ups route, now carries its `id`, so a program or the admin can open it in the editor.

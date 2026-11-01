Fixed
- A blog under a folder of another site, such as `/blog/`, now serves its admin at `/blog/admin/` and both APIs at `/blog/api/`, so the proxy rule `khalid init --stack` prints reaches them. Sign-in links, preview links, webhooks, the OpenAPI document and the newsletter form name the folder too. An old `/admin/` address moves there. A blog at `/` is unchanged.

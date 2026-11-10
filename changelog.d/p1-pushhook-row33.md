Fixed
- A push to the git host now always shows on a running `khalid serve`. The push hook renders the pages again itself once its pull brings in files, so a push that lands while the server is still starting is no longer missed until the next edit.

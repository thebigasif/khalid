Fixed
- Changing the site's head and body code (`[code]` in `site.toml`) through the Admin API now needs `admin:all`, as a post's head code already did, because that code runs in every reader's browser. A key with only `settings:write` can still read the code and save other settings with it unchanged; a change to the code answers 403 with `rule: code`. Owners and admins are not affected.

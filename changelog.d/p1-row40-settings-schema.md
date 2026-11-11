Fixed
- The API document now types the Admin API's `/settings/` answer as `AdminSiteSettings`: one `settings` object with the site's core settings. Before, it borrowed the Content API's `SiteSettings` (a list of public settings), so the TypeScript SDK typed the admin reply wrong. A test now refuses two schema types with the same name.

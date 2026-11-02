# 0000. A staff account writes as one author profile

**Number:** next free. **Status:** Proposed, 2026-11-02. Task: author profiles end to end (`task/p1-authors-complete-*`).

## Context

An author profile is a file in `content/<language>/authors/`, the source of truth for bylines and author pages. A staff account is a row in `data/site.db`. Nothing joined them, so a new owner had no byline or author page until they found the Authors screen and typed a slug, and "Your account" could not show the person's own profile. The plan does not say where the link lives.

## Decision

- **The link lives on the account,** as a nullable `author` column in `users` (migration 0029) holding the profile's identity, such as `en.asif`. The author file stays free of emails and database numbers, so a content folder can be shared or published without leaking who signs in.
- **The profile is found or made when the account is.** `khalid staff owner` and `POST /staff/` call `AuthorProfiles::profile_for(name)`: the slug comes from the name in the site's default language and slug mode, an existing file with that slug is used as it is, and otherwise a file with only the name is made through the Admin API's own author store (`SiteEntries<AdminAuthor>`), the same function `POST /authors/` calls.
- **A failure never blocks the account.** When the profile cannot be read or made, the account is made unlinked and the reason is logged; signing in never depends on a byline.
- **The session and staff list carry `author`,** so the admin's "Your account" edits that profile and a post started in the admin names it as its author.

## Consequences

- Two people with the same name share one profile until the owner gives one of them another; the Authors screen can still make and edit any profile.
- Accounts made before migration 0029 are linked the same way the first time the admin reads their session (`GET /session/`), so an existing owner gets a byline by signing in. An account whose profile cannot be made is told so on "Your account".
- Changing an account's name does not rename its profile, because the profile's slug is in posts' front matter.

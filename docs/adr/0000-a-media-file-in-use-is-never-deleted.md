# 0000. A media file the site still names is never deleted

**Number:** next free. **Status:** Proposed, 2026-11-10. Gap audit 3 row 10, brief G51, plan 5.15, KH-EDT-5.

## Context

The ADR "the media library keeps each file with its details" left deleting a file out, and the Media screen decision of 2026-10-31 kept the button off the screen because the API had no delete and "deleting a file a post still uses would break that post". The library cannot tell which posts show a file: posts name it by its address in their text, and nothing keeps a list of uses.

## Decision

- **`DELETE /api/admin/v1/media/{id}/?updated_at=...` removes the file for good**, with its `item.toml`, and answers 204. Like every other delete it names the copy it was made on (BL-API-7): no `updated_at` is a 400, an old one a 409 naming who changed the item. It needs `media:write`, the scope every media route uses.
- **A file the site still names is refused with a 409** that lists up to three of the files that name it, such as `content/en/posts/2026-09-21-how-to-write-a-title-tag.md`. Every address of a library file ends in `<key>/<name>`, on the site or in a bucket, so `khalid serve` reads `site.toml` and the text files under `content/` and `theme/` for that text at the moment of the delete. A delete is rare and a post saved a second ago must count, so nothing is cached. Links are never followed, so nothing outside the site is read.
- **No trash for media.** The bytes go first, then `media/<key>/`, so a delete that stops half way leaves the item listed and can run again. A bucket that answers 404 counts as already gone. Posts already go to the trash; a file nobody names has nothing to restore into, and the owner still has the original on their own machine.
- **`GET /api/admin/v1/media/folders/`** lists every folder that holds a file, and each folder above one, by name, with how many files each holds itself. The admin offers them as suggestions in the folder field; typing a new folder still creates it with the first file.

## Consequences

- A post that names a file in a way that does not end in `<key>/<name>`, such as a shortened link, is not found, and the file can be deleted under it. The library's own addresses always have that ending.
- On a site with ten thousand posts a delete reads every post once, about a second. If that grows too slow, the check can read the search index instead.
- Routers with no site behind them, such as the Admin API's own tests, delete without the check.

# Example: a membership blog

This is the blog of Silverline Darkroom, a club where members share a darkroom to develop and print film. The club is made up. The darkroom advice is real.

## Who it is for

A club, a studio, a course or any group with members. Free guides bring new readers in. Members read more once they sign in. A course can keep its handouts behind one password.

## What it shows

| Feature | Where to look |
|---|---|
| The server, which members-only posts and passwords need | `site.toml`: `mode = "server"` |
| Posts only signed-in members read | `access: members` in `push-processing` and `darkroom-booking` |
| A post that opens with a password | `password_hash:` in `printing-course-handouts` |
| The last part of a free guide for readers who join the mailing list | the `::gated` block in `choose-film-stock` |
| Step-by-step guides | the `::::howto` blocks in `develop-film-home` and `contact-sheet` |
| Safety warnings and key points | the `warning` and `summary` callouts |
| A page that explains membership | `content/en/pages/join.md` |

It uses the **Harbor** template, made for blogs that offer something in return for signing up: an offer card under the byline and cards for gated content in the text.

## What a reader sees

- **A members-only post** shows its title and summary to everyone, with a note on how to join. The full post is for members. Readers cannot sign in to the public blog yet, so for now nobody outside the admin reads these posts in full.
- **A password post** shows its title, its summary and a password box. The right password opens the full post.
- **A gated guide** shows its first part to everyone and asks for an email address before the rest.

## Try it

```
khalid --site fixtures/examples/membership lint
khalid --site fixtures/examples/membership serve
```

The site needs the Harbor template. Until Harbor is built into your copy of Khalid, change `name = "harbor"` to `name = "classic"` in `site.toml` to try it.

## Make it yours

The course handouts are locked with a password that does not exist, so they stay locked. To choose your own password, open the post in the admin and set it there. Khalid stores only a scrambled form of the password, never the password itself. Never type a password into a content file.

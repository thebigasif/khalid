# 0000. The admin wears a calm app frame and the blog's accent

**Number:** next free. **Status:** Proposed, 2026-11-02. Task: the admin's SaaS look (`task/p1-admin-saas-*`).

## Context

The founder signed in at serpdad.com/blog/admin/ and asked for an admin that "should look like a modern saas". The admin had a white page, a wrapping row of nineteen links, a header with the product's name, and screens that each set their own heading and notices. The plan fixes light only, plain words, one focus per screen and 44 px touch targets, but says nothing of the frame, density or colour.

## Decision

- **One frame for every signed-in screen.** A slim white side menu with the blog's mark and name, its screens in four groups (Write, Grow, Site, Settings) with a line icon each, and a white top bar naming where you are ("Write / Posts"), then search, New post and an account menu. The screen sits on an off-white background (`--color-app`), and things on it are white cards with hairline borders. Shadows only on raised things: menus, the search panel, toasts, primary buttons.
- **The navigation is data** (`components/navigation.ts`). The side menu, the top bar's "where you are" and the search's screen list all read it, so a new screen is one line.
- **Text is 14 px on a 16 px root.** Spacing stays in 4 and 8 px steps because rem is still 16 px. Corners are 8 px.
- **Density follows the pointer.** With a mouse or trackpad (`pointer: fine`) controls are 32 to 36 px tall; on touch they stay 44 px, as FRONTEND.md asks of touch targets.
- **The accent is the blog's own, when it reads well.** The server names `colors.accent` from the theme's tokens in a `khalid-accent` meta only when it reaches 4.5:1 against white, and the admin sets it as `--color-link` at start. Tint, hover and pill shades follow it with `color-mix`. A paler accent, or none, leaves the admin's blue.
- **Below 60em the side menu is behind a button.** It slides over the dimmed page, takes the focus, closes on Escape or a choice, and is hidden from the keyboard while closed.
- **Search covers what the admin can already find:** screens by name, and posts by title or address through the existing link search. It leaves Ctrl K inside the editor's text to link search.
- **Shared parts replace one-off markup:** `PageHeader`, `EmptyState`, `StatusPill`, `Toast`, and button styles primary, secondary (`.btn`) and ghost. The toast fades by CSS alone, so no timer can race a test or hide a message from a screen reader.
- **No site switcher.** Listing the server's sites needs the super-admin session, which a blog's own staff do not have; the Sites panel keeps its own address.

## Consequences

- Every screen changes look at once through the shared classes; only the main screens adopted `PageHeader` in this task, and the rest still write their own `h1` and lede, which look the same.
- The account menu holds Sign out, so it is one click further away.
- A theme whose accent is pale (a yellow, say) never colours the admin; the owner sees the admin's blue and the blog's own colour on the blog.

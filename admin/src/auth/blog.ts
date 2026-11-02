// Whose admin this is. The server names the blog and the main site's logo
// in the admin's page, so the sign-in screen shows them before anyone
// signs in, when neither API answers yet.

function named(name: string): string {
  return document.querySelector<HTMLMetaElement>(`meta[name=${name}]`)?.content.trim() ?? "";
}

/** The blog's name, as `site.toml` gives it; `null` when the page names none. */
export function blogName(): string | null {
  const name = named("khalid-name");
  return name === "" ? null : name;
}

/** The address of the main site's logo; `null` when there is none to show. */
export function blogLogo(): string | null {
  const logo = named("khalid-logo");
  return logo.startsWith("https://") || logo.startsWith("http://") ? logo : null;
}

/**
 * The theme's accent as `#rrggbb`; `null` when the page names none. The
 * server names it only when it reads well on white.
 */
export function blogAccent(): string | null {
  const accent = named("khalid-accent");
  return /^#[0-9a-fA-F]{6}$/.test(accent) ? accent : null;
}

/** Gives the admin's buttons, links and highlights the theme's accent, when there is one. */
export function wearAccent(root: HTMLElement): void {
  const accent = blogAccent();
  if (accent !== null) {
    root.style.setProperty("--color-link", accent);
  }
}

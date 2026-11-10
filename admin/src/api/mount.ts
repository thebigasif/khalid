// Where the blog that served the admin sits on its host. A blog under a
// folder of another site, such as /blog/, keeps its admin and both APIs
// under that folder too, and the server names the folder in the admin's
// page (KH-INS-3).

/** The blog's folder on its host, such as `/blog/`; `/` for a whole site. */
export function mountPath(): string {
  const named = document.querySelector<HTMLMetaElement>("meta[name=khalid-mount]")?.content;
  return named?.startsWith("/") && named.endsWith("/") ? named : "/";
}

/** Where the admin's screens sit, such as `/blog/admin`: the folder the server names, `admin` by default. */
export function adminPath(): string {
  const named = document.querySelector<HTMLMetaElement>("meta[name=khalid-admin-folder]")?.content;
  const folder = named !== undefined && /^[a-z0-9-]{1,40}$/.test(named) ? named : "admin";
  return `${mountPath()}${folder}`;
}

/** The blog's address with no slash at the end, which every API path follows. */
export function blogAddress(): string {
  return `${globalThis.location.origin}${mountPath().slice(0, -1)}`;
}

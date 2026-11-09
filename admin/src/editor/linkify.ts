// Tiptap's Link finds web addresses in typed and pasted text with
// linkifyjs, which carries every domain ending there is and adds 11 KB to
// the compressed admin. vite.config.ts points linkifyjs here instead: the
// admin links what a writer means as a link, an address that starts with
// http://, https:// or www., or an email address. A bare domain such as
// example.com stays text.

/** One piece of text, as Tiptap's Link reads it. */
type Found = {
  type: "url" | "email" | "text";
  value: string;
  isLink: boolean;
  href: string;
  start: number;
  end: number;
};

/** An address, ending before the punctuation that closes a sentence or a bracket. */
const ADDRESS =
  /(?:https?:\/\/|www\.)[^\s<>"]*[^\s<>".,;:!?'")\]]|[\w.+-]+@[\w-]+(?:\.[\w-]+)*\.[a-z]{2,}/gi;

/** The piece of `text` from `start` to `end`, a link when `link` says so. */
function piece(text: string, start: number, end: number, link: boolean, protocol: string): Found {
  const value = text.slice(start, end);
  const web = /^(https?:\/\/|www\.)/i.test(value);
  let href = value;
  if (link) {
    href = web ? value.replace(/^www\./i, `${protocol}://www.`) : `mailto:${value}`;
  }
  const type = !link ? "text" : web ? "url" : "email";
  return { type, value, isLink: link, href, start, end };
}

/** Every address in `text`, in order. */
export function find(text: string, options?: { defaultProtocol?: string }): Found[] {
  const protocol = options?.defaultProtocol ?? "http";
  return [...text.matchAll(ADDRESS)].map((match) =>
    piece(text, match.index, match.index + match[0].length, true, protocol),
  );
}

/** `text` cut into its addresses and the text between them, each read with `toObject`. */
export function tokenize(text: string) {
  const cuts: [number, number, boolean][] = [];
  let at = 0;
  for (const link of find(text)) {
    if (link.start > at) {
      cuts.push([at, link.start, false]);
    }
    cuts.push([link.start, link.end, true]);
    at = link.end;
  }
  if (at < text.length) {
    cuts.push([at, text.length, false]);
  }
  return cuts.map(([start, end, link]) => ({
    toObject: (protocol = "http") => piece(text, start, end, link, protocol),
  }));
}

/** Tiptap registers extra schemes here; the admin links only web and email addresses. */
export function registerCustomProtocol(_scheme: string, _optionalSlashes?: boolean) {}

/** Tiptap clears the extra schemes here when an editor closes. */
export function reset() {}

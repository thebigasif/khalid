/** A post's address as a search result shows it: the site, then each folder after a ›. */
export function shownAddress(url: string): string {
  let address: URL;
  try {
    address = new URL(url);
  } catch {
    return url;
  }
  const folders = address.pathname.split("/").filter((folder) => folder !== "");
  return [address.origin, ...folders].join(" › ");
}

/** A piece of a search result's text, where it starts, and whether Google shows it in bold. */
export type Piece = { text: string; bold: boolean; at: number };

/**
 * `text` in pieces, with every place it holds `keyword`, in any case, in
 * bold, as Google marks the words a reader searched for.
 */
export function boldParts(text: string, keyword: string): Piece[] {
  const wanted = keyword.trim().toLowerCase();
  const lower = text.toLowerCase();
  // Lowercasing changed a length, so positions no longer line up.
  if (wanted === "" || lower.length !== text.length) {
    return [{ text, bold: false, at: 0 }];
  }
  const pieces: Piece[] = [];
  let from = 0;
  for (let at = lower.indexOf(wanted); at !== -1; at = lower.indexOf(wanted, from)) {
    if (at > from) {
      pieces.push({ text: text.slice(from, at), bold: false, at: from });
    }
    pieces.push({ text: text.slice(at, at + wanted.length), bold: true, at });
    from = at + wanted.length;
  }
  if (from < text.length) {
    pieces.push({ text: text.slice(from), bold: false, at: from });
  }
  return pieces;
}

/** Each kind of thing a page's JSON-LD describes, once, in the order the graph lists them. */
export function schemaTypes(schema: Record<string, unknown>): string[] {
  const graph = schema["@graph"];
  const nodes: unknown[] = Array.isArray(graph) ? graph : [];
  const types = nodes.flatMap((node) => {
    const kind = typeof node === "object" && node !== null ? Reflect.get(node, "@type") : null;
    return typeof kind === "string" ? [kind] : [];
  });
  return [...new Set(types)];
}

/** Google's Rich Results Test for the page at `url`. */
export function richResultsTest(url: string): string {
  return `https://search.google.com/test/rich-results?url=${encodeURIComponent(url)}`;
}

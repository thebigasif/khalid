// Puts a suggested internal link into a post's markdown: on the words of
// the text that name the other post, when there are some outside any
// link, or else as a short line at the end.

/** Whether the text at `at` sits inside a markdown link's words or address. */
function inLink(markdown: string, at: number): boolean {
  const before = markdown.slice(0, at);
  const opened = before.lastIndexOf("[");
  const closed = before.lastIndexOf("]");
  const address = before.lastIndexOf("](");
  const ended = before.lastIndexOf(")");
  return opened > closed || address > ended;
}

/**
 * `markdown` with a link to `path`: on the first place it holds `anchor`
 * outside a link, or else as a line "Related: [title](path)." at the end.
 */
export function withLink(markdown: string, anchor: string | null, title: string, path: string) {
  if (anchor !== null && anchor !== "") {
    for (let at = markdown.indexOf(anchor); at !== -1; at = markdown.indexOf(anchor, at + 1)) {
      if (!inLink(markdown, at)) {
        const end = at + anchor.length;
        return `${markdown.slice(0, at)}[${anchor}](${path})${markdown.slice(end)}`;
      }
    }
  }
  return `${markdown.trimEnd()}\n\nRelated: [${title}](${path}).\n`;
}

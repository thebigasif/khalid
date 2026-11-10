// The numbered parts of a post's text a comment can be about. The API counts
// them from 1 and stores only the number, so the editor decides what a part
// is: a run of lines between blank lines, with a code block or a `:::` block
// kept whole up to its closing fence.

const WORDS = 6;

/** The fence a line opens, such as "```" or ":::", or `null` for an ordinary line. */
function opening(line: string): string | null {
  const found = /^(`{3,}|~{3,}|:{3,})/.exec(line.trimStart());
  return found?.[1] ?? null;
}

/** The first words of `lines`, without markdown marks, cut short with "…". */
function nameOf(lines: string[]): string {
  const first = lines.find((line) => line.trim() !== "") ?? "";
  const words = first
    .replace(/^[\s#>*+\-:`~]+/, "")
    .replace(/\{.*$/, "")
    .split(/\s+/)
    .filter((word) => word !== "");
  const kept = words.slice(0, WORDS).join(" ");
  return words.length > WORDS ? `${kept}…` : kept;
}

/** The name of each part of `markdown`, the first part first; part `n` is at `n - 1`. */
export function partNames(markdown: string): string[] {
  const parts: string[][] = [];
  let current: string[] = [];
  let fence: string | null = null;
  for (const line of markdown.split("\n")) {
    if (fence !== null) {
      current.push(line);
      if (line.trim() === fence) {
        fence = null;
      }
      continue;
    }
    if (line.trim() === "") {
      if (current.length > 0) {
        parts.push(current);
      }
      current = [];
      continue;
    }
    fence = opening(line);
    current.push(line);
  }
  if (current.length > 0) {
    parts.push(current);
  }
  return parts.map(nameOf);
}

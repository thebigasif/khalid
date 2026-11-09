// @tiptap/react imports the whole fast-equals library, 7 KB of the admin,
// for one thing: the default way useEditorState tells whether a selector's
// answer changed. vite.config.ts points that import here. The selectors in
// RichText.tsx answer with a yes or no and with a small object of plain
// values, so this compares plain values, arrays and plain objects, even ones
// that hold themselves. Anything else, such as a date, counts as changed,
// which at worst draws the editor's menus once more.

type Plain = Record<string, unknown>;

function isPlain(value: unknown): value is Plain {
  if (typeof value !== "object" || value === null) {
    return false;
  }
  const kind = Object.getPrototypeOf(value);
  return Array.isArray(value) || kind === Object.prototype || kind === null;
}

function sameIn(a: unknown, b: unknown, open: Map<unknown, unknown>): boolean {
  if (a === b || (Number.isNaN(a) && Number.isNaN(b))) {
    return true;
  }
  if (!isPlain(a) || !isPlain(b) || Array.isArray(a) !== Array.isArray(b)) {
    return false;
  }
  // A value already being compared with this one holds itself; the rest of
  // the walk decides.
  if (open.get(a) === b) {
    return true;
  }
  const keys = Object.keys(a);
  if (keys.length !== Object.keys(b).length) {
    return false;
  }
  open.set(a, b);
  const same = keys.every((key) => Object.hasOwn(b, key) && sameIn(a[key], b[key], open));
  open.delete(a);
  return same;
}

/** Whether `a` and `b` hold the same plain values, as fast-equals' function of this name says. */
export function circularDeepEqual(a: unknown, b: unknown): boolean {
  return sameIn(a, b, new Map());
}

/** One change a schema check offers, as the Admin API sends it. */
export type SchemaChange = {
  target: "property" | "type" | "value";
  path: string;
  from: string;
  to: string;
  label: string;
};

/** One step of a path: a property name, or a place in a list. */
type Step = string | number;

/** `offers[1].pricecurrency` as its steps: `offers`, `1`, `pricecurrency`. */
export function pathSteps(path: string): Step[] {
  const steps: Step[] = [];
  for (const part of path.split(".")) {
    const name = part.replace(/\[\d+\]/g, "");
    if (name !== "") {
      steps.push(name);
    }
    for (const index of part.matchAll(/\[(\d+)\]/g)) {
      steps.push(Number(index[1]));
    }
  }
  return steps;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** The value one step into `value`, or undefined when there is none. */
function into(value: unknown, step: Step): unknown {
  if (typeof step === "number") {
    return Array.isArray(value) ? value[step] : undefined;
  }
  return isRecord(value) ? value[step] : undefined;
}

/** `object` with the key `from` renamed `to`, in the same place among its keys. */
function renamed(
  object: Record<string, unknown>,
  from: string,
  to: string,
): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(object)) {
    out[key === from ? to : key] = value;
  }
  return out;
}

/** `value` with `from` made `to`: itself when it is `from`, or the item in a list that is. */
function swapped(value: unknown, from: string, to: string): unknown {
  if (value === from) {
    return to;
  }
  if (Array.isArray(value)) {
    return value.map((item) => (item === from ? to : item));
  }
  return value;
}

/**
 * `text`, JSON-LD, with `change` made, written out again with two spaces. Null when the text is
 * not JSON or no longer holds what the change was made for, so the writer sees the reports again.
 */
export function applySchemaChange(text: string, change: SchemaChange): string | null {
  let root: unknown;
  try {
    root = JSON.parse(text);
  } catch {
    return null;
  }
  const steps = pathSteps(change.path);
  const last = steps.pop();
  if (last === undefined) {
    return null;
  }
  const parent = steps.reduce<unknown>((value, step) => into(value, step), root);
  if (typeof last === "number") {
    if (!Array.isArray(parent) || parent[last] !== change.from) {
      return null;
    }
    parent[last] = change.to;
    return JSON.stringify(root, null, 2);
  }
  if (!isRecord(parent) || !(last in parent)) {
    return null;
  }
  if (change.target === "property") {
    const fixed = renamed(parent, change.from, change.to);
    if (steps.length === 0) {
      return JSON.stringify(fixed, null, 2);
    }
    const holder = steps.slice(0, -1).reduce<unknown>((value, step) => into(value, step), root);
    const place = steps[steps.length - 1];
    if (place === undefined) {
      return null;
    }
    if (typeof place === "number" && Array.isArray(holder)) {
      holder[place] = fixed;
    } else if (typeof place === "string" && isRecord(holder)) {
      holder[place] = fixed;
    } else {
      return null;
    }
    return JSON.stringify(root, null, 2);
  }
  const before = parent[last];
  const after = swapped(before, change.from, change.to);
  if (after === before) {
    return null;
  }
  parent[last] = after;
  return JSON.stringify(root, null, 2);
}

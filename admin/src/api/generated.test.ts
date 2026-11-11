import { describe, expect, it } from "vitest";

// Every Admin API client, as source text, by file name.
const CLIENTS = import.meta.glob<string>(["./*.ts", "!./*.test.ts", "!./*.d.ts"], {
  query: "?raw",
  import: "default",
  eager: true,
});

/** The clients that send their own requests through `call`. */
function callers(): [string, string][] {
  return Object.entries(CLIENTS).filter(([, source]) => source.includes('from "./call"'));
}

describe("the Admin API clients", () => {
  it("take their types from the OpenAPI document, not from hand", () => {
    const found = callers();
    expect(found.map(([name]) => name)).toContain("./site.ts");
    const typedByHand = found
      .filter(([, source]) => !source.includes('from "./schema"'))
      .map(([name]) => name);
    expect(typedByHand).toEqual([]);
  });
});

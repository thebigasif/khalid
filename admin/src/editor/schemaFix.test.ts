import { describe, expect, it } from "vitest";
import { applySchemaChange, pathSteps } from "./schemaFix";

describe("pathSteps", () => {
  it("splits names and list places", () => {
    expect(pathSteps("@graph[0].offers[1].pricecurrency")).toEqual([
      "@graph",
      0,
      "offers",
      1,
      "pricecurrency",
    ]);
    expect(pathSteps("name")).toEqual(["name"]);
  });
});

describe("applySchemaChange", () => {
  const product = JSON.stringify({
    "@type": "Product",
    name: "Kit",
    offers: { "@type": "Offer", price: "5", pricecurrency: "USD", availability: "instock" },
  });

  it("renames a property in its place", () => {
    const fixed = applySchemaChange(product, {
      target: "property",
      path: "offers.pricecurrency",
      from: "pricecurrency",
      to: "priceCurrency",
      label: "Change pricecurrency to priceCurrency",
    });
    expect(fixed).not.toBeNull();
    const offers = JSON.parse(fixed ?? "{}").offers;
    expect(Object.keys(offers)).toEqual(["@type", "price", "priceCurrency", "availability"]);
  });

  it("renames a property at the top", () => {
    const fixed = applySchemaChange('{"@type": "Event", "startdate": "2026-11-12"}', {
      target: "property",
      path: "startdate",
      from: "startdate",
      to: "startDate",
      label: "",
    });
    expect(JSON.parse(fixed ?? "{}")).toEqual({ "@type": "Event", startDate: "2026-11-12" });
  });

  it("changes a value and a type", () => {
    const value = applySchemaChange(product, {
      target: "value",
      path: "offers.availability",
      from: "instock",
      to: "https://schema.org/InStock",
      label: "",
    });
    expect(JSON.parse(value ?? "{}").offers.availability).toBe("https://schema.org/InStock");
    const kind = applySchemaChange('{"@graph": [{"@type": ["Code", "Thing"]}]}', {
      target: "type",
      path: "@graph[0].@type",
      from: "Code",
      to: "SoftwareSourceCode",
      label: "",
    });
    expect(JSON.parse(kind ?? "{}")["@graph"][0]["@type"]).toEqual(["SoftwareSourceCode", "Thing"]);
  });

  it("changes an item of a list", () => {
    const fixed = applySchemaChange('{"@type": "Offer", "priceCurrency": ["usd"]}', {
      target: "value",
      path: "priceCurrency[0]",
      from: "usd",
      to: "USD",
      label: "",
    });
    expect(JSON.parse(fixed ?? "{}").priceCurrency).toEqual(["USD"]);
  });

  it("gives null when the text changed or is not JSON", () => {
    const change = {
      target: "property" as const,
      path: "offers.pricecurrency",
      from: "pricecurrency",
      to: "priceCurrency",
      label: "",
    };
    expect(applySchemaChange("{", change)).toBeNull();
    expect(applySchemaChange('{"offers": {}}', change)).toBeNull();
  });
});

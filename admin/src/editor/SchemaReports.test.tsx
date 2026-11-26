import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import type { SchemaChecks } from "../api/schemaCheck";
import { en } from "../strings/en";
import { SchemaReports } from "./SchemaReports";

const words = en.schema.reports;

const CHECKS: SchemaChecks = {
  types: ["Product"],
  schema_org: [
    {
      level: "error",
      path: "offers.pricecurrency",
      what: "pricecurrency is not a schema.org property; it looks like priceCurrency.",
      why: "Search engines skip a name schema.org does not have, without saying so.",
      fix: "Change pricecurrency to priceCurrency.",
      change: {
        target: "property",
        path: "offers.pricecurrency",
        from: "pricecurrency",
        to: "priceCurrency",
        label: "Change pricecurrency to priceCurrency",
      },
    },
  ],
  google: [],
};

describe("the two schema reports", () => {
  it("offers a one-click change and says when a report has nothing", () => {
    const onChange = vi.fn();
    render(<SchemaReports checks={CHECKS} onChange={onChange} />);
    const schemaOrg = screen.getByRole("region", { name: words.schemaOrg });
    expect(within(schemaOrg).getByText(words.where("offers.pricecurrency"))).toBeInTheDocument();
    fireEvent.click(
      within(schemaOrg).getByRole("button", { name: "Change pricecurrency to priceCurrency" }),
    );
    expect(onChange).toHaveBeenCalledWith(CHECKS.schema_org[0]?.change);
    const google = screen.getByRole("region", { name: words.google });
    expect(within(google).getByText(words.nothing)).toBeInTheDocument();
  });

  it("shows the advice in words when a finding has no one change", () => {
    const first = CHECKS.schema_org[0];
    if (first === undefined) {
      throw new Error("no finding");
    }
    const checks: SchemaChecks = { ...CHECKS, schema_org: [{ ...first, change: null }] };
    render(<SchemaReports checks={checks} onChange={vi.fn()} />);
    expect(screen.getByText("Change pricecurrency to priceCurrency.")).toBeInTheDocument();
    expect(screen.queryByRole("button")).toBeNull();
  });
});

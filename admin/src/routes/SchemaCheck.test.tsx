import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { SchemaChecks } from "../api/schemaCheck";
import { en } from "../strings/en";
import { seoToolWords } from "../strings/seoToolWords";

const words = seoToolWords.schemaCheck;

const SLIP: SchemaChecks = {
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
const CLEAN: SchemaChecks = { types: ["Product"], schema_org: [], google: [] };

describe("the schema check", () => {
  it("checks a page address and shows both reports, with the fix in words", async () => {
    signedIn();
    const sent = adminBlog({ "POST /schema/checks/": () => json(200, SLIP) });
    renderAt("/schema-check");
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    await userEvent.type(
      screen.getByRole("textbox", { name: words.addressLabel }),
      "https://shop.example/kit/",
    );
    await userEvent.click(screen.getByRole("button", { name: words.check }));
    expect(await screen.findByText(words.found("Product"))).toBeVisible();
    expect(screen.getByRole("region", { name: en.schema.reports.schemaOrg })).toBeVisible();
    expect(screen.getByText("Change pricecurrency to priceCurrency.")).toBeVisible();
    expect(
      screen.queryByRole("button", { name: "Change pricecurrency to priceCurrency" }),
    ).toBeNull();
    expect(await sent[0]?.json()).toEqual({ url: "https://shop.example/kit/" });
    expect(await axeViolations()).toEqual([]);
  });

  it("fixes pasted code in place and checks it again", async () => {
    signedIn();
    const sent = adminBlog({
      "POST /schema/checks/": async (request) => {
        const body = await request.clone().json();
        return json(200, String(body.json_ld).includes("pricecurrency") ? SLIP : CLEAN);
      },
    });
    renderAt("/schema-check");
    await userEvent.click(await screen.findByRole("radio", { name: words.code }));
    const code = screen.getByRole("textbox", { name: words.codeLabel });
    await userEvent.click(code);
    await userEvent.paste(
      '{"@type": "Product", "name": "Kit", "offers": {"pricecurrency": "GBP"}}',
    );
    await userEvent.click(screen.getByRole("button", { name: words.check }));
    await userEvent.click(
      await screen.findByRole("button", { name: "Change pricecurrency to priceCurrency" }),
    );
    expect(await screen.findAllByText(en.schema.reports.nothing)).toHaveLength(2);
    expect((code as HTMLTextAreaElement).value).toContain('"priceCurrency": "GBP"');
    expect(sent).toHaveLength(2);
  });

  it("says why when the page cannot be checked", async () => {
    signedIn();
    const problem = {
      type: "https://serpdad.com/khalid/problems/invalid-body",
      title: "The page has no schema to check.",
      status: 422,
      detail: "It carries no JSON-LD script.",
      fix: "Check another page, or paste the code you want checked.",
    };
    adminBlog({ "POST /schema/checks/": () => json(422, problem) });
    renderAt("/schema-check");
    await userEvent.type(
      await screen.findByRole("textbox", { name: words.addressLabel }),
      "https://plain.example/",
    );
    await userEvent.click(screen.getByRole("button", { name: words.check }));
    expect(await screen.findByRole("alert")).toHaveTextContent(problem.title);
  });
});

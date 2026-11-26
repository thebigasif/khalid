import { fireEvent, render, screen, within } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { en } from "../strings/en";
import { SchemaSuggestions } from "./SchemaPanel";

const words = en.schema.reports;

describe("schema the post could carry", () => {
  it("names each type with its reason and how sure Khalid is", () => {
    render(
      <SchemaSuggestions
        suggestions={[
          {
            kind: "Recipe",
            reason: "The post lists 3 ingredients and 3 steps.",
            confidence: 90,
            draft: null,
          },
        ]}
        filled={false}
        onFill={vi.fn()}
      />,
    );
    const list = screen.getByRole("region", { name: words.suggested });
    expect(within(list).getByText("Recipe")).toBeInTheDocument();
    expect(within(list).getByText(/3 ingredients and 3 steps/)).toBeInTheDocument();
    expect(within(list).getByText(`(${words.sure(90)})`)).toBeInTheDocument();
  });

  it("shows nothing when the post suggests nothing", () => {
    const { container } = render(
      <SchemaSuggestions suggestions={[]} filled={false} onFill={vi.fn()} />,
    );
    expect(container).toBeEmptyDOMElement();
  });

  it("fills the schema field with a draft, and says it replaces what is there", () => {
    const onFill = vi.fn();
    const draft = '{"@type": "LocalBusiness"}';
    const business = { kind: "LocalBusiness", reason: "An address.", confidence: 55, draft };
    const { rerender } = render(
      <SchemaSuggestions suggestions={[business]} filled={false} onFill={onFill} />,
    );
    fireEvent.click(screen.getByRole("button", { name: words.fill("LocalBusiness") }));
    expect(onFill).toHaveBeenCalledWith(draft);
    rerender(<SchemaSuggestions suggestions={[business]} filled={true} onFill={onFill} />);
    expect(
      screen.getByRole("button", { name: words.replace("LocalBusiness") }),
    ).toBeInTheDocument();
  });
});

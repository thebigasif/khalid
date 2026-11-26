import { render, screen, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { en } from "../strings/en";
import { SchemaSuggestions } from "./SchemaPanel";

const words = en.schema.reports;

describe("schema the post could carry", () => {
  it("names each type with its reason and how sure Khalid is", () => {
    render(
      <SchemaSuggestions
        suggestions={[
          { kind: "Recipe", reason: "The post lists 3 ingredients and 3 steps.", confidence: 90 },
        ]}
      />,
    );
    const list = screen.getByRole("region", { name: words.suggested });
    expect(within(list).getByText("Recipe")).toBeInTheDocument();
    expect(within(list).getByText(/3 ingredients and 3 steps/)).toBeInTheDocument();
    expect(within(list).getByText(`(${words.sure(90)})`)).toBeInTheDocument();
  });

  it("shows nothing when the post suggests nothing", () => {
    const { container } = render(<SchemaSuggestions suggestions={[]} />);
    expect(container).toBeEmptyDOMElement();
  });
});

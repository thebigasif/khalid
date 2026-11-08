import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { COUNTS, proofBlog } from "../../test/consent";
import { PRIVACY_DEMO } from "../../test/privacy";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { en } from "../strings/en";
import { privacyWords } from "../strings/privacyWords";

const WORDS = privacyWords.proof;

/** Opens the Privacy screen on its Proof tab. */
async function openProof() {
  renderAt("/privacy");
  await userEvent.click(await screen.findByRole("button", { name: privacyWords.tabs.proof }));
}

/** The cells of the table row whose heading is `name`. */
function cells(name: string): string[] {
  const row = screen.getByRole("rowheader", { name }).closest("tr");
  return within(row as HTMLElement)
    .getAllByRole("cell")
    .map((cell) => cell.textContent ?? "");
}

/** Every file the screen saves for download, as it was made. */
function watchSaves(): Blob[] {
  const saved: Blob[] = [];
  URL.createObjectURL = (made: Blob | MediaSource) => {
    if (made instanceof Blob) {
      saved.push(made);
    }
    return "blob:proof";
  };
  URL.revokeObjectURL = () => undefined;
  return saved;
}

describe("the Proof tab", () => {
  it("shows the choices a week a row, one column per category, and the settings history", async () => {
    signedIn();
    proofBlog();
    await openProof();
    expect(await screen.findByRole("heading", { name: WORDS.counts })).toBeVisible();
    expect(cells(WORDS.weekOf("26 October 2026"))).toEqual(["6", "1", "4", "0", "3"]);
    expect(cells(WORDS.weekOf("2 November 2026"))).toEqual(["5", "2", "4", "1", "3"]);
    expect(cells(WORDS.total)).toEqual(["11", "3", "8", "1", "6"]);
    const history = screen.getByRole("table", { name: WORDS.history });
    expect(within(history).getByText("0123456789abcdef")).toBeVisible();
    expect(within(history).getByText("1 October 2026")).toBeVisible();
    expect(within(history).getByText("8 November 2026")).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("downloads the same totals the table shows", async () => {
    signedIn();
    proofBlog();
    const saved = watchSaves();
    await openProof();
    await userEvent.click(await screen.findByRole("button", { name: WORDS.download }));
    await vi.waitFor(() => expect(saved).toHaveLength(1));
    const lines = (await saved[0]?.text())?.trim().split("\n") ?? [];
    expect(lines[0]).toBe("day,version,choices,preferences,analytics,marketing,external_media");
    expect(lines).toContain("2026-11-03,fedcba9876543210,5,2,4,1,3");
    expect(lines.at(-1)).toBe(`total,,${cells(WORDS.total).join(",")}`);
  });

  it("asks for the period picked with the analytics control", async () => {
    signedIn();
    const asked = proofBlog();
    await openProof();
    await screen.findByRole("heading", { name: WORDS.counts });
    await userEvent.selectOptions(screen.getByLabelText(en.analytics.days), "90");
    await userEvent.click(screen.getByRole("button", { name: en.analytics.show }));
    await vi.waitFor(() => expect(asked).toEqual(["28", "90"]));
  });

  it("shows a static site its settings history and an empty counts state", async () => {
    signedIn();
    proofBlog({ ...PRIVACY_DEMO, mode: "static" }, { ...COUNTS, per_day: [] });
    await openProof();
    expect(await screen.findByText(WORDS.none)).toBeVisible();
    expect(screen.getByText(privacyWords.settings.countsStatic)).toBeVisible();
    expect(screen.queryByRole("button", { name: WORDS.download })).toBeNull();
    const history = screen.getByRole("table", { name: WORDS.history });
    expect(within(history).getAllByRole("row")).toHaveLength(3);
  });
});

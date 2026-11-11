import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { item, mediaBlog, mediaPage } from "../../test/media";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { en } from "../strings/en";

const words = en.media.library.replace;

const LOGO = {
  id: "3f2a9c1e0b57d0aa",
  name: "logo.png",
  before: "The Acme Ltd logo",
  after: "The Acme logo",
};

/** A blog whose library has one logo to change, and the replaces it was sent. */
function blogWithLogo(changes = [LOGO]) {
  const asked: { find: string; replace: string; apply: boolean }[] = [];
  mediaBlog({
    "GET /media/": () => mediaPage([item({ alt: LOGO.before })]),
    "POST /media/alt-text/": async (request) => {
      const body = await request.json();
      asked.push(body);
      return json(200, { applied: body.apply, changes });
    },
  });
  return asked;
}

/** Types the words to find and to put in, and asks for the changes. */
async function preview(find: string, replace: string) {
  await userEvent.type(await screen.findByLabelText(words.find), find);
  await userEvent.type(screen.getByLabelText(words.replace), replace);
  await userEvent.click(screen.getByRole("button", { name: words.show }));
}

describe("changing words in alt text", () => {
  it("shows every change first and saves only when asked", async () => {
    signedIn();
    const asked = blogWithLogo();
    renderAt("/media");
    await preview("Acme Ltd", "Acme");
    const table = await screen.findByRole("table", { name: words.table });
    const [, row] = within(table).getAllByRole("row");
    expect(row).toHaveTextContent("logo.png");
    expect(row).toHaveTextContent(LOGO.before);
    expect(row).toHaveTextContent(LOGO.after);
    expect(screen.getByText(words.found(1))).toBeVisible();
    expect(screen.getByText(words.consequence)).toBeVisible();
    expect(asked).toEqual([{ find: "Acme Ltd", replace: "Acme", apply: false }]);
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: words.apply(1) }));
    expect(await screen.findByText(words.done(1))).toBeVisible();
    expect(asked.at(-1)).toEqual({ find: "Acme Ltd", replace: "Acme", apply: true });
    expect(screen.queryByRole("table", { name: words.table })).toBeNull();
  });

  it("says so when no alt text has the words, and offers nothing to save", async () => {
    signedIn();
    blogWithLogo([]);
    renderAt("/media");
    await preview("Globex", "Initech");
    expect(await screen.findByText(words.none)).toBeVisible();
    expect(screen.queryByRole("button", { name: words.apply(0) })).toBeNull();
  });

  it("drops a preview once the words change, so it never saves what was not shown", async () => {
    signedIn();
    blogWithLogo();
    renderAt("/media");
    await preview("Acme Ltd", "Acme");
    expect(await screen.findByRole("button", { name: words.apply(1) })).toBeVisible();
    await userEvent.type(screen.getByLabelText(words.replace), " Inc");
    expect(screen.queryByRole("button", { name: words.apply(1) })).toBeNull();
  });
});

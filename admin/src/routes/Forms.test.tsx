import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { noContent } from "../../test/blog";
import { redirectsBlog } from "../../test/redirects";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { FormAnswer, FormSummary } from "../api/forms";
import { en } from "../strings/en";
import { formWords } from "../strings/formWords";

const CONTACT: FormSummary = { id: "contact", answers: 12, last_at: "2026-11-03T09:07:00Z" };
const WAITLIST: FormSummary = {
  id: "workshop-waitlist",
  answers: 1,
  last_at: "2026-11-01T10:00:00Z",
};

/** An answer from lena@example.com on the contact page, sent at 09:0`id` on 3 November. */
function answer(id: number, message: string): FormAnswer {
  return {
    id,
    page: "/blog/write-to-us/",
    sent_at: `2026-11-03T09:0${id}:00Z`,
    fields: [
      { name: "reply_to", value: "lena@example.com" },
      { name: "message", value: message },
    ],
  };
}

/** One page of answers out of `pages`, as the Admin API sends it. */
function page(answers: FormAnswer[], at: number, pages: number) {
  const pagination = {
    page: at,
    limit: 10,
    pages,
    total: answers.length,
    next: at < pages ? at + 1 : null,
    prev: at > 1 ? at - 1 : null,
  };
  return { answers, meta: { pagination } };
}

describe("the forms screen", () => {
  it("shows the answers to the form answered last, newest first", async () => {
    signedIn();
    redirectsBlog({
      "GET /forms/": () => json(200, { forms: [CONTACT, WAITLIST] }),
      "GET /forms/contact/answers/": () =>
        json(200, page([answer(7, "Do you look at shop sitemaps?"), answer(5, "Thanks")], 1, 1)),
    });
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: en.nav.forms }));
    expect(await screen.findByRole("heading", { level: 1, name: formWords.title })).toBeVisible();
    const which = await screen.findByRole("combobox", { name: formWords.which });
    expect(which).toHaveValue("contact");
    const newest = (await screen.findByText("Do you look at shop sitemaps?")).closest("li");
    const answers = screen.getAllByRole("listitem").filter((item) => item.matches(".answer"));
    expect(answers).toHaveLength(2);
    expect(answers[0]).toBe(newest);
    expect(answers[1]).toHaveTextContent("Thanks");
    expect(newest).toHaveTextContent("lena@example.com");
    const link = within(newest ?? document.body).getByRole("link", { name: "/blog/write-to-us/" });
    expect(link).toHaveAttribute("href", "/blog/write-to-us/");
    expect(screen.queryByRole("button", { name: formWords.older })).toBeNull();
    expect(await axeViolations()).toEqual([]);
  });

  it("switches to another form and pages through older answers", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /forms/": () => json(200, { forms: [CONTACT, WAITLIST] }),
      "GET /forms/contact/answers/": (request) =>
        new URL(request.url).searchParams.get("page") === "2"
          ? json(200, page([answer(1, "The oldest note")], 2, 2))
          : json(200, page([answer(7, "The newest note")], 1, 2)),
      "GET /forms/workshop-waitlist/answers/": () =>
        json(200, page([answer(2, "Count me in")], 1, 1)),
    });
    renderAt("/forms");
    await userEvent.click(await screen.findByRole("button", { name: formWords.older }));
    expect(await screen.findByText("The oldest note")).toBeVisible();
    expect(sent.some((request) => request.url.includes("page=2"))).toBe(true);
    const which = screen.getByRole("combobox", { name: formWords.which });
    await userEvent.selectOptions(which, "workshop-waitlist");
    expect(await screen.findByText("Count me in")).toBeVisible();
    expect(screen.queryByText("The oldest note")).toBeNull();
  });

  it("deletes one answer for good after asking once more", async () => {
    signedIn();
    let kept = [answer(7, "Please erase what I sent"), answer(5, "Thanks")];
    const sent = redirectsBlog({
      "GET /forms/": () => json(200, { forms: [CONTACT] }),
      "GET /forms/contact/answers/": () => json(200, page(kept, 1, 1)),
      "DELETE /forms/contact/answers/7/": () => {
        kept = kept.filter((each) => each.id !== 7);
        return noContent();
      },
    });
    renderAt("/forms");
    const card = (await screen.findByText("Please erase what I sent")).closest("li");
    await userEvent.click(within(card ?? document.body).getByRole("button", { name: /^Delete/ }));
    expect(screen.getByText(formWords.asking)).toBeVisible();
    expect(sent.some((request) => request.method === "DELETE")).toBe(false);
    await userEvent.click(screen.getByRole("button", { name: formWords.sure }));
    expect(await screen.findByText(formWords.removed)).toBeVisible();
    expect(screen.queryByText("Please erase what I sent")).toBeNull();
    expect(screen.getByText("Thanks")).toBeVisible();
  });

  it("keeps the answer when the owner changes their mind", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /forms/": () => json(200, { forms: [CONTACT] }),
      "GET /forms/contact/answers/": () => json(200, page([answer(7, "Hello")], 1, 1)),
    });
    renderAt("/forms");
    await screen.findByText("Hello");
    await userEvent.click(screen.getByRole("button", { name: /^Delete/ }));
    await userEvent.click(screen.getByRole("button", { name: formWords.keep }));
    expect(screen.queryByText(formWords.asking)).toBeNull();
    expect(sent.some((request) => request.method === "DELETE")).toBe(false);
  });

  it("says plainly when no one has sent a form yet", async () => {
    signedIn();
    redirectsBlog({ "GET /forms/": () => json(200, { forms: [] }) });
    renderAt("/forms");
    expect(await screen.findByText(formWords.none)).toBeVisible();
  });

  it("shows why an editor cannot read the answers", async () => {
    signedIn();
    const problem = {
      type: "https://khalid.dev/problems/forbidden",
      title: "This key cannot do that",
      status: 403,
      detail: "The key lacks subscribers:write.",
      fix: "Sign in as an owner or an admin.",
    };
    redirectsBlog({ "GET /forms/": () => json(403, problem) });
    renderAt("/forms");
    expect(await screen.findByRole("alert")).toHaveTextContent(problem.fix);
  });
});

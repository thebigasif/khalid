import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog, noContent } from "../../test/blog";
import { axeViolations, json, renderAt, serverAnswers } from "../../test/render";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";

const LENA = { account: { id: 1, email: "lena@example.com", name: "Lena", role: "editor" } };
const TITLE = en.signIn.title(en.signIn.thisBlog);

/** A meta the server writes in the admin's page; the test removes it after. */
function named(name: string, content: string) {
  const meta = document.createElement("meta");
  meta.name = name;
  meta.content = content;
  document.head.append(meta);
  return meta;
}

/** Opens the staff token field, which waits behind a small link. */
async function openToken() {
  await userEvent.click(await screen.findByRole("button", { name: en.signIn.token.open }));
  return screen.getByLabelText(en.signIn.token.label);
}

describe("signing in", () => {
  it("sends someone who is signed out to the sign-in screen", async () => {
    renderAt("/");
    expect(await screen.findByRole("heading", { level: 1, name: TITLE })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("keeps an accepted token for the tab and opens the admin", async () => {
    serverAnswers(404);
    renderAt("/sign-in");
    const field = await openToken();
    expect(field).toHaveAttribute("type", "password");
    expect(field).toHaveFocus();
    await userEvent.type(field, "ks_0123_abcd");
    await userEvent.click(screen.getByRole("button", { name: en.signIn.token.submit }));
    const heading = await screen.findByRole("heading", { level: 1, name: en.dashboard.title });
    expect(heading).toBeVisible();
    expect(sessionStorage.getItem("khalid-admin-key")).toBe("ks_0123_abcd");
    expect(await axeViolations()).toEqual([]);
  });

  it("explains a refused token and keeps nothing", async () => {
    serverAnswers(401);
    renderAt("/sign-in");
    await userEvent.type(await openToken(), "wrong");
    await userEvent.click(screen.getByRole("button", { name: en.signIn.token.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.rejected.what);
    expect(sessionStorage.getItem("khalid-admin-key")).toBeNull();
  });

  it("signs in with an email, then signs each request with the session's CSRF token", async () => {
    const sent = adminBlog({ "POST /session/": () => json(200, { ...LENA, csrf: "csrf-test" }) });
    renderAt("/sign-in");
    await userEvent.type(await screen.findByLabelText(en.signIn.email.address), "lena@example.com");
    await userEvent.type(screen.getByLabelText(en.signIn.email.password), "correct horse");
    await userEvent.click(screen.getByRole("button", { name: en.signIn.email.submit }));
    const heading = await screen.findByRole("heading", { level: 1, name: en.dashboard.title });
    expect(heading).toBeVisible();
    expect(await sent[0]?.json()).toEqual({ email: "lena@example.com", password: "correct horse" });
    const later = sent.at(-1);
    expect(later?.headers.get("x-khalid-csrf")).toBe("csrf-test");
    expect(later?.headers.get("Authorization")).toBeNull();
    expect(sessionStorage.getItem("khalid-admin-key")).toBeNull();
  });

  it("shows the API's words for a wrong password, and keeps nothing", async () => {
    const wrong = {
      type: "https://khalid.dev/problems/unauthorized",
      title: "The email and password do not match.",
      status: 401,
      detail: "Khalid found no account with both.",
      fix: "Check both and try again.",
    };
    adminBlog({ "POST /session/": () => json(401, wrong) });
    renderAt("/sign-in");
    await userEvent.type(await screen.findByLabelText(en.signIn.email.address), "lena@example.com");
    await userEvent.type(screen.getByLabelText(en.signIn.email.password), "wrong");
    await userEvent.click(screen.getByRole("button", { name: en.signIn.email.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(wrong.title);
    expect(sessionStorage.getItem("khalid-admin-csrf")).toBeNull();
  });

  it("shows the blog's own name and logo, as the server names them", async () => {
    const name = named("khalid-name", "Field notes");
    const logo = named("khalid-logo", "https://example.com/logo.svg");
    renderAt("/sign-in");
    const title = en.signIn.title("Field notes");
    expect(await screen.findByRole("heading", { level: 1, name: title })).toBeVisible();
    const shown = document.querySelector("img.sign-logo");
    expect(shown).toHaveAttribute("src", "https://example.com/logo.svg");
    expect(screen.getByText(en.signIn.ownerCommand)).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    name.remove();
    logo.remove();
  });

  it("keeps the staff token behind a link, with no command line on the page", async () => {
    renderAt("/sign-in");
    const open = await screen.findByRole("button", { name: en.signIn.token.open });
    expect(open).toHaveAttribute("aria-expanded", "false");
    expect(screen.queryByLabelText(en.signIn.token.label)).toBeNull();
    expect(document.body).not.toHaveTextContent("keys create");
    await openToken();
    expect(open).toHaveAttribute("aria-expanded", "true");
    expect(await axeViolations()).toEqual([]);
  });

  it("ends an email sign-in on the server when signing out", async () => {
    sessionStorage.setItem("khalid-admin-csrf", "csrf-test");
    const sent = adminBlog({ "DELETE /session/": noContent });
    renderAt("/");
    await userEvent.click(await screen.findByRole("button", { name: shellWords.accountMenu }));
    expect(screen.getByText(shellWords.byEmail)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: en.signOut }));
    expect(await screen.findByRole("heading", { level: 1, name: TITLE })).toBeVisible();
    expect(sent.some((request) => request.method === "DELETE")).toBe(true);
    expect(sessionStorage.getItem("khalid-admin-csrf")).toBeNull();
  });

  it("signs out and forgets the token", async () => {
    sessionStorage.setItem("khalid-admin-key", "ks_0123_abcd");
    renderAt("/");
    await userEvent.click(await screen.findByRole("button", { name: shellWords.accountMenu }));
    expect(screen.getByText(shellWords.byToken)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: en.signOut }));
    expect(await screen.findByRole("heading", { level: 1, name: TITLE })).toBeVisible();
    expect(sessionStorage.getItem("khalid-admin-key")).toBeNull();
  });
});

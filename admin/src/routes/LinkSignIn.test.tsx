import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt } from "../../test/render";
import { en } from "../strings/en";

const LENA = { account: { id: 1, email: "lena@example.com", name: "Lena", role: "owner" } };
const WORDS = en.signIn.welcome;

// The options as webauthn-rs sends them for a new passkey.
const OPTIONS = {
  publicKey: {
    rp: { id: "blog.example.com", name: "blog.example.com" },
    user: { id: "AQID", name: "lena@example.com", displayName: "Lena" },
    challenge: "Y2hhbGxlbmdl",
    pubKeyCredParams: [{ type: "public-key", alg: -7 }],
  },
};

// What the browser's toJSON() gives for the new passkey.
const CREDENTIAL = {
  id: "BwgJ",
  rawId: "BwgJ",
  type: "public-key",
  response: { clientDataJSON: "e30", attestationObject: "oA" },
  clientExtensionResults: {},
};

/** This device, with passkeys, making one as `create` says. */
function device(create: (options: CredentialCreationOptions) => Promise<unknown>) {
  const made = vi.fn(create);
  vi.stubGlobal("PublicKeyCredential", class {});
  const credentials = { create: made, get: vi.fn() };
  Object.defineProperty(navigator, "credentials", { value: credentials, configurable: true });
  return made;
}

/** A blog that redeems the link, and takes the passkey it is sent; gives what it was sent. */
function linkBlog() {
  return adminBlog({
    "POST /session/link/redeem/": () => json(200, { ...LENA, csrf: "csrf-link" }),
    "GET /session/": () => json(200, { ...LENA, csrf: "csrf-link" }),
    "POST /session/passkeys/options/": () => json(200, OPTIONS),
    "POST /session/passkeys/": () => new Response(null, { status: 201 }),
  });
}

describe("signing in with a link", () => {
  it("signs in with the token after the #, then offers a passkey for next time", async () => {
    device(async () => null);
    const sent = linkBlog();
    renderAt("/sign-in/link#token=link_test");
    const heading = await screen.findByRole("heading", { level: 1, name: WORDS.title });
    expect(heading).toBeVisible();
    expect(screen.getByText(WORDS.lede)).toBeVisible();
    expect(screen.getByRole("button", { name: WORDS.add })).toHaveClass("primary");
    expect(await sent[0]?.json()).toEqual({ token: "link_test" });
    expect(sessionStorage.getItem("khalid-admin-csrf")).toBe("csrf-link");
    expect(await axeViolations()).toEqual([]);
  });

  it("adds a passkey made on this device, then opens the admin", async () => {
    device(async () => ({ id: "BwgJ", type: "public-key", toJSON: () => CREDENTIAL }));
    const sent = linkBlog();
    renderAt("/sign-in/link#token=link_test");
    await userEvent.click(await screen.findByRole("button", { name: WORDS.add }));
    const done = await screen.findByRole("heading", { level: 1, name: WORDS.addedTitle });
    expect(done).toBeVisible();
    const route = (request: Request) => new URL(request.url).pathname;
    const added = sent.find((request) => route(request).endsWith("/session/passkeys/"));
    const body = { name: WORDS.passkeyName, challenge: "Y2hhbGxlbmdl", credential: CREDENTIAL };
    expect(await added?.json()).toEqual(body);
    await userEvent.click(screen.getByRole("link", { name: WORDS.open }));
    const heading = await screen.findByRole("heading", { level: 1, name: en.dashboard.title });
    expect(heading).toBeVisible();
  });

  it("explains a passkey the device did not make, and keeps the way to a password", async () => {
    device(async () => {
      throw new DOMException("The person cancelled.", "NotAllowedError");
    });
    linkBlog();
    renderAt("/sign-in/link#token=link_test");
    await userEvent.click(await screen.findByRole("button", { name: WORDS.add }));
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(en.passkeys.problems.noPasskey.what);
    await userEvent.click(screen.getByRole("link", { name: WORDS.password }));
    const heading = await screen.findByRole("heading", { level: 1, name: en.account.title });
    expect(heading).toBeVisible();
  });

  it("offers a password in a browser without passkeys", async () => {
    linkBlog();
    renderAt("/sign-in/link#token=link_test");
    const password = await screen.findByRole("link", { name: WORDS.password });
    expect(password).toHaveClass("primary");
    expect(screen.getByText(WORDS.noPasskeys)).toBeVisible();
    expect(screen.queryByRole("button", { name: WORDS.add })).toBeNull();
  });

  it("explains a link that has expired or was used", async () => {
    adminBlog({ "POST /session/link/redeem/": () => json(401, {}) });
    renderAt("/sign-in/link#token=used");
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(en.problems.badLink.what);
    expect(alert).toHaveTextContent("khalid staff owner");
    expect(screen.getByRole("link", { name: en.signIn.link.back })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });
});

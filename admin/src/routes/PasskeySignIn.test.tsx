import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt } from "../../test/render";
import { en } from "../strings/en";

const LENA = { account: { id: 1, email: "lena@example.com", name: "Lena", role: "editor" } };

// The options as webauthn-rs sends them for a sign-in: bytes as unpadded base64url.
const OPTIONS = {
  publicKey: {
    challenge: "Y2hhbGxlbmdl",
    timeout: 300000,
    rpId: "blog.example.com",
    allowCredentials: [],
    userVerification: "preferred",
  },
  mediation: "conditional",
};

// What the browser's toJSON() gives for the passkey it used.
const USED = {
  id: "BwgJ",
  rawId: "BwgJ",
  type: "public-key",
  response: {
    clientDataJSON: "e30",
    authenticatorData: "AA",
    signature: "AQ",
    userHandle: "AQID",
  },
  clientExtensionResults: {},
};

/** This device, with passkeys, answering a sign-in as `get` says. */
function device(get: (options: CredentialRequestOptions) => Promise<unknown>) {
  const used = vi.fn(get);
  vi.stubGlobal("PublicKeyCredential", class {});
  Object.defineProperty(navigator, "credentials", { value: { get: used }, configurable: true });
  return used;
}

/** Presses the passkey button, once the screen shows it. */
async function pressPasskey() {
  await userEvent.click(await screen.findByRole("button", { name: en.signIn.passkey.submit }));
}

describe("signing in with a passkey", () => {
  it("leads with a passkey, then signs in with the one this device picks", async () => {
    const get = device(async () => ({ id: "BwgJ", type: "public-key", toJSON: () => USED }));
    const sent = adminBlog({
      "POST /session/passkey/options/": () => json(200, OPTIONS),
      "POST /session/passkey/": () => json(200, { ...LENA, csrf: "csrf-passkey" }),
    });
    renderAt("/sign-in");
    const passkey = await screen.findByRole("button", { name: en.signIn.passkey.submit });
    const email = screen.getByRole("button", { name: en.signIn.email.submit });
    expect(passkey).toHaveClass("primary");
    expect(email).not.toHaveClass("primary");
    expect(screen.getByText(en.signIn.or)).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(passkey);
    const heading = await screen.findByRole("heading", { level: 1, name: en.dashboard.title });
    expect(heading).toBeVisible();
    const publicKey = get.mock.calls[0]?.[0].publicKey as PublicKeyCredentialRequestOptions;
    expect(new TextDecoder().decode(publicKey.challenge as Uint8Array)).toBe("challenge");
    const signIn = sent.find((request) => new URL(request.url).pathname.endsWith("passkey/"));
    expect(await signIn?.json()).toEqual({ challenge: "Y2hhbGxlbmdl", credential: USED });
    expect(sessionStorage.getItem("khalid-admin-csrf")).toBe("csrf-passkey");
  });

  it("explains a passkey the device did not give, and keeps nothing", async () => {
    device(async () => {
      throw new DOMException("The person cancelled.", "NotAllowedError");
    });
    adminBlog({ "POST /session/passkey/options/": () => json(200, OPTIONS) });
    renderAt("/sign-in");
    await pressPasskey();
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(en.signIn.problems.noPasskey.what);
    expect(sessionStorage.getItem("khalid-admin-csrf")).toBeNull();
  });

  it("says when nobody has added a passkey to this blog yet", async () => {
    const get = device(async () => null);
    adminBlog({ "POST /session/passkey/options/": () => json(404, {}) });
    renderAt("/sign-in");
    await pressPasskey();
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(en.signIn.problems.noPasskeys.what);
    expect(get).not.toHaveBeenCalled();
  });

  it("leads with the email form in a browser without passkeys", async () => {
    renderAt("/sign-in");
    const email = await screen.findByRole("button", { name: en.signIn.email.submit });
    expect(email).toHaveClass("primary");
    expect(screen.queryByRole("button", { name: en.signIn.passkey.submit })).toBeNull();
  });
});

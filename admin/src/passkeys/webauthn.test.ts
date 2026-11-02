import { describe, expect, it, vi } from "vitest";
import { canUsePasskeys, choosePasskey, toBase64Url } from "./webauthn";

// The options as webauthn-rs sends them for a sign-in: bytes as unpadded base64url.
const OPTIONS = {
  publicKey: {
    challenge: "Y2hhbGxlbmdl",
    rpId: "blog.example.com",
    allowCredentials: [{ type: "public-key", id: "BAUG" }],
  },
  mediation: "conditional",
};

/** Bytes for a made-up answer from the device. */
function bytes(...values: number[]): ArrayBuffer {
  return Uint8Array.from(values).buffer;
}

/** This device, with passkeys, answering a sign-in as `get` says. */
function device(get: (options: CredentialRequestOptions) => Promise<unknown>) {
  const used = vi.fn(get);
  vi.stubGlobal("PublicKeyCredential", class {});
  Object.defineProperty(navigator, "credentials", { value: { get: used }, configurable: true });
  return used;
}

describe("signing in with a passkey on this device", () => {
  it("passes the options' bytes as bytes, and asks at once, not from the email field", async () => {
    const get = device(async () => null);
    await choosePasskey(OPTIONS);
    const asked = get.mock.calls[0]?.[0];
    expect(asked?.mediation).toBeUndefined();
    const publicKey = asked?.publicKey as PublicKeyCredentialRequestOptions;
    expect(new TextDecoder().decode(publicKey.challenge as Uint8Array)).toBe("challenge");
    const allowed = publicKey.allowCredentials?.[0]?.id as Uint8Array;
    expect(Array.from(allowed)).toEqual([4, 5, 6]);
  });

  it("builds the sign-in from its buffers in a browser without toJSON", async () => {
    device(async () => ({
      id: "BwgJ",
      rawId: bytes(7, 8, 9),
      type: "public-key",
      response: {
        clientDataJSON: bytes(123, 125),
        authenticatorData: bytes(0),
        signature: bytes(1),
        userHandle: bytes(1, 2, 3),
      },
    }));
    const used = await choosePasskey(OPTIONS);
    expect(used).toEqual({
      challenge: "Y2hhbGxlbmdl",
      credential: {
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
      },
    });
    expect(toBase64Url(bytes(1, 2, 3))).toBe("AQID");
  });

  it("gives nothing when the person cancels, or the options are not a sign-in's", async () => {
    device(async () => {
      throw new DOMException("The person cancelled.", "NotAllowedError");
    });
    expect(await choosePasskey(OPTIONS)).toBeNull();
    expect(await choosePasskey({ mediation: "conditional" })).toBeNull();
  });

  it("knows a browser without passkeys", () => {
    expect(canUsePasskeys()).toBe(false);
    device(async () => null);
    expect(canUsePasskeys()).toBe(true);
  });
});

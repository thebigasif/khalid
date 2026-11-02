import { addPasskey, passkeyOptions } from "../api/passkeys";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { en, type UserMessage } from "../strings/en";
import { makePasskey } from "./webauthn";

/**
 * Makes a passkey on this device and adds it to the signed-in account as
 * `name`. Each step stops at the first problem: the options, the device,
 * the blog. `null` once the blog has it.
 */
export async function addThisDevice(name: string): Promise<UserMessage | null> {
  const options = await passkeyOptions();
  if (!isData(options)) {
    return problemMessage(options.problem, en.problems.unreachable);
  }
  const made = await makePasskey(options.data);
  if (made === null) {
    return en.passkeys.problems.noPasskey;
  }
  const added = await addPasskey(name, made);
  return isData(added) ? null : problemMessage(added.problem, en.problems.unreachable);
}

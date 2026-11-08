import type { SiteChange, SiteSettings } from "../api/site";
import { type PartProps, type Section, SettingsTabs } from "../components/SettingsTabs";
import { Contact } from "../privacy/Contact";
import { Options } from "../privacy/Options";
import { Proof } from "../privacy/Proof";
import { Scripts } from "../privacy/Scripts";
import { Text } from "../privacy/Text";
import { same } from "../seo/parts";
import { privacyWords } from "../strings/privacyWords";

/** What one part of the screen saves: parts of `[privacy]`, the `[code]` list, or both. */
export type PrivacyChange = Pick<SiteChange, "privacy" | "code">;

/** What every part of the screen is given: the settings as read, and how to save. */
export type PrivacyProps = PartProps<PrivacyChange>;

// Each part of the screen, in the order its tabs show.
const SECTIONS: Section<keyof typeof privacyWords.tabs, PrivacyChange>[] = [
  { tab: "settings", show: (props) => <Options {...props} /> },
  { tab: "scripts", show: (props) => <Scripts {...props} /> },
  { tab: "text", show: (props) => <Text {...props} /> },
  { tab: "contact", show: (props) => <Contact {...props} /> },
  { tab: "proof", show: (props) => <Proof {...props} /> },
];

/** Only what `change` changes on `site`, or `null` when it changes nothing. */
function changed(site: SiteSettings, change: PrivacyChange): PrivacyChange | null {
  const now: Record<string, unknown> = { ...site.privacy };
  const parts = Object.entries(change.privacy ?? {}).filter(
    ([name, part]) => !same(part, now[name] ?? null),
  );
  const code =
    change.code !== undefined && !same(change.code, site.code ?? []) ? change.code : undefined;
  if (parts.length === 0 && code === undefined) {
    return null;
  }
  return {
    ...(parts.length === 0 ? {} : { privacy: Object.fromEntries(parts) }),
    ...(code === undefined ? {} : { code }),
  };
}

/** How readers are asked for consent: the `[privacy]` table and the categories of `[code]`. */
export function Privacy() {
  return (
    <SettingsTabs words={privacyWords} sections={SECTIONS} id="privacy-part" changed={changed} />
  );
}

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { type ReactNode, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeSite, readSite, type SiteChange, type SiteSettings } from "../api/site";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { Contact } from "../privacy/Contact";
import { Options } from "../privacy/Options";
import { Scripts } from "../privacy/Scripts";
import { Text } from "../privacy/Text";
import { same } from "../seo/parts";
import type { UserMessage } from "../strings/en";
import { privacyWords } from "../strings/privacyWords";

const SITE = ["site"];

/** What one part of the screen saves: parts of `[privacy]`, the `[code]` list, or both. */
export type PrivacyChange = Pick<SiteChange, "privacy" | "code">;

/** What every part of the screen is given: the settings as read, and how to save. */
export type PrivacyProps = { site: SiteSettings; onSave: (change: PrivacyChange) => Promise<void> };

type Tab = keyof typeof privacyWords.tabs;

// Each part of the screen, in the order its tabs show.
const SECTIONS: { tab: Tab; show: (props: PrivacyProps) => ReactNode }[] = [
  { tab: "settings", show: (props) => <Options {...props} /> },
  { tab: "scripts", show: (props) => <Scripts {...props} /> },
  { tab: "text", show: (props) => <Text {...props} /> },
  { tab: "contact", show: (props) => <Contact {...props} /> },
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
  const words = privacyWords;
  const queries = useQueryClient();
  const read = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const [tab, setTab] = useState<Tab>("settings");
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (site: SiteSettings, change: PrivacyChange) => {
    const sent = changed(site, change);
    if (sent === null) {
      setSaid(words.unchanged);
      setFailed(null);
      return;
    }
    const reply = await changeSite({ updated_at: site.updated_at, ...sent });
    setSaid(isData(reply) ? words.saved : null);
    setFailed(isData(reply) ? null : failureMessage(reply));
    await queries.invalidateQueries({ queryKey: SITE });
  };

  let shown = <p role="status">{words.loading}</p>;
  if (read !== undefined && !isData(read)) {
    shown = <Problem message={failureMessage(read)} />;
  } else if (isData(read)) {
    const site = read.data;
    const section = SECTIONS.find((each) => each.tab === tab);
    shown = (
      <>
        <fieldset className="switch spaced" aria-label={words.sections}>
          {SECTIONS.map((each) => (
            <button
              key={each.tab}
              type="button"
              aria-pressed={each.tab === tab}
              onClick={() => {
                setTab(each.tab);
                setSaid(null);
              }}
            >
              {words.tabs[each.tab]}
            </button>
          ))}
        </fieldset>
        {site.restart_needed ? <p className="careful">{words.restart}</p> : null}
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{said}</p>
        {section === undefined ? null : (
          <section className="focus wide" aria-labelledby="privacy-part">
            <h2 id="privacy-part">{words.tabs[section.tab]}</h2>
            {/* A new copy of site.toml starts the part's fields again from what it holds. */}
            <div key={`${section.tab} ${site.updated_at}`}>
              {section.show({ site, onSave: (change) => save(site, change) })}
            </div>
          </section>
        )}
      </>
    );
  }
  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      {shown}
    </>
  );
}

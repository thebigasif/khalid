import { useQuery, useQueryClient } from "@tanstack/react-query";
import { type ReactNode, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeSite, readSite, type SiteChange, type SiteSettings } from "../api/site";
import type { UserMessage } from "../strings/en";
import { PageHeader } from "./PageHeader";
import { Problem } from "./Problem";

const SITE = ["site"];

/** What every part of a settings screen is given: the settings as read, and how to save. */
export type PartProps<Change> = { site: SiteSettings; onSave: (change: Change) => Promise<void> };

/** One part of a settings screen, and how it shows. */
export type Section<Tab extends string, Change> = {
  tab: Tab;
  show: (props: PartProps<Change>) => ReactNode;
};

/** The words every tabbed settings screen shows, besides its parts' own. */
export type SettingsWords<Tab extends string> = {
  title: string;
  lede: string;
  loading: string;
  sections: string;
  saved: string;
  unchanged: string;
  restart: string;
  tabs: Record<Tab, string>;
};

type Props<Tab extends string, Change> = {
  words: SettingsWords<Tab>;
  /** Each part of the screen, in the order its tabs show; the first opens. */
  sections: Section<Tab, Change>[];
  /** The id of the open part's heading, unique on the page. */
  id: string;
  /** Only what `change` changes on `site`, or `null` when it changes nothing. */
  changed: (site: SiteSettings, change: Change) => Omit<SiteChange, "updated_at"> | null;
};

/** A settings screen of site.toml in tabs, one part at a time, each saved on its own. */
export function SettingsTabs<Tab extends string, Change>({
  words,
  sections,
  id,
  changed,
}: Props<Tab, Change>) {
  const queries = useQueryClient();
  const read = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const [tab, setTab] = useState(sections[0]?.tab);
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (site: SiteSettings, change: Change) => {
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
    const section = sections.find((each) => each.tab === tab);
    shown = (
      <>
        <fieldset className="switch spaced" aria-label={words.sections}>
          {sections.map((each) => (
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
          <section className="focus wide" aria-labelledby={id}>
            <h2 id={id}>{words.tabs[section.tab]}</h2>
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

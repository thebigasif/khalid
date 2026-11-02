import { useQuery, useQueryClient } from "@tanstack/react-query";
import { type ReactNode, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeSite, readSite, type SiteSettings } from "../api/site";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { type Part, partOf, same } from "../seo/parts";
import { SearchAppearance } from "../seo/SearchAppearance";
import type { UserMessage } from "../strings/en";
import { seoWords } from "../strings/seoWords";

const SITE = ["site"];

/** Saves some parts of `[seo]`, each replacing its part whole; `null` removes one. */
export type SaveParts = (parts: Record<string, Part | null>) => Promise<void>;

/** What every part of the screen is given: the settings as read, and how to save. */
export type SectionProps = { site: SiteSettings; onSave: SaveParts };

type Tab = keyof typeof seoWords.tabs;

// Each part of the screen, in the order its tabs show.
const SECTIONS: { tab: Tab; show: (props: SectionProps) => ReactNode }[] = [
  { tab: "appearance", show: (props) => <SearchAppearance {...props} /> },
];

/** Every `[seo]` setting of site.toml, one part at a time, each saved on its own. */
export function SeoSettings() {
  const words = seoWords;
  const queries = useQueryClient();
  const read = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const [tab, setTab] = useState<Tab>("appearance");
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (site: SiteSettings, parts: Record<string, Part | null>) => {
    const now = (name: string) => (site.seo?.[name] === undefined ? null : partOf(site.seo, name));
    const changed = Object.entries(parts).filter(([name, part]) => !same(part, now(name)));
    if (changed.length === 0) {
      setSaid(words.unchanged);
      setFailed(null);
      return;
    }
    const seo = Object.fromEntries(changed);
    const reply = await changeSite({ updated_at: site.updated_at, seo });
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
          <section className="focus wide" aria-labelledby="seo-part">
            <h2 id="seo-part">{words.tabs[section.tab]}</h2>
            {/* A new copy of site.toml starts the part's fields again from what it holds. */}
            <div key={`${section.tab} ${site.updated_at}`}>
              {section.show({ site, onSave: (parts) => save(site, parts) })}
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

import type { SiteSettings } from "../api/site";
import { type PartProps, type Section, SettingsTabs } from "../components/SettingsTabs";
import { Breadcrumbs } from "../seo/Breadcrumbs";
import { Crawlers } from "../seo/Crawlers";
import { Feeds } from "../seo/Feeds";
import { type Part, partOf, same } from "../seo/parts";
import { Representation } from "../seo/Representation";
import { SearchAppearance } from "../seo/SearchAppearance";
import { ShowInSearch } from "../seo/ShowInSearch";
import { SiteSearch } from "../seo/SiteSearch";
import { SocialSharing } from "../seo/SocialSharing";
import { Webmaster } from "../seo/Webmaster";
import { seoWords } from "../strings/seoWords";

type Parts = Record<string, Part | null>;

/** Saves some parts of `[seo]`, each replacing its part whole; `null` removes one. */
export type SaveParts = (parts: Parts) => Promise<void>;

/** What every part of the screen is given: the settings as read, and how to save. */
export type SectionProps = PartProps<Parts>;

// Each part of the screen, in the order its tabs show.
const SECTIONS: Section<keyof typeof seoWords.tabs, Parts>[] = [
  { tab: "appearance", show: (props) => <SearchAppearance {...props} /> },
  { tab: "index", show: (props) => <ShowInSearch {...props} /> },
  { tab: "representation", show: (props) => <Representation {...props} /> },
  { tab: "social", show: (props) => <SocialSharing {...props} /> },
  { tab: "webmaster", show: (props) => <Webmaster {...props} /> },
  { tab: "crawlers", show: (props) => <Crawlers {...props} /> },
  { tab: "feeds", show: (props) => <Feeds {...props} /> },
  { tab: "breadcrumbs", show: (props) => <Breadcrumbs {...props} /> },
  { tab: "search", show: (props) => <SiteSearch {...props} /> },
  // TODO(Yoast WP2): llms.txt, the schema files and local business get a
  // tab here; docs/seo-settings.md lists them.
];

/** Only the parts that differ from `site`, or `null` when none does. */
function changed(site: SiteSettings, parts: Parts) {
  const now = (name: string) => (site.seo?.[name] === undefined ? null : partOf(site.seo, name));
  const seo = Object.entries(parts).filter(([name, part]) => !same(part, now(name)));
  return seo.length === 0 ? null : { seo: Object.fromEntries(seo) };
}

/** Every `[seo]` setting of site.toml, one part at a time, each saved on its own. */
export function SeoSettings() {
  return <SettingsTabs words={seoWords} sections={SECTIONS} id="seo-part" changed={changed} />;
}

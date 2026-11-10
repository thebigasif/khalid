import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeSite, readSite, type SiteNav, type SiteSettings } from "../api/site";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { savedWords } from "../components/SettingsTabs";
import { MenuEditor } from "../site/MenuEditor";
import type { UserMessage } from "../strings/en";
import { navWords as words } from "../strings/navWords";

const SITE = ["site"];

/** Both menus as `site` holds them, changed on the screen and saved together. */
function Menus({ site, onSave }: { site: SiteSettings; onSave: (nav: SiteNav) => void }) {
  const [primary, setPrimary] = useState(site.nav?.primary ?? []);
  const [footer, setFooter] = useState(site.nav?.footer ?? []);
  return (
    <>
      <MenuEditor menu="primary" links={primary} languages={site.languages} setLinks={setPrimary} />
      <MenuEditor menu="footer" links={footer} languages={site.languages} setLinks={setFooter} />
      <button type="button" className="btn primary" onClick={() => onSave({ primary, footer })}>
        {words.save}
      </button>
    </>
  );
}

/** The header and footer menus, saved to `site.toml`. */
export function Navigation() {
  const queries = useQueryClient();
  const read = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (updated_at: string, nav: SiteNav) => {
    const reply = await changeSite({ updated_at, nav });
    setSaid(isData(reply) ? savedWords(words, reply.data) : null);
    setFailed(isData(reply) ? null : failureMessage(reply));
    await queries.invalidateQueries({ queryKey: SITE });
  };

  let shown = <p role="status">{words.loading}</p>;
  if (read !== undefined && !isData(read)) {
    shown = <Problem message={failureMessage(read)} />;
  } else if (isData(read)) {
    const site = read.data;
    shown = (
      <>
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{said}</p>
        <Menus key={site.updated_at} site={site} onSave={(nav) => save(site.updated_at, nav)} />
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

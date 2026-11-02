import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeSite, readSite } from "../api/site";
import { readTemplates, type Template } from "../api/templates";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { LookForm } from "../look/LookForm";
import { lookChange, startOf, type Values } from "../look/lookChange";
import { en, type UserMessage } from "../strings/en";

// The Look screen shares the settings with the Settings screen, so a save
// here shows there at once.
const SITE = ["site"];
const TEMPLATES = ["templates"];

/** The blog's template and that template's options, saved to `site.toml`. */
export function Look() {
  const words = en.look;
  const queries = useQueryClient();
  const site = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const templates = useQuery({ queryKey: TEMPLATES, queryFn: readTemplates }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  let shown = <p role="status">{words.loading}</p>;
  if (site !== undefined && !isData(site)) {
    shown = <Problem message={failureMessage(site)} />;
  } else if (templates !== undefined && !isData(templates)) {
    shown = <Problem message={failureMessage(templates)} />;
  } else if (isData(site) && isData(templates)) {
    const settings = site.data;
    const start = startOf(templates.data, settings);
    const save = async (chosen: Template, values: Values) => {
      const change = lookChange(settings, chosen, values);
      if (change === null) {
        setSaid(words.unchanged);
        return;
      }
      const reply = await changeSite(change);
      setSaid(isData(reply) ? words.saved : null);
      setFailed(isData(reply) ? null : failureMessage(reply));
      await queries.invalidateQueries({ queryKey: SITE });
    };
    shown = (
      <>
        {settings.restart_needed ? <p className="careful">{words.restart}</p> : null}
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{said}</p>
        {start === undefined ? null : (
          <LookForm
            key={settings.updated_at}
            templates={templates.data}
            start={start}
            site={settings}
            onSave={save}
          />
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

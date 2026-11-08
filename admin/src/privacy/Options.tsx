import { useState } from "react";
import type { PrivacyProps } from "../routes/Privacy";
import { Check, SectionForm, TextField } from "../seo/Fields";
import { privacyWords } from "../strings/privacyWords";
import { DEFAULT_MONTHS, type GpcMode, given } from "./parts";

const GPC_MODES: GpcMode[] = ["marketing", "reject-all"];

/** How long a choice lasts, what GPC turns off, counting, and an outside consent tool. */
export function Options({ site, onSave }: PrivacyProps) {
  const words = privacyWords.settings;
  const table = site.privacy ?? {};
  const isStatic = site.mode === "static";
  const [months, setMonths] = useState(String(table.lifetime_months ?? DEFAULT_MONTHS));
  const [gpc, setGpc] = useState<GpcMode>(table.gpc ?? "marketing");
  const [counts, setCounts] = useState(table.counts ?? !isStatic);
  const [usesTool, setUsesTool] = useState(table.external_tool !== undefined);
  const [tool, setTool] = useState(table.external_tool ?? "");

  // A value equal to the default is left out, so site.toml stays short.
  const save = () => {
    const lifetime = Number(months);
    const named = usesTool ? given(tool) : undefined;
    return onSave({
      privacy: {
        lifetime_months: lifetime === DEFAULT_MONTHS ? null : lifetime,
        gpc: gpc === "marketing" ? null : gpc,
        counts: counts === !isStatic ? null : counts,
        external_tool: named ?? null,
      },
    });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <TextField
        id="privacy-months"
        type="number"
        label={words.months}
        hint={words.monthsHint}
        value={months}
        onChange={setMonths}
      />
      <fieldset aria-describedby="privacy-gpc-hint">
        <legend>{words.gpc}</legend>
        <span className="hint" id="privacy-gpc-hint">
          {words.gpcHint}
        </span>
        {GPC_MODES.map((mode) => {
          const id = `privacy-gpc-${mode}`;
          const reject = mode === "reject-all";
          return (
            <div className="choice" key={mode}>
              <input
                id={id}
                type="radio"
                name="privacy-gpc"
                checked={gpc === mode}
                aria-describedby={`${id}-hint`}
                onChange={() => setGpc(mode)}
              />
              <div>
                <label htmlFor={id}>{reject ? words.gpcRejectAll : words.gpcMarketing}</label>
                <span className="hint" id={`${id}-hint`}>
                  {reject ? words.gpcRejectAllHint : words.gpcMarketingHint}
                </span>
              </div>
            </div>
          );
        })}
      </fieldset>
      {isStatic ? (
        <p className="hint">{words.countsStatic}</p>
      ) : (
        <Check
          id="privacy-counts"
          label={words.counts}
          hint={words.countsHint}
          checked={counts}
          onChange={setCounts}
        />
      )}
      <Check
        id="privacy-tool"
        label={words.tool}
        hint={words.toolHint}
        checked={usesTool}
        onChange={setUsesTool}
      />
      {usesTool ? (
        <TextField
          id="privacy-tool-name"
          label={words.toolName}
          hint={words.toolNameHint}
          value={tool}
          onChange={setTool}
        />
      ) : null}
    </SectionForm>
  );
}

import type { ChangeEvent } from "react";
import { categoryName, useCategories } from "../api/categories";
import { reachWords } from "../strings/reachWords";
import type { Access, ReachDraft } from "./reach";
import { SettingField } from "./SettingField";
import type { SettingsDraft } from "./settings";

type Props = {
  settings: SettingsDraft;
  onSetting: <K extends keyof SettingsDraft>(key: K, value: SettingsDraft[K]) => void;
  onBlur: () => void;
};

type Typed = ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>;

const ACCESS: readonly Access[] = ["public", "members"];

/** The text field for `key`: its id, value, hint and handlers. */
function bind(key: keyof ReachDraft & string, value: string, props: Props) {
  return {
    id: `setting-${key}`,
    value,
    "aria-describedby": `setting-${key}-hint`,
    onChange: (event: Typed) => {
      const text = event.target.value;
      if (key === "access") {
        props.onSetting("access", ACCESS.find((each) => each === text) ?? "public");
      } else if (key !== "featured" && key !== "passwordOn") {
        props.onSetting(key, text);
      }
    },
    onBlur: props.onBlur,
  };
}

/** A tick box for `key`, with its hint under the label. */
function Tick({ name, props }: { name: "featured" | "passwordOn"; props: Props }) {
  const id = `setting-${name}`;
  return (
    <div className="field">
      <label className="choice">
        <input
          id={id}
          type="checkbox"
          checked={props.settings[name]}
          aria-describedby={`${id}-hint`}
          onChange={(event) => props.onSetting(name, event.target.checked)}
          onBlur={props.onBlur}
        />
        <span>{reachWords[name]}</span>
      </label>
      <span className="hint" id={`${id}-hint`}>
        {reachWords[`${name}Hint`]}
      </span>
    </div>
  );
}

/** Where the post is filed: its tags, its category and whether it is featured. */
export function FilingFields(props: Props) {
  const categories = useCategories();
  const { settings } = props;
  const words = reachWords;
  // A category the list does not hold yet still shows, so a save never drops it.
  const known = categories.some((category) => category.slug === settings.category);
  return (
    <>
      <SettingField id="setting-tags" label={words.tags} hint={words.tagsHint}>
        <input {...bind("tags", settings.tags, props)} />
      </SettingField>
      <SettingField id="setting-category" label={words.category} hint={words.categoryHint}>
        <select {...bind("category", settings.category, props)}>
          <option value="">{words.noCategory}</option>
          {categories.map((category) => (
            <option key={category.slug} value={category.slug}>
              {categoryName(category)}
            </option>
          ))}
          {known || settings.category === "" ? null : (
            <option value={settings.category}>{settings.category}</option>
          )}
        </select>
      </SettingField>
      <Tick name="featured" props={props} />
    </>
  );
}

/** Who may read the post and how it is shown, folded away: most posts leave these alone. */
export function ReaderFields(props: Props) {
  const { settings } = props;
  const words = reachWords;
  return (
    <details>
      <summary>{words.title}</summary>
      <div>
        <SettingField id="setting-access" label={words.access} hint={words.accessHint}>
          <select {...bind("access", settings.access, props)}>
            {ACCESS.map((access) => (
              <option key={access} value={access}>
                {words.accessChoices[access]}
              </option>
            ))}
          </select>
        </SettingField>
        <Tick name="passwordOn" props={props} />
        {settings.passwordOn ? (
          <SettingField id="setting-newPassword" label={words.password} hint={words.passwordHint}>
            <input
              {...bind("newPassword", settings.newPassword, props)}
              type="password"
              autoComplete="new-password"
            />
          </SettingField>
        ) : null}
        <SettingField id="setting-template" label={words.template} hint={words.templateHint}>
          <input {...bind("template", settings.template, props)} spellCheck={false} />
        </SettingField>
        <SettingField id="setting-headCode" label={words.headCode} hint={words.headCodeHint}>
          <textarea {...bind("headCode", settings.headCode, props)} rows={3} spellCheck={false} />
        </SettingField>
      </div>
    </details>
  );
}

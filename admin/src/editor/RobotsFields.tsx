import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";
import {
  IMAGE_PREVIEWS,
  type ImagePreview,
  parseRobots,
  RESTRICTIONS,
  type Robots,
  robotsText,
} from "./robots";
import { SettingField } from "./SettingField";

type Props = {
  /** The post's robots setting, empty for the blog's. */
  value: string;
  onChange: (value: string) => void;
  onBlur: () => void;
};

/** The main choice, whether search engines may list the post, as the select shows it. */
function visibility(robots: Robots): string {
  if (!robots.noindex) {
    return "";
  }
  return robots.nofollow ? "noindex, nofollow" : "noindex";
}

/** Whether search engines may list the post, then the rarer choices, folded away. */
export function RobotsFields({ value, onChange, onBlur }: Props) {
  const words = seoEditorWords;
  const field = en.settings.more.fields.robots;
  const robots = parseRobots(value);
  const set = (changed: Partial<Robots>) => onChange(robotsText({ ...robots, ...changed }));
  const choose = (picked: string) =>
    set({ noindex: picked.startsWith("noindex"), nofollow: picked.endsWith("nofollow") });
  const turn = (restriction: (typeof RESTRICTIONS)[number], on: boolean) => {
    const others = robots.restrictions.filter((each) => each !== restriction);
    set({ restrictions: on ? [...others, restriction] : others });
  };
  const pick = (size: string) =>
    set({ maxImagePreview: IMAGE_PREVIEWS.find((each) => each === size) ?? "" });
  return (
    <>
      <SettingField id="setting-robots" label={field.label} hint={field.hint}>
        <select
          id="setting-robots"
          aria-describedby="setting-robots-hint"
          value={visibility(robots)}
          onChange={(event) => choose(event.target.value)}
          onBlur={onBlur}
        >
          {en.settings.more.robots.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </SettingField>
      <details>
        <summary>{words.advanced}</summary>
        <div>
          <p className="hint">{words.advancedHint}</p>
          {RESTRICTIONS.map((restriction) => (
            <div className="choice" key={restriction}>
              <input
                id={`robots-${restriction}`}
                type="checkbox"
                checked={robots.restrictions.includes(restriction)}
                onChange={(event) => turn(restriction, event.target.checked)}
                onBlur={onBlur}
              />
              <label htmlFor={`robots-${restriction}`}>{words.restrictions[restriction]}</label>
            </div>
          ))}
          <SettingField id="robots-snippet" label={words.maxSnippet} hint={words.maxSnippetHint}>
            <input
              id="robots-snippet"
              type="number"
              min={0}
              aria-describedby="robots-snippet-hint"
              value={robots.maxSnippet}
              onChange={(event) => set({ maxSnippet: event.target.value })}
              onBlur={onBlur}
            />
          </SettingField>
          <SettingField id="robots-image" label={words.maxImage} hint={words.maxImageHint}>
            <select
              id="robots-image"
              aria-describedby="robots-image-hint"
              value={robots.maxImagePreview}
              onChange={(event) => pick(event.target.value)}
              onBlur={onBlur}
            >
              {IMAGE_PREVIEWS.map((size: ImagePreview) => (
                <option key={size} value={size}>
                  {words.imageSizes[size]}
                </option>
              ))}
            </select>
          </SettingField>
        </div>
      </details>
    </>
  );
}

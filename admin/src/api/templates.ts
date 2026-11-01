// The template list of the Admin API, `GET /templates/`. Typed by hand,
// field for field as `AdminTemplate` in khalid-api, as the settings routes
// are in site.ts. The list comes from the running Khalid's engine, so the
// admin never names a template itself.
import { call, type Reply } from "./call";

/** One option's value as `site.toml` holds it. */
export type TemplateValue = boolean | number | string | string[];

/** The values one option takes, and the one it has when the site sets none. */
export type OptionTakes =
  | { kind: "switch"; default: boolean }
  | { kind: "choice"; allowed: string[]; default: string }
  | { kind: "count"; least: number; most: number; default: number }
  | { kind: "line"; most: number }
  | { kind: "lines"; most: number }
  | { kind: "picks"; allowed: string[]; default: string[] };

/** One option a template takes, with one plain sentence of help. */
export type TemplateOption = { key: string; help: string; takes: OptionTakes };

/** One built-in template. */
export type Template = {
  /** The name `[template]` uses, such as `classic`. */
  name: string;
  summary: string;
  /** The reader features it turns on or off. */
  reader: { key: string; on: boolean }[];
  options: TemplateOption[];
};

/** The template a site uses and the options it sets; the rest take their defaults. */
export type TemplateSetting = { name: string; options: Record<string, TemplateValue> };

async function templates(response: Response): Promise<Template[]> {
  const body: { templates: Template[] } = await response.json();
  return body.templates;
}

/** Every built-in template, classic first. */
export function readTemplates(): Promise<Reply<Template[]>> {
  return call("/templates/", {}, templates);
}

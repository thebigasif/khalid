// The template list of the Admin API, `GET /templates/`, typed from the
// OpenAPI document. The list comes from the running Khalid's engine, so
// the admin never names a template itself.
import { call, type Reply } from "./call";
import type { components } from "./schema";

type Schemas = components["schemas"];

/** One option's value as `site.toml` holds it. */
export type TemplateValue = Schemas["TemplateValue"];

/** The values one option takes, and the one it has when the site sets none. */
export type OptionTakes = Schemas["OptionTakes"];

/** One option a template takes, with one plain sentence of help. */
export type TemplateOption = Schemas["TemplateOptionSpec"];

/** One built-in template. */
export type Template = Schemas["AdminTemplate"];

/** The template a site uses and the options it sets; the rest take their defaults. */
export type TemplateSetting = Required<Schemas["TemplateSetting"]>;

async function templates(response: Response): Promise<Template[]> {
  const body: Schemas["TemplateList"] = await response.json();
  return body.templates;
}

/** Every built-in template, classic first. */
export function readTemplates(): Promise<Reply<Template[]>> {
  return call("/templates/", {}, templates);
}

// What the Look screen works out without the network: each option's value
// with the template's default filled in, what a template turns on and
// off, and the change to send, holding only the options that differ from
// the defaults so site.toml stays short.
import type { SiteChange, SiteSettings } from "../api/site";
import type { OptionTakes, Template, TemplateValue } from "../api/templates";

/** Every option of one template, by key. */
export type Values = Record<string, TemplateValue>;

/** The template in use, or the first one when this server has no such template. */
export function startOf(templates: Template[], site: SiteSettings): Template | undefined {
  return templates.find((template) => template.name === site.template.name) ?? templates[0];
}

/** The value an option has when the site sets none. */
export function defaultOf(takes: OptionTakes): TemplateValue {
  if (takes.kind === "lines") {
    return [];
  }
  return takes.kind === "line" ? "" : takes.default;
}

/** Every option of `template`, as `written` sets it or at its default. */
export function valuesOf(template: Template, written: Values): Values {
  const values: Values = {};
  for (const option of template.options) {
    values[option.key] = written[option.key] ?? defaultOf(option.takes);
  }
  return values;
}

/** Whether two values are the same, lists item by item. */
function same(one: TemplateValue | undefined, other: TemplateValue | undefined): boolean {
  return JSON.stringify(one) === JSON.stringify(other);
}

/** `values` as sent: lines without the blank ones, and only what differs from the default. */
export function setOnly(template: Template, values: Values): Values {
  const set: Values = {};
  for (const option of template.options) {
    let value = values[option.key];
    if (option.takes.kind === "lines" && Array.isArray(value)) {
      value = value.map((line) => line.trim()).filter((line) => line !== "");
    } else if (option.takes.kind === "line" && typeof value === "string") {
      value = value.trim();
    }
    if (value !== undefined && !same(value, defaultOf(option.takes))) {
      set[option.key] = value;
    }
  }
  return set;
}

/** The change that saves `chosen` with `values`, or null when it is what the site has. */
export function lookChange(site: SiteSettings, chosen: Template, values: Values) {
  const options = setOnly(chosen, values);
  const before = setOnly(chosen, valuesOf(chosen, site.template.options));
  const unchanged =
    site.template.name === chosen.name &&
    chosen.options.every((option) => same(options[option.key], before[option.key]));
  const change: SiteChange = {
    updated_at: site.updated_at,
    template: { name: chosen.name, options },
  };
  return unchanged ? null : change;
}

/** The reader features and switch options a template turns on, and the features it turns off. */
export function turns(template: Template): { on: string[]; off: string[] } {
  const switchedOn = template.options
    .filter((option) => option.takes.kind === "switch" && option.takes.default)
    .map((option) => option.key);
  const reader = (on: boolean) =>
    template.reader.filter((feature) => feature.on === on).map((feature) => feature.key);
  return { on: [...reader(true), ...switchedOn], off: reader(false) };
}

/** A key or name as words: `card_reading_time` reads "Card reading time". */
export function wordsOf(key: string): string {
  const words = key.replace(/[_-]+/g, " ").trim();
  return words.charAt(0).toUpperCase() + words.slice(1);
}

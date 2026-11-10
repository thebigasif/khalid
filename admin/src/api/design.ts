// The Design route of the Admin API, `/design/`. Typed by hand, field for
// field as `AdminDesign` and `DesignChange` in khalid-api, as the settings
// routes are in site.ts. The answer wraps one object, `{ "design": ... }`.
import { call, type Reply, sending } from "./call";

/** One font a body or heading may use. */
export type FontChoice = {
  /** The name a person reads, such as `System font` or `Inter`. */
  label: string;
  /** The CSS font list the file writes for it. */
  stack: string;
};

/** One colour on one ground, with its WCAG contrast. */
export type ContrastPair = {
  /** The colour drawn, such as `colors.link`. */
  token: string;
  /** `colors.background` or `colors.surface`. */
  ground: string;
  /** Such as 6.3 for 6.3 to 1, rounded down. */
  ratio: number;
  /** 4.5 for text, 3 for buttons and highlights. */
  target: number;
  passes: boolean;
  /** For a colour that fails, the nearest shade that passes on both grounds. */
  nearest: string | null;
};

/** The colours, fonts, shape and logo the blog's pages use. */
export type Design = {
  /** Every token by name, such as `colors.link`; a logo not set is "". */
  tokens: Record<string, string>;
  /** The tokens `khalid theme sync` keeps as they are. */
  locked: string[];
  fonts: FontChoice[];
  contrast: ContrastPair[];
  /** When `theme/tokens.toml` last changed; `null` when there is no file yet. */
  updated_at: string | null;
};

/** A change: only the tokens sent change. */
export type DesignChange = {
  updated_at: string | null;
  tokens?: Record<string, string>;
  /** The whole list of tokens a sync keeps, when it changes. */
  locked?: string[];
};

async function design(response: Response): Promise<Design> {
  const body: { design: Design } = await response.json();
  return body.design;
}

/** The design as the blog's pages use it now. */
export function readDesign(): Promise<Reply<Design>> {
  return call("/design/", {}, design);
}

/** Saves a change made on the copy that `change.updated_at` names. */
export function changeDesign(change: DesignChange): Promise<Reply<Design>> {
  return call("/design/", sending("PUT", { design: change }), design);
}

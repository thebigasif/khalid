import type { Editor } from "@tiptap/core";
import type { Node as ProseMirrorNode } from "@tiptap/pm/model";
import { type EditorState, NodeSelection } from "@tiptap/pm/state";
import { en } from "../../strings/en";
import { nodeName } from "../blocks/directiveNode";
import { BLOCK_SPECS, type BlockSpec } from "../blocks/specs";

/** A block's details in the form: a new block at `at`, or the block there. */
export type Details = {
  spec: BlockSpec;
  values: Record<string, string>;
  at: number;
  editing: boolean;
};

const labels: Record<string, string> = en.fields;
const blockLabels: Record<string, Record<string, string>> = en.blockFields;

/** The label of `field` in the form of block `block`, if the words have one. */
export function labelOf(block: string, field: string): string | undefined {
  return blockLabels[block]?.[field] ?? labels[field];
}

/** The details that take one of a few words, the default first. */
const CHOICES: Record<string, readonly string[]> = {
  kind: ["note", "tip", "warning"],
  show: ["everyone", "visitors", "members"],
  "chart.type": ["column", "bar", "line"],
  "event.status": ["scheduled", "cancelled", "postponed", "rescheduled", "moved-online"],
};

/** The words `field` of block `block` takes, or `undefined` when it takes any text. */
export function choicesOf(block: string, field: string): readonly string[] | undefined {
  return CHOICES[`${block}.${field}`] ?? CHOICES[field];
}

/** The fields a block's form asks for: its attributes, then its text if it holds any. */
export function fieldsOf(spec: BlockSpec): string[] {
  return spec.leaf ? [...spec.attributes] : [...spec.attributes, "body"];
}

/** A new block that opens its form first, because it has something to fill in. */
export function newDetails(spec: BlockSpec, at: number): Details | null {
  const fields = fieldsOf(spec);
  if (fields.length === 0) {
    return null;
  }
  const values = Object.fromEntries(fields.map((field) => [field, ""]));
  return { spec, values, at, editing: false };
}

/** The details of the directive block selected in the text, if one is. */
export function selectedDetails(state: EditorState): Details | null {
  const { selection } = state;
  return selection instanceof NodeSelection ? detailsOf(selection.node, selection.from) : null;
}

/** The details of `node`, sitting at `at`, if it is a directive block with any. */
export function detailsOf(node: ProseMirrorNode, at: number): Details | null {
  const spec = BLOCK_SPECS.find((candidate) => nodeName(candidate) === node.type.name);
  if (spec === undefined || fieldsOf(spec).length === 0) {
    return null;
  }
  const values = Object.fromEntries(
    fieldsOf(spec).map((field) => {
      const value: unknown = node.attrs[field];
      return [field, typeof value === "string" ? value : ""];
    }),
  );
  return { spec, values, at, editing: true };
}

/** The form's values as node attributes: an empty field writes nothing. */
function attributes(values: Record<string, string>): Record<string, string | null> {
  return Object.fromEntries(
    Object.entries(values).map(([field, value]) => {
      const kept = field === "body" ? value.trimEnd() : value.trim();
      return [field, kept === "" ? null : kept];
    }),
  );
}

/** Adds the block the form describes, or changes the block it was opened on. */
export function applyDetails(editor: Editor, details: Details, values: Record<string, string>) {
  const attrs = attributes(values);
  if (!details.editing) {
    const block = { type: nodeName(details.spec), attrs };
    editor.chain().focus().insertContentAt(details.at, block).run();
    return;
  }
  editor
    .chain()
    .focus()
    .command(({ tr }) => {
      const node = tr.doc.nodeAt(details.at);
      if (node === null) {
        return false;
      }
      tr.setNodeMarkup(details.at, undefined, { ...node.attrs, ...attrs });
      return true;
    })
    .run();
}

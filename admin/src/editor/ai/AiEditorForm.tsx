import { useRef } from "react";
import { createPortal } from "react-dom";
import type { AdminPost } from "../../api/posts";
import { EditorForm as Form } from "../EditorForm";
import { type SuggestionRequest, useAiReady } from "./api";
import { fillField, helps, paragraph, textField, withBlock } from "./fields";
import { SuggestBox } from "./SuggestBox";
import { slot, useSlots } from "./slots";
import { aiWords, type SuggestField } from "./words";
import "./ai.css";

type Props = { post: AdminPost };

const TASKS: Record<SuggestField, SuggestionRequest["task"]> = {
  seoTitle: "seo_title",
  excerpt: "meta_description",
  xTitle: "share_title",
  xDescription: "share_description",
  facebookTitle: "share_title",
  facebookDescription: "share_description",
};

const FIELDS = Object.keys(TASKS) as SuggestField[];

type Place =
  | { kind: "field"; host: HTMLElement; field: SuggestField }
  | { kind: "fix"; host: HTMLElement; what: string; message: string }
  | { kind: "summary"; host: HTMLElement };

/** Where the AI buttons go: beside each field, under each check about words, and in the bar. */
function places(root: HTMLElement): Place[] {
  const found: Place[] = [];
  const bar = root.querySelector(".bar");
  if (bar !== null) {
    found.push({ kind: "summary", host: slot(bar, "summarize") });
  }
  for (const field of FIELDS) {
    const box = textField(root, `setting-${field}`)?.closest(".field");
    if (box) {
      found.push({ kind: "field", host: slot(box, "suggest"), field });
    }
  }
  for (const check of root.querySelectorAll("section.check")) {
    const what = paragraph(check, 1);
    if (helps(what)) {
      const message = [1, 2, 3].map((n) => paragraph(check, n)).join(" ");
      found.push({ kind: "fix", host: slot(check, "fix"), what, message });
    }
  }
  return found;
}

/** Adds `block` at the end of the post's Markdown, switching to Markdown first if need be. */
function addBlock(root: HTMLElement, block: string) {
  const source = () => root.querySelector("textarea.source");
  const add = () => {
    const text = source();
    if (text instanceof HTMLTextAreaElement) {
      fillField(text, withBlock(text.value, block));
      return true;
    }
    return false;
  };
  if (add()) {
    return;
  }
  const markdown = root.querySelector('.switch button[aria-pressed="false"]');
  if (markdown instanceof HTMLButtonElement) {
    markdown.click();
  }
  let tries = 0;
  const wait = () => {
    tries += 1;
    if (!add() && tries < 30) {
      requestAnimationFrame(wait);
    }
  };
  requestAnimationFrame(wait);
}

/**
 * The editor with AI suggestions beside it, when the owner turned them on:
 * Suggest beside the SEO title, description and share fields, Suggest a fix
 * under each check about words, and Summarize in the bar.
 */
export function EditorForm({ post }: Props) {
  const ready = useAiReady();
  const root = useRef<HTMLDivElement>(null);
  const shown = useSlots(root, ready, places);
  const value = (id: string) => textField(root.current ?? document, id)?.value.trim() || null;
  // The saved post's words, with the title, keyword and Markdown on screen
  // winning, so a suggestion reads what the writer sees.
  const words = (): Omit<SuggestionRequest, "task"> => {
    const source = root.current?.querySelector("textarea.source");
    return {
      post: post.id,
      title: value("title"),
      focus_keyword: value("setting-focusKeyword"),
      text: source instanceof HTMLTextAreaElement ? source.value : null,
    };
  };
  const fillWith = (id: string) => (text: string) => {
    const field = textField(root.current ?? document, id);
    if (field !== null) {
      fillField(field, text);
    }
  };
  const box = (place: Place) => {
    if (place.kind === "field") {
      const name = aiWords.suggestField(aiWords.fields[place.field]);
      const ask = () => ({ ...words(), task: TASKS[place.field] });
      const pick = { kind: "use", onUse: fillWith(`setting-${place.field}`) } as const;
      return <SuggestBox label={aiWords.suggest} name={name} ask={ask} pick={pick} />;
    }
    if (place.kind === "fix") {
      const finding = { rule: place.what, message: place.message };
      const ask = () => ({ ...words(), task: "lint_fix" as const, finding });
      const name = aiWords.fixLabel(place.what);
      return <SuggestBox label={aiWords.fix} name={name} ask={ask} pick={{ kind: "copy" }} />;
    }
    const ask = () => ({ ...words(), task: "takeaways" as const });
    const onAdd = (block: string) => (root.current ? addBlock(root.current, block) : undefined);
    const pick = { kind: "block", onAdd } as const;
    const name = aiWords.summarizeLabel;
    return <SuggestBox label={aiWords.summarize} name={name} ask={ask} pick={pick} />;
  };
  return (
    <div ref={root} className="ai-host">
      <Form post={post} />
      {shown.map((place, at) => createPortal(box(place), place.host, `${place.kind} ${at}`))}
    </div>
  );
}

import { useRef } from "react";
import { createPortal } from "react-dom";
import type { MediaItem } from "../../api/media";
import { DetailsForm as Form } from "../../media/DetailsForm";
import { useAiReady } from "./api";
import { fillField, textField } from "./fields";
import { SuggestBox } from "./SuggestBox";
import { slot, useSlots } from "./slots";
import { aiWords } from "./words";
import "./ai.css";

type Props = { item: MediaItem };

/** Under the alt text field, once the form draws it for a picture. */
function altPlace(root: HTMLElement): { host: HTMLElement }[] {
  const box = textField(root, "media-alt")?.closest(".field");
  return box ? [{ host: slot(box, "alt") }] : [];
}

/**
 * A file's details with Suggest alt text under the alt field, when the
 * owner turned AI suggestions on. It sends the file's name, caption and
 * current alt text, never the picture itself.
 */
export function DetailsForm({ item }: Props) {
  const ready = useAiReady();
  const root = useRef<HTMLDivElement>(null);
  const shown = useSlots(root, ready, altPlace);
  const value = (id: string) => textField(root.current ?? document, id)?.value.trim() || null;
  const ask = () => ({
    task: "alt_text" as const,
    picture: { name: item.name, caption: value("media-caption"), alt: value("media-alt") },
  });
  const onUse = (text: string) => {
    const field = textField(root.current ?? document, "media-alt");
    if (field !== null) {
      fillField(field, text);
    }
  };
  return (
    <div ref={root} className="ai-host">
      <Form item={item} />
      {shown.map((place) =>
        createPortal(
          <SuggestBox
            label={aiWords.alt}
            name={aiWords.altLabel}
            ask={ask}
            pick={{ kind: "use", onUse }}
          />,
          place.host,
          "alt",
        ),
      )}
    </div>
  );
}

import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { listMedia, type MediaItem, uploadMedia } from "../api/media";
import { copyPicture } from "../api/pictures";
import { isData } from "../api/redirects";
import { en } from "../strings/en";
import { isPhoto, squareCrop } from "./photo";

type Props = { value: string; onChange: (photo: string) => void };

/** The library folder uploaded photos go into. */
const FOLDER = "authors";

/**
 * An author's photo: a preview, an upload or a pick from Media, both made
 * square for the round avatar, and remove. A file name can still be typed.
 */
export function PhotoPicker({ value, onChange }: Props) {
  const words = en.authors.edit.photo;
  const [status, setStatus] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [browsing, setBrowsing] = useState(false);
  const library = useQuery({
    queryKey: ["media", "photos"],
    queryFn: () => listMedia({}),
    enabled: browsing,
  }).data;

  const use = async (item: MediaItem) => {
    const crop = squareCrop(item);
    const copy = crop === null ? { url: item.url } : await copyPicture(item.id, crop, words.failed);
    setBusy(false);
    if ("failed" in copy) {
      setStatus(copy.failed);
      return;
    }
    onChange(copy.url);
    setBrowsing(false);
    setStatus(words.ready);
  };

  const upload = async (file: File | undefined) => {
    if (file === undefined) {
      return;
    }
    setBusy(true);
    setStatus(words.uploading);
    const reply = await uploadMedia(file, FOLDER);
    if (!isData(reply)) {
      setBusy(false);
      setStatus(words.failed);
      return;
    }
    await use(reply.data.item);
  };

  const pictures = isData(library) ? library.data.media.filter(isPhoto) : [];
  return (
    <fieldset className="photo-picker">
      <legend>{words.label}</legend>
      <p className="hint" id="photo-hint">
        {words.hint}
      </p>
      {/* A file beside the author's file shows only on the blog, so it has no preview here. */}
      {value.startsWith("/") || value.startsWith("https://") ? (
        <img className="photo-preview" src={value} alt={words.preview} width={96} height={96} />
      ) : null}
      <div className="actions">
        <label className="btn" htmlFor="photo-upload">
          {words.upload}
        </label>
        <input
          id="photo-upload"
          className="visually-hidden"
          type="file"
          accept="image/jpeg,image/png,image/webp"
          aria-describedby="photo-hint"
          disabled={busy}
          onChange={(event) => void upload(event.target.files?.[0])}
        />
        <button type="button" className="btn" onClick={() => setBrowsing(!browsing)}>
          {words.choose}
        </button>
        {value === "" ? null : (
          <button
            type="button"
            className="btn"
            onClick={() => {
              onChange("");
              setStatus(words.removed);
            }}
          >
            {words.remove}
          </button>
        )}
      </div>
      <p role="status">{status}</p>
      {browsing ? (
        <section aria-labelledby="photo-library-title">
          <h3 id="photo-library-title">{words.chooseTitle}</h3>
          {library !== undefined && pictures.length === 0 ? <p>{words.none}</p> : null}
          <ul className="photo-grid" role="list">
            {pictures.map((item) => (
              <li key={item.id}>
                <button
                  type="button"
                  disabled={busy}
                  aria-label={words.pick(item.alt === "" ? item.name : item.alt)}
                  onClick={() => {
                    setBusy(true);
                    void use(item);
                  }}
                >
                  <img src={item.url} alt="" width={80} height={80} loading="lazy" />
                </button>
              </li>
            ))}
          </ul>
        </section>
      ) : null}
      <details>
        <summary>{words.path}</summary>
        <div className="field">
          <label htmlFor="edit-photo">{words.pathLabel}</label>
          <span className="hint" id="edit-photo-hint">
            {words.pathHint}
          </span>
          <input
            id="edit-photo"
            value={value}
            aria-describedby="edit-photo-hint"
            onChange={(event) => onChange(event.target.value)}
          />
        </div>
      </details>
    </fieldset>
  );
}

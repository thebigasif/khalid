import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { isData } from "../api/call";
import { listMedia } from "../api/media";
import { designWords } from "../strings/designWords";

type Props = { value: string; onChange: (logo: string) => void };

/** The logo: a picture picked from Media, or none so the blog shows its name. */
export function LogoPicker({ value, onChange }: Props) {
  const words = designWords;
  const [browsing, setBrowsing] = useState(false);
  const library = useQuery({
    queryKey: ["media", "logo"],
    queryFn: () => listMedia({}),
    enabled: browsing,
  }).data;
  const pictures = isData(library)
    ? library.data.media.filter((item) => item.kind === "image")
    : [];

  return (
    <section aria-labelledby="logo-title">
      <h2 id="logo-title">{words.logo}</h2>
      <p className="hint" id="logo-hint">
        {words.logoHint}
      </p>
      {value === "" ? (
        <p>{words.noLogo}</p>
      ) : (
        <img className="logo-preview" src={value} alt={words.logoAlt} height={48} />
      )}
      <div className="actions">
        <button
          type="button"
          className="btn"
          aria-describedby="logo-hint"
          aria-expanded={browsing}
          onClick={() => setBrowsing(!browsing)}
        >
          {words.chooseLogo}
        </button>
        {value === "" ? null : (
          <button type="button" className="btn" onClick={() => onChange("")}>
            {words.removeLogo}
          </button>
        )}
      </div>
      {browsing ? (
        <section aria-labelledby="logo-pictures">
          <h3 id="logo-pictures">{words.pickTitle}</h3>
          {library !== undefined && pictures.length === 0 ? <p>{words.noPictures}</p> : null}
          <ul className="photo-grid">
            {pictures.map((item) => (
              <li key={item.id}>
                <button
                  type="button"
                  aria-label={words.pick(item.alt === "" ? item.name : item.alt)}
                  onClick={() => {
                    onChange(item.url);
                    setBrowsing(false);
                  }}
                >
                  <img src={item.url} alt="" width={80} height={80} loading="lazy" />
                </button>
              </li>
            ))}
          </ul>
        </section>
      ) : null}
    </section>
  );
}

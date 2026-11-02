import { useNavigate } from "@tanstack/react-router";
import { type FormEvent, useState } from "react";
import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";
import { CHECKS, INDEX, type PostSearch, postSearch, SEO } from "./filters";
import { SelectFilter } from "./SelectFilter";
import { STAGES } from "./stages";

type Props = { search: PostSearch };

/** The posts screen's filters; showing them writes them into the address, from page 1. */
export function PostFilters({ search }: Props) {
  const navigate = useNavigate();
  const [checks, setChecks] = useState<string>(search.checks ?? "");
  const [stage, setStage] = useState<string>(search.stage ?? "");
  const [index, setIndex] = useState<string>(search.index ?? "");
  const [seo, setSeo] = useState<string>(search.seo ?? "");

  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    void navigate({ to: "/posts", search: postSearch({ checks, stage, index, seo }) });
  };

  return (
    <form className="filters" method="get" aria-label={en.filters.label} onSubmit={submit}>
      <SelectFilter
        id="checks"
        label={en.posts.checks}
        any={en.filters.anyChecks}
        value={checks}
        options={CHECKS.map((value) => ({ value, label: en.filters.checks[value] }))}
        onChange={setChecks}
      />
      <SelectFilter
        id="stage"
        label={en.posts.review}
        any={en.filters.anyStage}
        value={stage}
        options={STAGES.map((value) => ({ value, label: en.stage[value] }))}
        onChange={setStage}
      />
      <SelectFilter
        id="index"
        label={en.posts.index}
        any={en.filters.anyIndex}
        value={index}
        options={INDEX.map((value) => ({ value, label: en.index[value] }))}
        onChange={setIndex}
      />
      <SelectFilter
        id="seo"
        label={seoEditorWords.seoFilter}
        any={seoEditorWords.anySeo}
        value={seo}
        options={SEO.map((value) => ({ value, label: seoEditorWords.seoFilters[value] }))}
        onChange={setSeo}
      />
      <button type="submit" className="btn">
        {en.filters.show}
      </button>
    </form>
  );
}

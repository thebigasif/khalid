import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { changeDesign, readDesign } from "../api/design";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { DesignForm } from "../design/DesignForm";
import { designChange, type Values } from "../design/designChange";
import { designWords } from "../strings/designWords";
import type { UserMessage } from "../strings/en";

const DESIGN = ["design"];

/** The blog's colours, fonts, shape and logo, saved to `theme/tokens.toml`. */
export function Design() {
  const words = designWords;
  const queries = useQueryClient();
  const design = useQuery({ queryKey: DESIGN, queryFn: readDesign }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  let shown = <p role="status">{words.loading}</p>;
  if (design !== undefined && !isData(design)) {
    shown = <Problem message={failureMessage(design)} />;
  } else if (isData(design)) {
    const read = design.data;
    const save = async (values: Values) => {
      const change = designChange(read, values);
      if (change === null) {
        setSaid(words.unchanged);
        return;
      }
      const reply = await changeDesign(change);
      setSaid(isData(reply) ? words.live : null);
      setFailed(isData(reply) ? null : failureMessage(reply));
      if (isData(reply)) {
        queries.setQueryData(DESIGN, reply);
      }
    };
    shown = (
      <>
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{said}</p>
        <DesignForm key={read.updated_at ?? "new"} design={read} onSave={save} />
      </>
    );
  }
  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      {shown}
    </>
  );
}

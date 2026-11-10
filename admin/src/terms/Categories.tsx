import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { readSite } from "../api/site";
import { createCategory, listCategories, type NewCategory } from "../api/terms";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { termWords } from "../strings/termWords";
import type { CategoryPick } from "./CategoryChange";
import { CategoryTable } from "./CategoryTable";
import { NewCategoryForm } from "./NewCategoryForm";
import { categoryName, nameLanguages } from "./names";

/** The query key the screen refreshes after a category changes. */
export const CATEGORIES = ["categories"];

// The same key the Look screen reads the settings under.
const SITE = ["site"];

type Props = { onPick: (pick: CategoryPick) => void };

/** The site's categories, adding one, and picking one to rename or delete. */
export function Categories({ onPick }: Props) {
  const words = termWords.categories;
  const queries = useQueryClient();
  const listed = useQuery({ queryKey: CATEGORIES, queryFn: listCategories }).data;
  const site = useQuery({ queryKey: SITE, queryFn: readSite }).data;
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const settings = isData(site) ? site.data : undefined;
  const languages = nameLanguages(settings?.languages, isData(listed) ? listed.data : []);
  const first = settings?.default_language ?? languages[0] ?? "en";
  const ordered = [first, ...languages.filter((code) => code !== first)];

  const add = async (category: NewCategory) => {
    const reply = await createCategory(category);
    if (!isData(reply)) {
      setDone(null);
      setFailed(failureMessage(reply));
      return false;
    }
    setFailed(null);
    const [made] = reply.data;
    setDone(termWords.addCategory.added(made ? categoryName(made, first) : category.slug));
    await queries.invalidateQueries({ queryKey: CATEGORIES });
    return true;
  };

  let list = <p role="status">{words.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    list = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    list = <p role="status">{words.none}</p>;
  } else if (isData(listed)) {
    list = (
      <>
        <CategoryTable
          categories={listed.data}
          language={first}
          onPick={(category, action) => onPick({ category, action, languages: ordered })}
        />
        <p className="muted">{words.removeNote}</p>
      </>
    );
  }
  return (
    <>
      <h2 id="categories-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      {list}
      <section aria-labelledby="add-category-title">
        <h3 id="add-category-title">{termWords.addCategory.title}</h3>
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{done}</p>
        <NewCategoryForm languages={ordered} onAdd={add} />
      </section>
    </>
  );
}

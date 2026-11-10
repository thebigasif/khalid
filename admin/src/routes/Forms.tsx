import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { type FormAnswer, type FormSummary, listAnswers, listForms } from "../api/forms";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { moment } from "../strings/dates";
import { en } from "../strings/en";
import { formWords } from "../strings/formWords";

/** What visitors sent through form blocks: one form at a time, newest answers first. */
export function Forms() {
  const words = formWords;
  const forms = useQuery({ queryKey: ["forms"], queryFn: listForms }).data;
  const [chosen, setChosen] = useState<string | null>(null);

  let shown = <p role="status">{words.loading}</p>;
  if (forms !== undefined && !isData(forms)) {
    shown = <Problem message={problemMessage(forms.problem, en.problems.unreachable)} />;
  } else if (isData(forms) && forms.data.length === 0) {
    shown = <p role="status">{words.none}</p>;
  } else if (isData(forms)) {
    const form = forms.data.find((each) => each.id === chosen) ?? forms.data[0];
    if (form !== undefined) {
      shown = (
        <>
          <div className="field">
            <label htmlFor="forms-which">{words.which}</label>
            <select
              id="forms-which"
              value={form.id}
              onChange={(event) => setChosen(event.target.value)}
            >
              {forms.data.map((each) => (
                <option key={each.id} value={each.id}>
                  {words.option(each.id, each.answers)}
                </option>
              ))}
            </select>
          </div>
          <FormAnswers key={form.id} form={form} />
        </>
      );
    }
  }
  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      <dl className="terms">
        {words.terms.map(({ term, meaning }) => (
          <div key={term}>
            <dt>{term}</dt>
            <dd>{meaning}</dd>
          </div>
        ))}
      </dl>
      {shown}
    </>
  );
}

/** The answers to `form`, a page at a time. */
function FormAnswers({ form }: { form: FormSummary }) {
  const words = formWords;
  const [page, setPage] = useState(1);
  const answers = useQuery({
    queryKey: ["forms", form.id, page],
    queryFn: () => listAnswers(form.id, page),
  }).data;

  if (answers === undefined) {
    return <p role="status">{words.loading}</p>;
  }
  if (!isData(answers)) {
    return <Problem message={problemMessage(answers.problem, en.problems.unreachable)} />;
  }
  const { pages } = answers.data.meta.pagination;
  return (
    <>
      <p role="status">{words.count(form.answers, form.id, moment(form.last_at))}</p>
      <ol className="answers">
        {answers.data.answers.map((answer) => (
          <Answer key={answer.id} answer={answer} />
        ))}
      </ol>
      {pages > 1 ? (
        <nav className="paging" aria-label={words.page(page, pages)}>
          <button
            type="button"
            className="btn"
            disabled={page <= 1}
            onClick={() => setPage(page - 1)}
          >
            {words.newer}
          </button>
          <span>{words.page(page, pages)}</span>
          <button
            type="button"
            className="btn"
            disabled={page >= pages}
            onClick={() => setPage(page + 1)}
          >
            {words.older}
          </button>
        </nav>
      ) : null}
    </>
  );
}

/** One answer: when and where it was sent, and each field as the form named it. */
function Answer({ answer }: { answer: FormAnswer }) {
  return (
    <li className="card answer">
      <h2>
        <time dateTime={answer.sent_at}>{moment(answer.sent_at)}</time>
        {` ${formWords.sentOn} `}
        <a href={answer.page}>{answer.page}</a>
      </h2>
      <dl>
        {answer.fields.map((field) => (
          <div key={field.name}>
            <dt>{field.name}</dt>
            <dd>{field.value}</dd>
          </div>
        ))}
      </dl>
    </li>
  );
}

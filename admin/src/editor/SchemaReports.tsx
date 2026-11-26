import type { CheckFinding, SchemaChecks } from "../api/schemaCheck";
import { en } from "../strings/en";
import type { SchemaChange } from "./schemaFix";

type Props = {
  checks: SchemaChecks;
  /** Makes one change in the schema field, or null when there is no field to change. */
  onChange: ((change: SchemaChange) => void) | null;
};

/** The findings of one report, or a line saying there is nothing to fix. */
function Report({
  id,
  title,
  findings,
  onChange,
}: { id: string; title: string } & Pick<Props, "onChange"> & {
    findings: CheckFinding[];
  }) {
  const words = en.schema.reports;
  return (
    <section aria-labelledby={id}>
      <h3 id={id}>{title}</h3>
      {findings.length === 0 ? <p className="hint">{words.nothing}</p> : null}
      {findings.map((finding) => {
        const block = finding.level === "error";
        const tag = block ? words.error : words.warning;
        const change = finding.change ?? null;
        return (
          <section
            key={`${finding.path} ${finding.what}`}
            className={`check ${block ? "block" : "warn"}`}
            aria-label={tag}
          >
            <p className="tag">{tag}</p>
            {finding.path === "" ? null : <p className="hint">{words.where(finding.path)}</p>}
            <p>
              <strong>{finding.what}</strong>
            </p>
            <p>{finding.why}</p>
            {change === null || onChange === null ? (
              <p>{finding.fix}</p>
            ) : (
              <button type="button" className="btn" onClick={() => onChange(change)}>
                {change.label}
              </button>
            )}
          </section>
        );
      })}
    </section>
  );
}

/** The two reports on the writer's own schema: correct schema.org, and what Google needs. */
export function SchemaReports({ checks, onChange }: Props) {
  const words = en.schema.reports;
  return (
    <div className="schema-reports">
      <Report
        id="schema-org-report"
        title={words.schemaOrg}
        findings={checks.schema_org}
        onChange={onChange}
      />
      <Report
        id="schema-google-report"
        title={words.google}
        findings={checks.google}
        onChange={onChange}
      />
    </div>
  );
}

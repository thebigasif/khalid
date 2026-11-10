import type { Change } from "../api/records";
import { moment } from "../strings/dates";
import { en } from "../strings/en";

type Props = { changes: Change[] };

/**
 * One page of the audit log: when, who, what happened and where. Who is a
 * name; an identity with no name, such as a revoked key, shows as its code.
 */
export function AuditTable({ changes }: Props) {
  const words = en.audit;
  return (
    <div className="scroll">
      <table aria-labelledby="audit-title">
        <thead>
          <tr>
            <th scope="col">{words.when}</th>
            <th scope="col">{words.who}</th>
            <th scope="col">{words.what}</th>
            <th scope="col">{words.where}</th>
          </tr>
        </thead>
        <tbody>
          {changes.map((change) => (
            <tr key={change.id}>
              <td>
                <time dateTime={change.at}>{moment(change.at)}</time>
              </td>
              <td data-label={words.who}>
                {change.by_name === change.by ? <code>{change.by}</code> : change.by_name}
              </td>
              <td data-label={words.what}>{words.happened(change.action, change.route)}</td>
              <td data-label={words.where}>
                <code>{change.path.replace("/api/admin/v1", "")}</code>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

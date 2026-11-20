import { useQuery } from "@tanstack/react-query";
import { readAlerts } from "../api/alerts";
import { isData } from "../api/call";
import { dashboardWords as words } from "../strings/dashboardWords";
import { day } from "../strings/dates";

/** The most problems one alert lists; the rest are counted. */
const MOST = 5;

/** The alerts Khalid raised for the owner, so one with no email set up still sees them. */
export function Alerts() {
  const alerts = useQuery({ queryKey: ["alerts"], queryFn: readAlerts }).data;
  // A calm blog, or an older one that cannot list them, leaves the panel out.
  if (!isData(alerts) || alerts.data.length === 0) {
    return null;
  }
  return (
    <section className="panel" aria-labelledby="alerts-title">
      <h2 id="alerts-title">{words.alerts}</h2>
      <p className="hint">{words.alertsLede}</p>
      <ol className="rows">
        {alerts.data.map((alert) => {
          const more = alert.findings.length - MOST;
          return (
            <li key={alert.kind}>
              <div>
                <strong>{alert.what}</strong>
                <span className="hint">
                  {alert.why} {alert.fix}
                </span>
                <ul>
                  {alert.findings.slice(0, MOST).map((finding) => (
                    <li key={finding.key}>{finding.line}</li>
                  ))}
                </ul>
                {more > 0 && <span className="hint">{words.alertsMore(more)}</span>}
              </div>
              <span className="pill block">{words.alertSince(day(alert.raised_at))}</span>
            </li>
          );
        })}
      </ol>
    </section>
  );
}

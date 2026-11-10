import { useQuery } from "@tanstack/react-query";
import { getRouteApi, Link } from "@tanstack/react-router";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { listDeliveries, listWebhooks } from "../api/webhooks";
import { Problem } from "../components/Problem";
import { en } from "../strings/en";
import { DeliveryTable } from "../webhooks/DeliveryTable";
import { eventSentence, hostOf } from "../webhooks/events";
import { WEBHOOKS } from "./Webhooks";

const route = getRouteApi("/signed-in/webhooks/$id");

/** What one webhook sent, newest first: delivered, trying again or given up, with each try. */
export function WebhookActivity() {
  const words = en.webhooks.sent;
  const { id } = route.useParams();
  const listed = useQuery({ queryKey: WEBHOOKS, queryFn: listWebhooks }).data;
  // Each page starts below the last message of the page before it.
  const [before, setBefore] = useState<number | undefined>(undefined);
  const sent = useQuery({
    queryKey: [...WEBHOOKS, id, "deliveries", before],
    queryFn: () => listDeliveries(id, before),
  }).data;

  const webhook = isData(listed) ? listed.data.find((each) => String(each.id) === id) : undefined;
  let shown = <p role="status">{words.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    shown = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && webhook === undefined) {
    shown = <Problem message={words.gone} />;
  } else if (sent !== undefined && !isData(sent)) {
    shown = <Problem message={failureMessage(sent)} />;
  } else if (webhook !== undefined && isData(sent)) {
    const next = sent.data.next;
    shown = (
      <>
        <p className="lede">{words.lede(eventSentence(webhook.event), hostOf(webhook.url))}</p>
        <h2 id="sent-title">{words.list}</h2>
        <p className="muted">{words.listLede}</p>
        <DeliveryTable deliveries={sent.data.deliveries} />
        <div className="flex flex-wrap items-center gap-3">
          {next === null || next === undefined ? null : (
            <button type="button" className="btn" onClick={() => setBefore(next)}>
              {words.older}
            </button>
          )}
          {before === undefined ? null : (
            <button type="button" className="btn" onClick={() => setBefore(undefined)}>
              {words.newest}
            </button>
          )}
        </div>
      </>
    );
  }
  return (
    <>
      <p>
        <Link to="/webhooks">{words.back}</Link>
      </p>
      <h1>{words.title}</h1>
      {shown}
    </>
  );
}

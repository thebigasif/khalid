import { useQuery } from "@tanstack/react-query";
import { failureMessage, isData } from "../api/call";
import { listNewsletters } from "../api/subscribers";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { subscriberWords } from "../strings/subscriberWords";
import { NewsletterCards } from "../subscribers/NewsletterCards";
import { SentTable } from "../subscribers/SentTable";

/** The newsletters and their readers: sending a post is the focus, the readers come last. */
export function Subscribers() {
  const words = subscriberWords;
  const listed = useQuery({ queryKey: ["newsletters"], queryFn: listNewsletters });
  const answer = listed.data;
  let body = <p role="status">{words.loading}</p>;
  if (answer !== undefined && !isData(answer)) {
    body = <Problem message={failureMessage(answer)} />;
  } else if (isData(answer)) {
    const { newsletters, sends } = answer.data;
    body = (
      <>
        <NewsletterCards newsletters={newsletters} />
        <SentTable sends={sends} />
      </>
    );
  }
  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      {body}
    </>
  );
}

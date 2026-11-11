# 0000. Bounces and complaints stop mail to an address

**Number:** next free. **Status:** Proposed, 2026-11-11. Gap audit 5, row 7 (`~/khalid-ops/backlog-2026-11-12.md`), KH-GRO-2, RULES.md BL-SEC-3 and BL-RUN-5.

## Context

Khalid sends newsletters through Resend or Postmark, but nothing listened when either service reported a bounce or a spam complaint. Dead addresses and readers who complained kept getting every issue, and both services flag or close accounts that keep doing so.

## Decision

- **One hook, `POST /api/hooks/mail`, beside git mode's push hook**, in the binary. It exists only when `[mail]` names Resend or Postmark and `KHALID_MAIL_HOOK_SECRET` is set. It is not in the Admin API or its OpenAPI file: like the push hook, a service calls it, not a person or an integration key.
- **Proof that the service sent it.** Resend signs events the Svix way, so the hook checks the HMAC-SHA256 of `id.timestamp.body` against `svix-signature` with the signing secret. Postmark does not sign events but puts a user name and password from the hook's address into `Authorization: Basic`, so the password must be the secret, compared in constant time. The timestamp is not checked: a replayed event can only stop an address that is already stopped.
- **What stops an address.** Resend `email.bounced` with bounce type `Permanent`, `email.complained`, Postmark `Bounce` of type `HardBounce` or `BadEmailAddress`, and Postmark `SpamComplaint`. Soft bounces change nothing. No count of soft bounces is kept.
- **What stopping does.** The subscriber becomes `unsubscribed`, leaves every newsletter, loses unused confirmation links, and gets `stopped_because` (`bounced` or `complained`, migration 0033). The sign-up form answers such an address with the usual "check your inbox" page but stores and sends nothing (`SignedUp::Stopped`), since anyone can type any address into the form. The address is not deleted, so the stop holds.
- **SMTP is left out.** An SMTP server reports bounces by mail to the sender, which Khalid does not read.

## Consequences

- A reader whose address was stopped by mistake cannot sign up again through the form. The admin does not show `stopped_because` yet, and there is no way to clear it; both are follow-ups.
- Transactional mail (form answers to the owner, sign-in links, alerts) is not stopped; only subscribers have a status to change.

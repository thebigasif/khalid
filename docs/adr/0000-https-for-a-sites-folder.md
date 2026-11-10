# 0000. `khalid serve --https` gets a certificate for every site of a sites folder

**Status:** Proposed, 2026-11-10. Backlog row 34 (gap audit 3), plan sections 4.4 and 4.7, KH-INS-4, KH-EDT-7. Number: next free.

## Context

`khalid serve --domain` gets and renews one certificate for one site (ADR "`khalid serve --domain` gets and renews its own certificate"). A sites folder serves many sites, each on its own domain, behind one listener (ADR "many sites in one process"). That ADR refused `--domain` in a sites folder and sent owners to a proxy such as Caddy, leaving built-in certificates for many domains to a later task. This is that task. The plan does not say how the owner asks for it, how many certificates there are, or where they are kept, so those choices are recorded here. The founder said yes in advance (2026-10-27).

## Decision

- **A flag of its own: `khalid serve --https`.** `--domain` names one domain, and a sites folder lists its domains in `sites.toml`, so the folder needs no name on the command line. `--https` is valid only in a sites folder and `--domain` only for one site; each refuses the other case and names the flag to use. `--accept-terms` and the hidden test flags work with either.
- **One certificate per domain, not one certificate naming every domain.** One certificate for all would fail for every site when one domain's DNS does not lead here yet, would be ordered again for every site added, and is capped at 100 names. Separate certificates fail and renew alone. The TLS listener picks the certificate by the name the browser asks for; a handshake with no name, or an unknown one, gets the first certificate set, so one site behaves as before.
- **One Let's Encrypt account, kept in the sites folder's `data/https/`.** Let's Encrypt limits new accounts per address, so one account per site would break on a large folder. The owner agrees to the terms once. The account is opened or signed in to once, in the background, so saved certificates are served at once while Let's Encrypt cannot be reached, as with one site.
- **Each site's certificate lives in its own `sites/<domain>/data/https/`.** It is where the single-site command keeps it, so a site keeps its certificate when it moves out of the folder or is served alone, and the certificate alert reads it for each site as it does for one.
- **A site that cannot get a certificate does not stop the others.** Its failure is logged and tried again an hour later. A domain Let's Encrypt never issues for, such as a name with no dot, is reported once, and the site is not served over HTTPS.
- **Port 80 moves a visitor to HTTPS on the name they asked for once that name has a certificate, and to the home site otherwise.** Only names with a certificate are echoed into the `Location` header, so a forged `Host` cannot send a visitor elsewhere.
- **Each order forgets only its own check answers.** Two domains ordering at once share the answers the listener and port 80 read, so clearing them all at the end of one order would fail the other.

## Consequences

- The bullet "No built-in HTTPS for a sites folder yet" in ADR "many sites in one process" is replaced by this record. A proxy in front with `--listen` still works.
- No new dependency. `tokio`'s `sync` feature is named in khalid-https; the binary has it already through hyper.
- A site removed from `sites.toml` while the server runs keeps its renewal task until the next restart. Sites are not removed while the process runs today.

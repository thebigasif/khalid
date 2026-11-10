---
title: "Running Khalid on a server"
order: 20
---

# Running Khalid on a server

Khalid is one file. Put it on a machine, point it at a site folder, and it serves the blog and the admin. This page shows the ways to keep it running. The files it names are in `packaging/` in the repository.

A site folder is the folder that holds `site.toml`, `content/` and `theme/`. Khalid keeps what it works out from those files, such as its database and image sizes, in `data/` inside the same folder. It also notes who last changed each post through the admin, the API or the scheduler in `data/post-editors.txt`, so the admin can name them. A post you then change by hand in its file shows no name until the next change in the admin. `khalid backup` saves the whole folder in one archive, as "Backing up and restoring" below shows.

## Install with one line

On Linux or a Mac, one line installs the newest release. On Windows, download the `.exe` from the releases page. To build Khalid from source instead, follow `CONTRIBUTING.md` and copy `target/release/khalid` to the server.

```sh
curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sh
```

The script downloads the newest release for your machine and checks it against the release's list of checksums, `SHA256SUMS`. A checksum is a fingerprint of a file: if the download was damaged or changed, the fingerprints differ and the script stops without installing anything. As root it installs to `/usr/local/bin`, otherwise to `~/.local/bin`.

It also installs two extras, and says what it did with each:

- **The manual pages**, the help that `man khalid-init` shows. They go to `~/.local/share/man/man1`, or to `/usr/local/share/man/man1` as root, and are checked against `SHA256SUMS` like the binary.
- **Tab completion**, which makes your shell finish a `khalid` command when you press Tab. The script sets it up for the shell you log in with, if that is bash, zsh or fish:

| Shell | For you | As root, for everyone |
| --- | --- | --- |
| bash | `~/.local/share/bash-completion/completions/khalid` | `/usr/local/share/bash-completion/completions/khalid` |
| zsh | `~/.zfunc/_khalid` | `/usr/local/share/zsh/site-functions/_khalid` |
| fish | `~/.config/fish/completions/khalid.fish` | `/usr/local/share/fish/vendor_completions.d/khalid.fish` |

Open a new terminal to use it. For zsh in your home folder, `~/.zshrc` needs the line `fpath=(~/.zfunc $fpath)` before `compinit`; the script prints it when the line is missing and leaves the file alone. If an extra cannot be installed, for example on an older release without manual pages or with another shell, the script says why and the binary stays installed. `docs/cli.md` shows how to add either one by hand.

Options, written after `sh -s --`:

- `--version v1.3.2` installs that release instead of the newest.
- `--dir FOLDER` installs into another folder.
- `--systemd`, as root, also sets up the service described below.
- `--no-man` leaves out the manual pages.
- `--no-completions` leaves out Tab completion.

For example, to install and set up the service on a server:

```sh
curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sudo sh -s -- --systemd
```

## Signing in to the admin the first time

The admin is at `admin/` in the blog's folder: `/admin/` for a blog that is the whole site, and `/blog/admin/` for one under `/blog/`.

To put the admin in another folder, for example because a page of the blog already uses `/admin/`, name it in `site.toml`:

```toml
admin_folder = "studio"
```

The admin then answers at `/studio/`, or `/blog/studio/` under `/blog/`, and `/admin/` is left to the blog's pages. Sign-in links, the staff toolbar, the link `khalid staff owner` prints and each `admin_path` in the Admin API name the new folder. The name is one folder of lowercase letters, digits and hyphens. It cannot be `api`, `ghost` or one of the site's language codes, since those folders already answer. Restart the server after changing it.

The first person in is the owner, and the command line makes them:

```sh
khalid --site blog staff owner --email you@example.com --name "Your Name"
```

This is the only time Khalid shows the sign-in link. The link works once, within 15 minutes. It is shown on two addresses: the site's own, and `localhost`, for a site that is not online yet. Open the `localhost` one while `khalid serve` runs on the same computer. If `khalid serve` listens somewhere other than `127.0.0.1:8080`, pass the same `--listen` to `staff owner`. Once signed in, choose a password or add a passkey.

## systemd on a Linux machine

systemd is the program most Linux systems use to start services at boot. Khalid ships a unit file for it, `packaging/systemd/khalid.service`, and a user file, `packaging/systemd/khalid.sysusers`.

The unit runs Khalid as its own `khalid` user, never as root. The site folder is `/var/lib/khalid`, and the service can write nowhere else. It answers on `127.0.0.1:8080`, so only a program on the same machine, such as a proxy, can reach it.

1. Put the binary at `/usr/local/bin/khalid`.
2. Copy the two files and make the user:

   ```sh
   sudo cp packaging/systemd/khalid.service /etc/systemd/system/
   sudo mkdir -p /etc/sysusers.d
   sudo cp packaging/systemd/khalid.sysusers /etc/sysusers.d/khalid.conf
   sudo systemd-sysusers
   ```

   Some systems, such as Ubuntu, have no `/etc/sysusers.d` folder until something is put there, so the second line makes it. If your system says `systemd-sysusers: command not found`, make the user with `useradd` instead:

   ```sh
   sudo useradd --system --user-group --home-dir /var/lib/khalid --no-create-home --shell /usr/sbin/nologin khalid
   ```

3. Make the site, as the `khalid` user:

   ```sh
   sudo install -d -o khalid -g khalid -m 0750 /var/lib/khalid
   sudo -u khalid khalid init /var/lib/khalid --address https://blog.example.com --yes
   ```

4. Start it now and at every boot:

   ```sh
   sudo systemctl enable --now khalid
   ```

`systemctl status khalid` says whether it runs, and `journalctl -u khalid` shows what it printed.

To change the command, for example to answer on every address of the machine, run `sudo systemctl edit khalid` and write:

```ini
[Service]
ExecStart=
ExecStart=/usr/local/bin/khalid --site /var/lib/khalid serve --listen 0.0.0.0:8080
```

The empty `ExecStart=` line clears the old command first. The unit already lets Khalid answer on ports 80 and 443 without root, for when it gets its own certificate with `serve --domain`.

The unit also sets `Environment=MIMALLOC_PURGE_DELAY=0`, the allocator setting `docs/standards/performance.md` records: it makes mimalloc return freed pages to the OS at once, which keeps the server's idle memory inside its budget. If you run the release binary by hand instead of through the unit, the image or `compose.yaml`, set the same variable in the shell first:

```sh
MIMALLOC_PURGE_DELAY=0 khalid --site /var/lib/khalid serve --listen 127.0.0.1:8080
```

## Changing settings while the server runs

`khalid serve` watches the site folder. When you save `site.toml`, `routes.toml`, `categories.toml`, `tags.toml` or any file under `theme/`, such as `tokens.toml`, `site.css` or a font, it reads the settings again the way it does at start and renders the pages again. You do not need to restart it. Several files saved at once cost one render.

If a saved `site.toml` or theme file has a mistake, the blog keeps serving the last good pages and settings. The server writes one line to its log, the same message `khalid --site` gives, with the file and the line to fix. Fix the file and save it again.

A few settings only take effect when the server starts:

- `mount`, `admin_folder` and `mode` shape the server's own addresses, so it keeps the values it started with.
- `address`, `languages`, `slugs`, `[media]`, `[mail]`, `[alerts]`, `[code]`, `[search_console]`, `[visitor_log]`, `[limits]`, `[ai]` and `preview_link_days` also set up the admin, mail, alerts and other parts of the server at start. The pages use the new value at once; the rest uses it after a restart.
- The `--listen`, `--domain` and `--https` options, the environment variables, and the `data/` folder with its database are read once at start.

When you change one of these, the log says a restart is needed, and so does the admin: its settings screens show a note, and the Admin API's settings answer `restart_needed: true`. A change to anything else, such as the blog's name, its look, its SEO or its privacy settings, answers `false`, since the blog already uses it. Restart with `systemctl restart khalid`, or stop and start the server.

## Docker

Docker runs programs in containers, each with its own files. The `Dockerfile` at the top of the repository builds an image from scratch, which means the image starts empty. It holds the Khalid binary, built fully static so it needs no other library, and the certificates Khalid needs to make HTTPS requests. There is no shell and no package manager in it, so there is nothing else to patch.

The image keeps the site folder at `/site` and answers on port 8080. It runs as user 65532, not as root.

```sh
docker build -t khalid .
mkdir site
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD/site:/site" khalid init /site --address https://blog.example.com --yes
docker run -d -u "$(id -u):$(id -g)" -p 8080:8080 -v "$PWD/site:/site" khalid
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD/site:/site" khalid staff owner --email you@example.com --name "Your Name"
```

The build compiles Khalid and the admin: about 20 minutes on a machine with 4 cores, longer on a smaller one. To skip it, use the published image below. `-u` runs Khalid as you, so the files it writes in `site/` stay yours. The blog is then at `http://localhost:8080/`, and the last line prints the admin's sign-in link, as "Signing in to the admin the first time" above says.

The image already adds `--site /site` to every command, so write the command without it: `khalid staff owner`, not `khalid --site /site staff owner`. Khalid refuses `--site` given twice. With Compose, the same command is `docker compose run --rm khalid staff owner --email you@example.com --name "Your Name"`.

Each release also publishes the image, for x86_64 and ARM, so you can skip the build. Use `ghcr.io/thebigasif/khalid:latest` in place of `khalid` in the commands above, or a version such as `ghcr.io/thebigasif/khalid:1.3.2` to stay on one release.

## Docker Compose

Docker Compose starts containers from a file, `compose.yaml`, so the settings live in one place. The file in the repository runs the image with the site in `./site`, port 8080, a read-only container and no extra rights.

```sh
mkdir site
docker compose run --rm khalid init /site --address https://blog.example.com --yes
docker compose up -d
```

Compose runs Khalid as user 1000, the first user on most Linux machines. If `id -u` prints another number, put `KHALID_UID=<that number>` and `KHALID_GID=<the number id -g prints>` in a file named `.env` next to `compose.yaml`. When the image cannot be downloaded yet, Compose builds it from the repository.

`KHALID_ADDRESS` changes the site's address without editing `site.toml`, for example when a site moves to a new domain. Uncomment the `environment` lines in `compose.yaml` to set it.

## A blog under a folder of another site

A blog can live in a folder of a website built with something else, such as `https://example.com/blog/`. The folder is the blog's **mount**. Make the site with it, and name the program that serves the website:

```sh
khalid init blog --address https://example.com --mount /blog/ --stack caddy
```

Khalid prints the rule to add to that program, for nginx, Caddy, Apache, Traefik, a Cloudflare Worker, Vercel, Next.js, Netlify, Webflow or Shopify. The rule passes everything under `/blog/` on to Khalid, and Khalid serves everything there:

| What | Address |
| --- | --- |
| The blog | `/blog/` |
| The admin | `/blog/admin/` |
| The Content API and the Admin API | `/blog/api/content/v1/` and `/blog/api/admin/v1/` |
| The OpenAPI document | `/blog/api/openapi.json`, whose server is `https://example.com/blog` |
| The git mode push hook | `/blog/api/hooks/git` |
| Preview links, forms, comments and newsletter sign-ups | under `/blog/` |

Every address Khalid gives out names the folder too: sign-in links, preview links, the `site` in each webhook, `Location` headers, `admin_path`, and the address the Connect to SerpDad page shows. An old `/admin/` address moves to `/blog/admin/`, unless `admin_folder` puts the admin elsewhere. The APIs also still answer at `/api/` for a program that reaches Khalid directly, without the proxy in front.

Search engines read `robots.txt` only at the root of a host, so the blog's own `robots.txt` under `/blog/` is never read. `khalid init` prints the line to add to the website's `robots.txt`, built from the site's address and mount:

```text
Sitemap: https://example.com/blog/sitemap.xml
```

Add it to `https://example.com/robots.txt`. `khalid doctor` fetches that file and says when the line is missing.

## Hosting platforms

The repository holds a ready file for four hosts that run the server, and a ready static blog for Cloudflare, last below. Each of the four builds the `Dockerfile` or installs the binary, keeps the site on a disk that survives restarts, and makes the site on the first start. `KHALID_ADDRESS` says the blog's address. The image's command, `serve --init`, makes a site there when the disk is empty, and leaves a site it finds as it is.

### Fly.io

Fly.io runs containers close to readers. You need its command line tool, `fly`, and an account.

1. In a copy of this repository, run `fly launch --no-deploy`. It reads `fly.toml` and asks for the app's name.
2. In `fly.toml`, set `KHALID_ADDRESS` to `https://<the app's name>.fly.dev`, or to your own domain once it points at Fly.
3. Run `fly deploy`. Fly builds the image, makes a 1 GB disk for the site, and starts the blog.

The site lives on one disk, so the app runs on one machine. Fly stops it when no one reads and starts it on the next visit.

### Railway

Railway builds the `Dockerfile` with the settings in `railway.json`: one copy of the service, checked at `/` before it takes readers.

1. In Railway, make a project from this repository.
2. Under the service's Settings, Networking, make a public domain, and point it at port 8080.
3. Add a volume mounted at `/site`.
4. Under Variables, add `KHALID_ADDRESS` as `https://${{RAILWAY_PUBLIC_DOMAIN}}`, and `RAILWAY_RUN_UID` as `0`. Railway's volumes belong to the root user, so Khalid must run as root to write to it. Railway still keeps the container apart from the machine.
5. Deploy.

### Coolify

Coolify is a hosting panel you run on your own server.

1. Add a new resource from this repository, with the Docker Compose build pack.
2. Set the Docker Compose location to `/packaging/coolify/compose.yaml`.
3. Deploy.

Coolify makes the blog's address, sends it to port 8080, and passes it to Khalid as `KHALID_ADDRESS`. The site lives in the volume `khalid-site`. To use your own domain, change the domain in the service's settings and deploy again.

### A Hetzner Cloud server

`packaging/hetzner/cloud-config.yaml` sets up a new server in its first minute: it runs the install script with `--systemd`, makes the site, and starts Khalid with HTTPS from Let's Encrypt, the free service that issues certificates. Pasting it means you agree to Let's Encrypt's subscriber agreement.

1. In the Hetzner Cloud console, make a Primary IPv4 address under Networking, and point your blog's domain at it.
2. Copy the file and change `blog.example.com` in it to that domain.
3. Create a server with that address, and paste the file under "Cloud config".

The blog is at `https://<your domain>/` a few minutes later. Point the domain first: Khalid asks for its certificate on the first start, and Let's Encrypt only gives one for a domain that leads to the server.

### Cloudflare, free

Cloudflare builds a static blog from GitHub and serves its files for free. `deploy/cloudflare/` is a small, ready blog for it: on each build Cloudflare installs one pinned Khalid release, checked against its checksums, runs `khalid build`, and serves `public/`. The admin, comments and forms need a running server, so they are not part of a static blog.

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/thebigasif/khalid/tree/main/deploy/cloudflare)

The button copies the folder into a new repository in your GitHub account and deploys it as a Worker. For Cloudflare Pages, the build command is `sh build.sh`, the output folder is `public`, and the root folder is `deploy/cloudflare`. `docs/deploy/cloudflare.md` gives every step, the blog's address and your own domain.

A blog behind Cloudflare's proxy, static or served, needs Cloudflare's Email Address Obfuscation turned off. In the domain's dashboard, go to **Security**, **Settings** (or **Scrape Shield** on older dashboards) and switch it off. It rewrites every email address and `mailto:` link and adds a script from `/cdn-cgi/`. Khalid's content security policy blocks that script, as it should, so addresses show as "[email protected]" and the share-by-email link opens a missing page.

## Sending email

Khalid sends email when a reader signs up for the newsletter: one message with a link that confirms the address. It also sends you each answer to a form block. Until `site.toml` says how mail is sent, the sign-up form says sign-ups are not open yet and stores nothing.

Gated content, the part of a post inside `:::gated`, opens with the same email. A reader types their address in the form where the part would be, opens the link, clicks the button, and lands back on the post with the part open, with scripts off. Their browser keeps a cookie for 180 days that opens gated content on every post. On another phone or browser they type the same address again and get a link that opens it there. Without `[mail]` the part stays locked for everyone.

Add a `[mail]` table with the address mail comes from and exactly one way to send it.

Through the mail server your host gives you, with SMTP (the protocol mail servers speak):

```toml
[mail]
from = "news@example.com"
name = "Field Notes"
postal_address = "Field Notes Ltd, 1 Park Row, Leeds LS1 5AB, UK"

[mail.smtp]
host = "smtp.example.com"
username = "news@example.com"
```

`postal_address` is the address where people can reach you or your business by post. Anti-spam laws ask every newsletter email to show one, so Khalid sends no newsletter until it is set; sign-up emails work without it.

Khalid connects on port 587 and turns encryption on before it sends anything. Add `port = 465` and `security = "tls"` for a server that is encrypted from the start. A relay on the same machine or your own network needs `private_host = true`; only then may it use `security = "none"`.

Form answers are emailed to the `from` address unless you add `forms_to = "hello@example.com"` under `[mail]`.

Every answer is also kept in the site database, mail or not. Open Forms in the admin to read them, newest first; only an owner or an admin sees it, because answers hold visitors' email addresses. Tools read the same answers through the Admin API at `/api/admin/v1/forms/` with a key that has `subscribers:write`.

To count clicks on the links in newsletter emails, add `count_clicks = true` under `[mail]`. It is off unless you set it. See "Numbers" in `docs/newsletters.md`.

Through Resend or Postmark, two services that send mail for websites, write an empty `[mail.resend]` or `[mail.postmark]` instead of `[mail.smtp]`. Postmark sends on its `outbound` stream unless you add `message_stream = "<name>"`.

The password, API key or token never goes in `site.toml`. Set it in the server's environment, where only Khalid reads it:

| Way of sending | Variable |
|---|---|
| SMTP with a `username` | `KHALID_SMTP_PASSWORD` |
| Resend | `KHALID_RESEND_API_KEY` |
| Postmark | `KHALID_POSTMARK_TOKEN` |

With systemd, run `sudo systemctl edit khalid` and add `Environment=KHALID_SMTP_PASSWORD=...` under `[Service]`, or keep it in a file only root can read and name it with `EnvironmentFile=`. With Compose, put it in the `.env` file and pass it on under `environment:` in `compose.yaml`, as `KHALID_SMTP_PASSWORD: ${KHALID_SMTP_PASSWORD}`. Khalid refuses to start, and says which variable is missing, when the way you picked needs one that is not set.

## Alerts

`khalid serve` tells you when something on the blog needs you. Once an hour it checks for a burst of visits ending on missing pages (404s), an HTTPS certificate with under two weeks left, and live posts that have not changed in a year. It also tells you when a scheduled post did not go live because the linter refused it, and `khalid audit` tells you about broken links. Each problem is sent once while it lasts, and again if it clears and comes back.

Every alert goes to the webhooks registered for `alert.raised`, so Slack, Zapier or Make can pass it on:

```sh
khalid webhooks add --event alert.raised --url https://hooks.example.com/khalid
```

To get alerts by email too, set up `[mail]` as above and add the address:

```toml
[alerts]
email = "owner@example.com"
not_found_spike = 50     # new 404s in one hour that count as a burst
stale_after_days = 365   # 0 switches the stale-post alert off
watch_minutes = 5        # how often the speed and uptime watch looks; 0 switches it off
slow_ms = 1000           # a slower middle answer time over an hour, or three looks, is an alert; 0 switches it off
```

### Speed and uptime

Once the server answers more than its own computer, with `--domain`, `--https` or a `--listen` address such as `0.0.0.0:8080`, it looks at the blog's home page at its public address every 5 minutes, the way a reader's browser would: through DNS, the certificate and any proxy in front. Three failed looks in a row send a "site down" alert. An hour whose middle answer time is over 1 second sends a "slow site" alert. Both are sent once, and again only after the site has recovered.

To see the record, run:

```sh
khalid --site /srv/blog watch
```

It shows, for the last day and the last 30 days, how many looks answered, how fast, and the longest gap between looks. A long gap means the server itself was off, since a stopped server cannot look. To be told about that too, have a second machine or an outside monitor watch the address.

## The visitor log

The visitor log keeps one row per visit for 7 to 14 days, so Khalid can check that a bot is who it says it is and see when a new post was first crawled. It sets no cookie and keeps only shortened addresses, but it is still personal data, so it is off until you switch it on. Ask your lawyer to read `docs/design/visitor-log.md` first.

```toml
[visitor_log]
on = true
keep_days = 14   # 7 to 14
```

While it is on, three commands read it. A crawler counts only when its address is on the list its company publishes, so a script that borrows Googlebot's name is never counted as Googlebot.

- `khalid crawl first`: for each post that went live after the log saw its first crawl, how long each crawler took to fetch it.
- `khalid crawl health`: each search and AI crawler's visits over the kept days, by what the site answered: the page, moved, missing or failed.
- `khalid crawl errors`: each address that answered crawlers with a 4xx or 5xx, most hits first.

Add `--json` to any of them for the same as one JSON object. The hourly alerts also raise `crawl-stopped` when Googlebot or Bingbot has not fetched a page for a week, and `crawler-errors` when crawlers got a server error in the last hour. Both go out like every other alert.

## Request limits

`khalid serve` limits how often one key or one reader may ask for work, so a busy script or a leaked key cannot slow the blog for everyone. Past a limit the answer is 429, "too many requests", with a `Retry-After` header that says how many seconds to wait. The usual numbers suit one blog on a small server. To change them, add a `[limits]` table to `site.toml` and restart the server:

```toml
[limits]
admin_per_minute = 600        # Admin API requests per key or signed-in staff member
content_per_minute = 6000     # Content API requests per content key
wrong_keys_per_minute = 10    # wrong keys one address may send before it is paused
search_per_minute = 120       # searches one reader's address may make
```

Each number may be 1 to 100000. Searches count only when the reader typed words, so opening the empty search page never counts. A reader past the limit sees the search form under one sentence that says to wait a minute, then search again. A blog behind a proxy on the same machine or network, such as Caddy or nginx, is counted by the address the proxy passes on in `X-Forwarded-For`.

## Many sites on one server

One Khalid process can serve several blogs, each on its own domain. In an empty folder, add each site by its domain. The first one is the home site, where super admins add more:

```sh
khalid sites add example.com
khalid sites add news.example.com
khalid sites admin you@example.com
khalid --site sites/example.com staff owner --email you@example.com --name "Your Name"
khalid serve --listen 127.0.0.1:8080
```

Each site gets its own folder in `sites/`, with its own posts, theme, staff and database, and `khalid --site sites/<domain>` runs any other command on it. `khalid sites list` shows them all.

A super admin is a person whose email `sites.toml` lists and who signs in on the home site. They can add a site from there while the server runs, and it answers at once. Keys and staff of the other sites cannot. The page for it is `/admin/sites` on the home site: it asks for your email, sends you a sign-in link, and opening the link brings you back to it. The sign-in link `khalid staff owner` prints opens the same page.

Point each domain's DNS at the server. Then serve every site over HTTPS on ports 443 and 80, each with its own free certificate from Let's Encrypt that Khalid gets and renews:

```sh
khalid serve --https --accept-terms
```

`--accept-terms` is your yes to Let's Encrypt's subscriber agreement, asked once for the whole folder. Without it, Khalid asks at the terminal. One Let's Encrypt account serves every site, kept in `data/https/` of the sites folder, and each site keeps its own certificate in its own `sites/<domain>/data/https/`, so it moves with the site. A site a super admin adds while the server runs gets its certificate within a minute, once its DNS leads here. A site whose domain does not lead here yet is tried again every hour, and the others are served meanwhile. Plain HTTP on port 80 moves each visitor to HTTPS on their own site. `--domain` is for a single site, and a sites folder refuses it.

To keep a proxy such as Caddy in front instead, serve with `--listen 127.0.0.1:8080` and let the proxy get the certificates.

## Backing up and restoring

`khalid --site /var/lib/khalid backup --out /root/blog.tar.gz` writes one archive of the site folder with a whole copy of the database. It is safe while the service runs. It leaves out `public/` and `data/media/`, which Khalid makes again. Run it from a daily timer and copy the archives off the server.

`khalid restore /root/blog.tar.gz /var/lib/khalid-restored` makes the site again from an archive, in a folder that is new or empty. It never unpacks over a folder that holds files. To put a backup in place of the live site:

1. Stop the service: `systemctl stop khalid`.
2. Move the site folder away: `mv /var/lib/khalid /var/lib/khalid-old`.
3. Restore into its place: `khalid restore /root/blog.tar.gz /var/lib/khalid`.
4. Give the files back to the service's user: `chown -R khalid:khalid /var/lib/khalid`.
5. Start the service: `systemctl start khalid`. Delete the old folder once the blog looks right.

## Taking your data out

`khalid db export` writes every table of the site's database as a CSV file and a JSON file (two plain formats spreadsheets and reporting tools read), one pair per table, into `data/export/`. Add `--out <folder>` to write them somewhere else. Google Sheets, Excel and Looker Studio open the CSV files as they are.

Sign-in material never leaves: sessions, sign-in links, passkeys and the site's own keys are left out, and so are the hashes of passwords and keys and the secrets that sign webhooks. A text field that a spreadsheet would run as a formula starts with `'`, so opening a file runs nothing.

## Your own code and the consent banner

Some tools ask you to paste code on every page: a tag manager, an analytics snippet or a chat window. Put it in a `[code]` table in `site.toml`.

```toml
[code]
head = """
<script async src="https://www.googletagmanager.com/gtag/js?id=G-1"></script>
<script>gtag('config', 'G-1');</script>"""
body = "<script src='https://chat.example.com/widget.js' defer></script>"
```

`head` goes at the end of every page's head and may hold `script`, `style`, `meta` and `link`. `body` goes at the end of every page's body and may hold only `script`. Khalid checks both when it starts and says what to change if something does not belong.

By default this code waits for each reader's consent, which means their yes. Every page then ends with a short note and two buttons, Allow and Do not allow. The buttons are a plain form, so they work with scripts turned off. The code runs only for a reader who pressed Allow, and a post's own head code waits too. A reader's choice is kept in a cookie for 180 days, and a small form at the bottom of each page lets them change it.

For code that sets no cookie and tracks no one, add `consent = "not-needed"` to `[code]`. The code then runs for every reader and no note is shown.

You can also write the code as a list, one `[[code]]` entry per tool, each tagged with the kind of consent it needs: `preferences`, `analytics`, `marketing` or `no-tracking`. An entry with no `category` counts as `marketing`. `vendor` names who runs the tool and `policy` links to their privacy policy:

```toml
[[code]]
head = "<script async src='https://stats.example.net/a.js'></script>"
category = "analytics"
vendor = "Example Stats"
policy = "https://stats.example.net/privacy"
```

The old single `[code]` table still works: `consent = "ask"` reads as one `marketing` entry and `consent = "not-needed"` as one `no-tracking` entry. For now, when any entry waits for consent, all of the site's code waits together; the banner with a switch per kind of consent comes in a later release.

The `[privacy]` table sets how consent is asked. Every key may be left out:

```toml
[privacy]
lifetime_months = 6      # how long a reader's choice lasts, 1 to 13 months
gpc = "marketing"        # what Global Privacy Control turns off: "marketing" or "reject-all"
counts = true            # count choices per day, server mode only
external_tool = "Cookiebot"  # only if you run a certified consent tool instead

[privacy.text.en]
banner = "May we count visits?"   # your own words for the banner

[privacy.contact]
name = "Example Ltd"
email = "privacy@example.com"
address = "1 Main Street, Springfield"
```

Global Privacy Control is a signal some browsers send to say the reader does not want to be tracked or sold.

Each script gets the page's nonce, a random value the browser checks against the page's security policy, so the code is allowed to run and nothing else is. A script may load other scripts and send data to any `https` address, as analytics tools do.

A static site asks with a small script instead, because a static host cannot read a cookie. `khalid build` writes code that waits as plain text, which no browser runs. The script shows a banner with Accept all, Reject all and Choose, keeps the reader's answer in a cookie for as long as `[privacy]` says, and runs each entry only for a reader who agreed to its category. A browser that sends Global Privacy Control, a setting that asks sites not to sell or share data, keeps marketing code off unless the reader turns it on at `privacy-choices/`. Code with `category = "no-tracking"` runs for every reader, and with `external_tool` set the code runs unheld for that tool to manage. With scripts off, the banner never shows and the code never runs. The script is under 2 KB and loads only on pages that ask something: a site with code that waits, and posts with an embed. Site verification tags belong in `[seo]`, which never waits.

`khalid build` also writes `.well-known/gpc.json`, which tells browsers the site honours Global Privacy Control. For a blog in a folder it is in `_root/`, with the other files for the root of the host.

## A copy search engines should skip

A staging copy or a sample blog should not show up in search results. Add this to its `site.toml`:

```toml
[seo.index]
site = "noindex"
```

Every page then carries `<meta name="robots" content="noindex, follow">`, whatever a post's own `robots` says, and the sitemaps list no page. `robots.txt` still lets crawlers in, because a crawler that cannot read a page never sees its noindex tag. Khalid's own demos are built this way.

## Bing Webmaster Tools

Bing Webmaster Tools is Bing's dashboard for site owners. Add the site there with the address in `site.toml`, put the code Bing gives you as `bing` under `[seo.verification]` in `site.toml`, and verify. Then make an API key, the password a program uses to reach Bing for you, under Settings, API Access, and set it as `KHALID_BING_API_KEY` in the environment, the same way as the mail secrets above. The key never goes in `site.toml`.

`khalid --site <folder> bing sitemap` hands Bing the sitemap, and `khalid --site <folder> bing report` shows Bing's last crawl day, the sitemaps it holds and the addresses it had trouble with. Both take `--json`.

## Google Search Console

Search Console is Google's report of how a site does in Google search. Khalid reads it with your own Google account, and only once you opt in. It never involves SerpDad unless you connect SerpDad yourself.

1. Add the site in Search Console and verify it, with the code Google gives you as `google` under `[seo.verification]` in `site.toml`.
2. In Google Cloud, make a project, turn on the "Google Search Console API", and make an OAuth client ID of type "Desktop app" under APIs and Services, Credentials. Google shows a client ID and a client secret.
3. Get a refresh token, a long-lived permission for that client to read your numbers: in the [OAuth 2.0 Playground](https://developers.google.com/oauthplayground/), open the settings, tick "Use your own OAuth credentials" and enter the client ID and secret, then authorise the scope `https://www.googleapis.com/auth/webmasters.readonly` with the Google account that owns the site and exchange the code for tokens. Copy the refresh token.
4. Add the table to `site.toml`. `property` is needed only when it differs from `address`, such as a domain property. `report_to` is who gets the weekly email, and needs `[mail]`.

```toml
[search_console]
client_id = "1234-abcd.apps.googleusercontent.com"
property = "sc-domain:example.com"
report_to = "you@example.com"
```

5. Set `KHALID_GOOGLE_CLIENT_SECRET` and `KHALID_GOOGLE_REFRESH_TOKEN` in the environment, the same way as the mail secrets above. Neither ever goes in `site.toml`.

Then, all with `--json` when a program reads the answer:

- `khalid --site <folder> search-console check` signs in and says whether the property can be read.
- `khalid --site <folder> search-console report` shows each post's clicks, impressions, click-through rate and position for the last 28 days Google has final numbers for, posts whose clicks fell three months in a row, missing and noindex pages that still get search traffic, and the week's winners and losers.
- `khalid --site <folder> search-console post <slug>` shows one post's top searches and whether Google has it in its index.
- `khalid --site <folder> search-console index` asks Google about every published post, up to 200 a run. Google allows 2,000 of these a day.
- `khalid --site <folder> search-console follow-ups` lists the long searches, 8 words or more, that showed each post in the last 28 days. People ask AI assistants in whole questions like these, and each is one a reader brings to the post. Next to each, it names the heading that answers it, or says no heading does yet. The fix is a heading that asks the question, with a 40 to 60 word answer under it. Google leaves out searches too rare to share, so a small site may see none.
- `khalid --site <folder> search-console email` sends the weekly email now. `khalid serve` sends it on its own once a week, so you need no timer while it runs. On a static site, run the command once a week, for example from a systemd timer with `OnCalendar=Mon 08:00`, or a cron line such as `0 8 * * 1`.

The weekly email goes out when a week has passed since the last one, and the first soon after `khalid serve` starts. Khalid keeps the day it last went out in `data/search-console-email.txt`, and the command writes it too, so a restart or a run by hand never sends the email twice in a week. If Google or the mail service fails, the server tries again an hour later.

`khalid serve` reads the connection and the two secrets once, at start, so restart it after setting them. It then answers the same follow-ups at `GET /api/admin/v1/search-console/follow-ups/`, and the same report at `GET /api/admin/v1/search-console/report/`, for a key with `settings:write`. The admin shows both under Search Console, with a link from each post to its editor. The posts are read on each request, so a heading you just added counts.

The Analytics screen also shows Google's numbers for all your pages together: clicks, times shown, the share clicked and the average position, for the same 7, 28 or 90 days as the page views, against the days before. It reads them from `GET /api/admin/v1/search-console/totals/?days=`, from 1 to 90 days, 28 when left out. Google needs about three days to make a day's numbers final, so these days end three days earlier than the page views. A blog that is not connected sees a link to the Search Console screen in their place, and Google is not asked.

A refresh token stops working when you take Khalid's permission back in your Google account, or when Google lets it lapse. The commands then say so: make a new one with step 3.

## Pages AI bots read but never cite

`khalid serve` counts each page view by where the reader came from, and logs which bots read which page. `khalid --site <folder> analytics uncited` lists the pages AI bots read in the last 30 days while no reader arrived from an AI assistant, such as ChatGPT or Perplexity, in the same days. Those pages are known to the assistants but not pointed to. `--days 7` looks at a shorter period, up to the 90 days the crawl log keeps, and `--json` prints the list for other tools.

A bot reading a page never counts as a citation: only a reader sent by an AI assistant does. Bots are named by the user agent they send, which anyone can copy, so a name in the list is what the bot claimed to be. A site built as static files has no counts, so the list stays empty.

## Git mode

Git mode keeps the site in a git repository, so posts ship like code. Each draft is a branch: `khalid new "Title" --branch` starts the branch `post/<slug>`, writes the post and commits it there. Write the post, commit, push the branch and open a pull request. Merging it publishes the post.

The server then needs what was merged. Run `khalid pull` from a deploy step, or let the git host tell the server on every push:

1. Make a long random secret, for example with `openssl rand -hex 32`, and set it as `KHALID_GIT_HOOK_SECRET` in the server's environment, the same way as the mail secrets above. The secret never goes in `site.toml`.
2. On GitHub, add a webhook under Settings, Webhooks, with the payload address `https://<your host>/api/hooks/git`, or `https://<your host>/blog/api/hooks/git` for a blog under `/blog/`, the content type `application/json` and the same secret. On GitLab, add it under Settings, Webhooks, with the same address and the secret as its token, for push events.

The server answers a signed push at once and pulls in the background; the post shows as soon as the pull finishes. A push signed with anything else gets a 401 and changes nothing. Without `KHALID_GIT_HOOK_SECRET`, the address does not exist.

## Analytics in the admin

The Analytics screen in the admin shows the same counts for the last 7, 28 or 90 days: the page views against the days before, each day, where readers came from, the most read pages, the readers each AI assistant sent, and the pages AI bots read but never cite. It needs an owner or admin sign-in, because views are visitor data.

Programs read the same numbers from the Admin API with a key that has `settings:write`: `GET /api/admin/v1/analytics/visitors/?days=28`, `GET /api/admin/v1/analytics/ai-visitors/?days=28`, `GET /api/admin/v1/analytics/uncited/?days=30` and `GET /api/admin/v1/analytics/ai-bots/?days=30`, which counts how often each assistant's bots read your pages beside the readers it sent. The OpenAPI document at `/api/openapi.json` describes each answer.

## Smaller pictures with AVIF

Khalid offers every picture in WebP, which every current browser reads, with the original JPEG or PNG for the rest. AVIF files are about half the size of a JPEG, so pages load faster on a slow phone, but each wide picture takes about a second to make. AVIF is off until you turn it on in `site.toml`:

```toml
[media]
avif = true
```

The next build makes each picture's AVIF sizes once and keeps them in `data/media/`, so later builds only make them for new or changed pictures. Pages then offer AVIF first, WebP second and the original last, and each browser takes the first it can read.

You can also offer AVIF in place of WebP, which nearly halves the space pictures take on disk and in a backup:

```toml
[media]
avif = "only"
```

Pages then offer AVIF first and the original last. Every current browser reads AVIF. The few that do not, such as Safari before version 16, get one JPEG or PNG at most 1024 px wide. That file is also the picture a post shares on social networks, which show it well but suggest 1200 px.

## Photo and GIF search in the editor

Writers can find a free photo on Unsplash or a GIF on GIPHY from the editor's block menu. Both are off until you turn them on in `site.toml`:

```toml
[media]
unsplash = true
gifs = true
```

Each needs a key, the password the blog uses to ask for you. Make an app at unsplash.com/developers and set its access key as `KHALID_UNSPLASH_ACCESS_KEY`, and make one at developers.giphy.com and set its key as `KHALID_GIPHY_API_KEY`, the same way as the mail secrets above. The keys never go in `site.toml` and never reach the writer's browser. Run `khalid doctor` afterwards: its "Picture search" line says which searches are on and names any key the server cannot see. It only checks that the key is set; it does not send it to Unsplash or GIPHY.

A chosen photo or GIF loads from Unsplash or GIPHY, as their terms ask, so readers' browsers fetch that picture from there. A photo gets its photographer's name and links under it.

## Writing suggestions in the editor

The editor can suggest an SEO title, a meta description, share titles and descriptions, alt text for a picture, a fix for a check that failed, and a short list of key takeaways. Khalid has no AI model inside it. A suggestion is written by a service you choose, and nothing is sent anywhere until you choose one in `site.toml`:

```toml
[ai]
provider = "anthropic"   # "serpdad", "anthropic", "openai", or "off", the default
model = "claude-haiku-5-5"   # optional, except for "openai"
endpoint = "https://api.example.com/v1/chat/completions"   # optional
```

- `serpdad` asks your SerpDad account. It works only while SerpDad is connected on the admin's SerpDad page.
- `anthropic` asks Anthropic's Messages API with your own key. Without `model`, it uses `claude-haiku-5-5`, which is quick and cheap.
- `openai` asks any service that speaks OpenAI's chat completions, with your own key. It needs `model`, the model's name as that service spells it, and `endpoint` when the service is not OpenAI itself.

The key is the password the blog uses to ask that service. Set it as `KHALID_AI_KEY` in the server's environment, the same way as the mail secrets above. It never goes in `site.toml`, never reaches a writer's browser and is never logged.

When it is on, each suggestion sends the words it needs and nothing more: the post's title, text and focus keyword for a title or description, the failing check's message for a fix, and the file name, caption and the post's title for alt text. Every button is marked "AI suggestion", and nothing changes in a post until the writer picks one. Set `provider = "off"`, or remove the table, to switch it off again.

Programs ask the same way through the Admin API with a key that may write posts: `GET /api/admin/v1/ai/` says whether suggestions are on and ready, and `POST /api/admin/v1/ai/suggestions/` takes a `task` (`seo_title`, `meta_description`, `share_title`, `share_description`, `alt_text`, `lint_fix` or `takeaways`) and either a post's identity in `post` or the words themselves. The OpenAPI document at `/api/openapi.json` shows each field. A post can hold a key takeaways box written by hand too: `:::takeaways`, one line starting with `- ` per point, then `:::`.

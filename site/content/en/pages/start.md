---
title: Start a Khalid blog
slug: start
description: Four ways to start a Khalid blog, from a one-line install to a free blog on Cloudflare, and where to read more.
---

Pick the way that suits you. Each one gives you the same Khalid. You can move between them later, because your blog is a folder of text files.

## Install it on your computer

On Linux or a Mac, this one line downloads the newest release, checks it and puts `khalid` on your path:

```text
curl -fsSL https://raw.githubusercontent.com/thebigasif/khalid/main/scripts/install.sh | sh
```

On Windows, download the `.exe` file from [the releases page](https://github.com/thebigasif/khalid/releases). Then make a blog and open it:

```sh
khalid init blog --address https://blog.example.com --yes
khalid --site blog serve
```

The blog is now at `http://127.0.0.1:8080/` on your computer. To sign in to the admin, run `khalid --site blog staff owner` with your email and name, and open the link it shows.

## Run it in Docker

Docker runs programs in a sealed box, the same on every machine. Each release publishes an image for x86_64 and ARM computers:

```sh
mkdir site
docker run --rm -u "$(id -u):$(id -g)" -v "$PWD/site:/site" ghcr.io/thebigasif/khalid:latest init /site --address https://blog.example.com --yes
docker run -d -u "$(id -u):$(id -g)" -p 8080:8080 -v "$PWD/site:/site" ghcr.io/thebigasif/khalid:latest
```

The blog is then at `http://localhost:8080/`, and its files stay in the `site` folder beside you.

The repository also has a Compose file and ready files for Fly.io, Railway, Coolify and a Hetzner Cloud server.

## Put a free blog on Cloudflare

Cloudflare can host a Khalid blog of plain files on its free plan. Select the button below. Cloudflare copies a small ready blog into your GitHub account and puts it online. Every change you push builds it again.

::button{href="https://deploy.workers.cloudflare.com/?url=https://github.com/thebigasif/khalid/tree/main/deploy/cloudflare" label="Deploy to Cloudflare"}

A blog of plain files has no admin, comments, forms, sign-ups, search or scheduling. For those, run Khalid as a server.

## Build it from source

You need git, Rust, a C compiler and Node.js. Node.js builds the admin; without it you get a working blog with no admin.

```sh
git clone https://github.com/thebigasif/khalid.git
cd khalid/admin
npm ci
npm run build
cd ..
cargo install --path crates/khalid --locked
```

## Read more

- [Getting started](https://github.com/thebigasif/khalid/blob/main/docs/getting-started.md): a site, a first post, the admin, checks, imports and backups.
- [Deploy](https://github.com/thebigasif/khalid/blob/main/docs/deploy.md): a server, Docker, hosting platforms and HTTPS.
- [Cloudflare](https://github.com/thebigasif/khalid/blob/main/docs/deploy/cloudflare.md): the button and the Cloudflare Pages steps.
- [Newsletters](https://github.com/thebigasif/khalid/blob/main/docs/newsletters.md): sending posts to subscribers.
- [Templates](https://github.com/thebigasif/khalid/blob/main/docs/templates.md): picking a layout and its options.

Not sure yet? Read [what Khalid does](/khalid/features/) or [see the demo blog](/khalid/demo/).

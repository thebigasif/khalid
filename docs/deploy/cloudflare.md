# A free Khalid blog on Cloudflare

Cloudflare can build your blog from GitHub and serve it to readers around the world for free. There are two ways in. The button does every step for you. The Pages steps follow the classic GitHub and Cloudflare Pages guide, for people who like to see each setting.

Both start from `deploy/cloudflare/` in this repository: a small, ready blog with a welcome post, an about page and an author, in Khalid's classic template. On each build, Cloudflare installs one pinned Khalid release, checks the download against the release's checksums, and runs `khalid build`. The pages land in `public/`, and Cloudflare serves that folder as plain files.

The blog runs in static mode: Cloudflare serves files, and nothing of Khalid keeps running. You write by changing the text files in your GitHub repository, and every change you push builds the blog again. The admin, comments, forms and the newsletter need a running server; `docs/deploy.md` shows the hosts for that.

## The button

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/thebigasif/khalid/tree/main/deploy/cloudflare)

1. Select the button. Sign in to Cloudflare, or make a free account.
2. Connect your GitHub or GitLab account when Cloudflare asks. Cloudflare copies the starter folder into a new repository in your account, and that repository is your blog from now on. Only the folder is copied, not the whole Khalid project.
3. On the setup page, pick a name for the repository and for the Worker. The Worker is Cloudflare's name for the project that serves the blog, and its name becomes part of the address. Leave the build command as `npm run build` and the deploy command as `npx wrangler deploy`.
4. Select **Create and deploy**. The first build takes a minute or two.

The blog is then at `https://<the Worker's name>.<your account's subdomain>.workers.dev`. Cloudflare shows the address when the deploy ends.

## Cloudflare Pages, step by step

1. On GitHub, fork `thebigasif/khalid`, so you have your own copy. Or use the repository the button made, which holds only the blog.
2. In the Cloudflare dashboard, go to **Workers & Pages**, select **Create application**, then **Pages**, then **Connect to Git**.
3. Select **GitHub**, allow Cloudflare to read your repository, pick it, and select **Begin setup**.
4. Fill in the build settings:

| Setting | Value |
|---|---|
| Project name | Your blog's name, such as `my-blog`. It becomes `my-blog.pages.dev` |
| Production branch | `main` |
| Framework preset | None |
| Build command | `sh build.sh` |
| Build output directory | `public` |
| Root directory (advanced), Path | `deploy/cloudflare` for a fork of Khalid. Leave it empty for the button's repository |
| Environment variable | `KHALID_ADDRESS` set to `https://my-blog.pages.dev` |

5. Select **Save and Deploy**.

The blog is at `https://<project name>.pages.dev` a minute or two later.

## Set your blog's address

Every page tells search engines its one true address, and Khalid takes it from `address` in `site.toml`. The starter says `https://blog.example.com`, so change it to yours after the first deploy:

- In your repository on GitHub, open `site.toml`, select the pencil, put your address after `address`, and commit. Cloudflare builds the blog again.
- Or, in Cloudflare, add the build variable `KHALID_ADDRESS` with your address. For a Worker it is under **Settings**, **Build**, **Variables and secrets**. Then build again.

## Use your own domain

Your domain must use Cloudflare for its DNS, the service that says which server a name points to.

- For the button's Worker: in **Workers & Pages**, select the Worker, go to **Settings**, **Domains & Routes**, **Add**, **Custom Domain**, enter the domain, and select **Add Custom Domain**.
- For Pages: select the project, then **Custom domains**, and follow the steps.

Then set the blog's address to the new domain, as above.

## Write a post

Add a file to `content/en/posts/` in your repository, as the welcome post shows, and commit it. Khalid checks every post before it builds. A post that breaks a basic search rule, such as a missing description, is left out, and the build log in Cloudflare says why and how to fix it. `docs/getting-started.md` explains every field of a post.

## Update Khalid

`build.sh` names the Khalid release it installs, such as `version="v1.1.1"`. A new release never changes your blog until you change that line to the newer release from the [releases page](https://github.com/thebigasif/khalid/releases) and commit.

## What is free

Cloudflare's free plan serves the files of a static site with no limit on visits. Workers Builds gives 3,000 build minutes a month, one build at a time, and stops a build after 20 minutes. A Khalid build takes well under a minute.

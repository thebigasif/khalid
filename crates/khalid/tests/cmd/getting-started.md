# The first steps of docs/getting-started.md

`tests/doc_examples.rs` runs each `console` block below with trycmd, in a
fresh copy of `getting-started.in/`, an empty folder. The `.out/` folder
beside this file is what makes trycmd use that fresh copy.

```console
$ khalid init blog --address https://blog.example.com --yes
Made a Khalid site in blog for https://blog.example.com/.
It holds site.toml, routes.toml, theme/tokens.toml, theme/templates/, content/en/posts/ and drafts/en/.
Next, write your first post with `khalid --site blog new --draft "Your title" --author "Your name"`. It waits in drafts/ until `khalid --site blog publish <slug>` takes it live.
To sign in to the admin, run `khalid --site blog staff owner --email you@example.com --name "Your name"`.

$ khalid --site blog lint
0 blocks and 0 warnings.
Every post scores 100 out of 100.

$ khalid --site blog build
Built [..] pages in blog/public.
...

$ khalid completions zsh
#compdef khalid
...
```

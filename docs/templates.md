# Templates: the look of your blog

A template sets the layout and features of your blog's pages: where the contents list sits, which cards the home page shows, and the sizes of text and space. Your colours, fonts and logo still come from your main site through theme sync, so every template looks like your brand.

Every template is built into Khalid. You pick one in `site.toml` and adjust that template's options. You cannot mix parts of different templates.

## Picking a template

```toml
[template]
name = "classic"
```

Without a `[template]` table your blog uses `classic`, the look Khalid has always had. Updating Khalid never changes your template.

Classic is the only template today. The ten templates in the templates spec arrive one or two at a time, and each one lists its options here.

## Options

Each template has its own options, set under `[template.options]`:

```toml
[template]
name = "classic"
[template.options]
# classic takes no options
```

An option a template does not know, or a value it does not take, stops every command with the line in `site.toml` and the fix. For example:

> site.toml, line 12, field "lead_image": the classic template has no option lead_image; it takes no options, so remove the line

An option you leave out takes the template's default.

## Reader features

A template may switch some `[reader]` features on or off by default, such as the reading time. A key you write in `[reader]` always wins over the template.

## Text sizes and spacing

`theme/tokens.toml` takes two more tables. Each value is optional; a value you leave out takes the template's default.

```toml
[type]
body = "1.0625rem 1.125rem"   # smallest and largest, or one size such as "1.125rem"
h1 = "2.25rem 3.25rem"
h2 = "1.5rem 2rem"
h3 = "1.25rem"
h4 = "1.125rem"
small = "0.9375rem"           # dates, captions and other small text
line_height = 1.65            # 1 to 2.5
heading_weight = 700          # 100 to 900, in steps of 100

[space]
unit = "0.5rem"               # the smallest gap
section = "3rem"              # the space between large parts of a page
```

Two sizes grow with the screen: the first on a phone 320 pixels wide, the second on a screen 1280 pixels wide and wider, and in between in step. Sizes are in `rem`, the browser's base text size.

`khalid theme sync` keeps both tables as you wrote them. A file in `theme/templates/` still replaces the template's file of the same name.

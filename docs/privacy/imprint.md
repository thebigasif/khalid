# The imprint

An imprint says who runs a website and how to reach them. German law asks for one, called an Impressum, on most websites run from Germany (§ 5 of the Digital Services Act, DDG). Khalid writes it for you at `<mount>impressum/`, in each of the site's languages, and links it from the footer of every page. It is a starting point, not legal advice.

## Turning it on

Add a `[privacy.imprint]` table to `site.toml`. Even an empty one turns the page on. The page takes your name, postal address and email from `[privacy.contact]`, which the [privacy page](privacy-page.md) uses too, so you write them once:

```toml
[privacy.contact]
name = "Notes GmbH"
email = "hello@example.com"
address = "Hauptstraße 1, 10115 Berlin"

[privacy.imprint]
phone = "+49 30 1234567"
register = "Amtsgericht Berlin, HRB 12345"
vat_id = "DE123456789"
responsible = "Erika Muster, Hauptstraße 1, 10115 Berlin"
```

If the table is there but a name, a postal address or an email is missing, Khalid stops with the field to fill in. A site without the table has no imprint and no footer link.

You can do the same in the admin: open Privacy, then Imprint, turn on "Add an imprint page", fill in the lines you need and save. Your name, postal address and email go on the Contact tab. Until all three are there, the Imprint tab says so and saving is refused. Turning the switch off and saving removes `[privacy.imprint]` and the page. The blog shows the change within a few seconds, with no restart.

## The lines you can add

| Setting | What it shows |
|---|---|
| `phone` | A phone number, a second quick way to reach you. |
| `register` | Your trade register entry, if you are in one, such as a GmbH. |
| `vat_id` | Your VAT identification number, if you have one. |
| `responsible` | Who answers for the articles, with their address, as § 18 (2) of the Media State Treaty (MStV) asks of journalistic sites. |

A line you leave out is left off the page.

## Writing your own page

Write `content/<language>/pages/impressum.md` with the slug `impressum`, and it replaces the generated page in that language. If your footer settings already link to that address, Khalid adds no second link.

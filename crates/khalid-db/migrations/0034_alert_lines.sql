-- The line each raised alert finding says (backlog 2026-11-12, row 15),
-- such as "Old post (en.old-post), last changed 2024-03-01", so the Admin
-- API and the dashboard can list what is raised to an owner with no email.
-- Rows raised before this keep an empty line; the list shows their key's
-- subject instead. Visitor data: a rebuild never touches it.
ALTER TABLE alerts_raised ADD COLUMN line TEXT NOT NULL DEFAULT '';

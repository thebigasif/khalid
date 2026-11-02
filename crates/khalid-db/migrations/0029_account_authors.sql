-- The author profile a staff account writes as, by the profile's identity
-- such as en.asif, so a new post names them and "Your account" edits their
-- profile (docs/adr/0000-an-account-writes-as-one-author.md). NULL means
-- the account has no profile yet. Migrations are forward-only (RULES.md
-- BL-DB-2).
ALTER TABLE users ADD COLUMN author TEXT
    CHECK (author IS NULL OR (length(author) BETWEEN 3 AND 200 AND instr(author, '.') > 1));

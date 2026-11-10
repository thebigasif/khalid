-- Form answer emails that could not be sent (G40). Visitor data: a
-- rebuild keeps it. A row is added when the first send fails, tried again
-- with the webhooks' backoff over about a day, and removed once the email
-- is sent. next_at is NULL once Khalid gave up; given-up rows are kept 30
-- days so `khalid doctor` can say how many never arrived.
CREATE TABLE form_mail (
    id INTEGER PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES sites (id) ON DELETE CASCADE,
    form TEXT NOT NULL CHECK (length(form) > 0),
    to_address TEXT NOT NULL CHECK (length(to_address) > 0),
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    first_at TEXT NOT NULL,
    failures INTEGER NOT NULL CHECK (failures > 0),
    last_at TEXT NOT NULL CHECK (last_at >= first_at),
    last_error TEXT NOT NULL,
    next_at TEXT CHECK (next_at IS NULL OR next_at > last_at)
) STRICT;

CREATE INDEX form_mail_due ON form_mail (site_id, next_at);

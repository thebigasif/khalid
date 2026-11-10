-- Welcome sequences and link clicks (G39). Visitor data: a rebuild never
-- touches it.

-- Which step of a newsletter's welcome sequence each subscriber was sent,
-- by the day it is due after they joined: 0 for welcome.md, 3 for
-- welcome-day-3.md. The row is written before the email goes out, so no
-- step is ever sent twice.
CREATE TABLE welcome_steps (
    site_id INTEGER NOT NULL REFERENCES sites (id) ON DELETE CASCADE,
    subscriber_id INTEGER NOT NULL REFERENCES subscribers (id) ON DELETE CASCADE,
    list TEXT NOT NULL CHECK (length(list) > 0),
    day INTEGER NOT NULL CHECK (day >= 0),
    sent_at TEXT NOT NULL CHECK (julianday(sent_at) IS NOT NULL),
    PRIMARY KEY (subscriber_id, list, day)
) STRICT, WITHOUT ROWID;

-- The links in one email that went out, when the site counts clicks.
-- Every reader's copy has the same link, so a click is counted against the
-- email and the link, never against a person.
CREATE TABLE newsletter_links (
    site_id INTEGER NOT NULL REFERENCES sites (id) ON DELETE CASCADE,
    send_id INTEGER NOT NULL REFERENCES newsletter_sends (id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position > 0),
    url TEXT NOT NULL CHECK (length(url) > 0),
    clicks INTEGER NOT NULL DEFAULT 0 CHECK (clicks >= 0),
    PRIMARY KEY (send_id, position)
) STRICT, WITHOUT ROWID;

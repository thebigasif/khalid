-- Consent proof (privacy plan U7, R15, KTD8). Visitor data: a rebuild
-- keeps it. Neither table holds an address or a reader identifier.
--
-- consent_versions: each version of what readers were shown, keyed by a
-- hash of the banner and category text, the categories, the vendors and
-- the lifetime, with the shown settings as JSON and the first and last
-- day it was in force. Kept for as long as the site exists.
CREATE TABLE consent_versions (
    id INTEGER PRIMARY KEY,
    site_id INTEGER NOT NULL REFERENCES sites (id) ON DELETE CASCADE,
    version TEXT NOT NULL CHECK (length(version) = 16),
    shown TEXT NOT NULL CHECK (json_valid(shown)),
    first_day TEXT NOT NULL CHECK (date(first_day) = first_day),
    last_day TEXT NOT NULL CHECK (date(last_day) = last_day AND last_day >= first_day),
    UNIQUE (site_id, version)
) STRICT;

-- consent_counts: per day and version, how many readers answered and how
-- many of them allowed each optional category. Kept 25 months.
CREATE TABLE consent_counts (
    site_id INTEGER NOT NULL REFERENCES sites (id) ON DELETE CASCADE,
    day TEXT NOT NULL CHECK (date(day) = day),
    version_id INTEGER NOT NULL REFERENCES consent_versions (id) ON DELETE CASCADE,
    choices INTEGER NOT NULL CHECK (choices > 0),
    preferences INTEGER NOT NULL CHECK (preferences BETWEEN 0 AND choices),
    analytics INTEGER NOT NULL CHECK (analytics BETWEEN 0 AND choices),
    marketing INTEGER NOT NULL CHECK (marketing BETWEEN 0 AND choices),
    external_media INTEGER NOT NULL CHECK (external_media BETWEEN 0 AND choices),
    PRIMARY KEY (site_id, day, version_id)
) STRICT, WITHOUT ROWID;

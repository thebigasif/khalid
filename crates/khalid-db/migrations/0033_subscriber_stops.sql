-- Bounces and spam complaints (gap audit 5, row 7). Visitor data: a rebuild
-- never touches it.

-- Why the mail service told Khalid to stop mailing a subscriber: 'bounced'
-- when the address does not take mail, 'complained' when its owner marked
-- an email as spam. Such a subscriber is unsubscribed, and the sign-up form
-- sends them nothing again. NULL for everyone else.
ALTER TABLE subscribers ADD COLUMN stopped_because TEXT
    CHECK (stopped_because IS NULL OR stopped_because IN ('bounced', 'complained'));

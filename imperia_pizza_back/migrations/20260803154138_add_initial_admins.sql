-- Add migration script here
INSERT INTO admins (telegram_id, name)
VALUES (8096030502, 'тима'),
    (10770672441, 'ому') ON CONFLICT (telegram_id) DO NOTHING;
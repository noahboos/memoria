ALTER TABLE card_templates DROP COLUMN markup;
ALTER TABLE card_templates ADD COLUMN front_markup TEXT NOT NULL DEFAULT '';
ALTER TABLE card_templates ADD COLUMN back_markup TEXT NOT NULL DEFAULT '';

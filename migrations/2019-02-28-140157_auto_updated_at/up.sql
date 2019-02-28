ALTER TABLE myusers ALTER COLUMN created_at SET NOT NULL;
ALTER TABLE myusers ALTER COLUMN created_at SET DEFAULT NOW();
ALTER TABLE myusers ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE myusers ALTER COLUMN updated_at SET DEFAULT NOW();
SELECT diesel_manage_updated_at('myusers');

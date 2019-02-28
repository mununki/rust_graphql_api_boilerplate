CREATE TABLE myuser (
	id SERIAL PRIMARY KEY,
	email VARCHAR(70) UNIQUE NOT NULL,
	first_name VARCHAR(50) NOT NULL,
	last_name VARCHAR(50) NOT NULL,
	bio TEXT,
	avatar VARCHAR,
	created_at TIMESTAMPTZ,
	updated_at TIMESTAMPTZ
)

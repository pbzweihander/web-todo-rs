CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 'f'
)

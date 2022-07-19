-- Add migration script here
CREATE TABLE integrations(
  id SERIAL PRIMARY KEY,
  webhook_url TEXT not null,
  success_url TEXT not null
)

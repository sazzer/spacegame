CREATE TABLE players (
  player_id uuid PRIMARY KEY,
  version uuid NOT NULL,
  created timestamp WITH time zone NOT NULL,
  updated timestamp WITH time zone NOT NULL,
  name text NOT NULL,
  avatar_url text NOT NULL,
  authentication_providers jsonb NOT NULL
);


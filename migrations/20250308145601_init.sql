CREATE TABLE question (
  id INTEGER PRIMARY KEY,
  added_on INTEGER NOT NULL DEFAULT CURRENT_TIMESTAMP,
  approved INTEGER NOT NULL DEFAULT FALSE,  -- 0 = false, 1 = true
  author_id INTEGER NOT NULL,
  guild_id INTEGER NOT NULL,
  exhausted INTEGER NOT NULL DEFAULT FALSE,
  deleted INTEGER NOT NULL DEFAULT FALSE,
  content TEXT NOT NULL
) STRICT;

CREATE TABLE config (
  guild_id INTEGER PRIMARY KEY,
  qotd_channel_id INTEGER,
  queue_channel_id INTEGER,
  schedule_frequency_hours INTEGER,
  schedule_first_post_timestamp INTEGER,
  mod_role_id INTEGER,
  ping_role_id INTEGER,
  blacklist_role_id INTEGER,
  whitelist_role_id INTEGER,
  autothread INTEGER NOT NULL DEFAULT FALSE,
  autoapprove INTEGER NOT NULL DEFAULT FALSE
) STRICT;


CREATE TABLE qotd_history (
  id INTEGER PRIMARY KEY,
  question_id INTEGER NOT NULL,
  posted_on INTEGER NOT NULL,
  guild_id INTEGER NOT NULL
) STRICT;

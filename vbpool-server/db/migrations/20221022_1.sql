--  PRAGMA foreign_keys = ON;

CREATE TABLE pool_forms (
  pool_form_id INTEGER PRIMARY KEY AUTOINCREMENT,
  pool_form_name VARCHAR NOT NULL,
  pool_form_user_id INTEGER NOT NULL,
  pool_form_is_paid BOOLEAN NOT NULL,
  pool_form_json VARCHAR NOT NULL
);


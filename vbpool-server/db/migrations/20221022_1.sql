--  PRAGMA foreign_keys = ON;

CREATE TABLE pool_forms (
  pool_form_id INTEGER PRIMARY KEY AUTOINCREMENT,
  pool_form_name VARCHAR NOT NULL,
  pool_form_user_id INTEGER NOT NULL,
  pool_form_is_paid BOOLEAN NOT NULL
);

CREATE TABLE pool_form_group_stage (
  pool_form_group_stage_id INTEGER PRIMARY KEY AUTOINCREMENT,
  pool_form_id INTEGER NOT NULL,
  pool_form_group_stage_pos_1 VARCHAR NOT NULL,
  pool_form_group_stage_pos_2 VARCHAR NOT NULL,
  pool_form_group_stage_pos_3 VARCHAR NOT NULL,
  pool_form_group_stage_pos_4 VARCHAR NOT NULL,
  FOREIGN KEY (pool_form_id)
    REFERENCES pool_forms (pool_form_id)
);


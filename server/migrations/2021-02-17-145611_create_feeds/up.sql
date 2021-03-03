CREATE TABLE feeds (
  id VARCHAR NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  link VARCHAR NOT NULL,
  node_selector VARCHAR NOT NULL,
  title_selector VARCHAR,
  link_selector VARCHAR
)

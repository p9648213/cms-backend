-- Your SQL goes here
CREATE TABLE websites (
  id SERIAL PRIMARY KEY,
  agency_id INTEGER,
  name VARCHAR(100),
  domain VARCHAR(100) UNIQUE,
  status VARCHAR(20),
  CONSTRAINT fk_agencies
    FOREIGN KEY(agency_id) 
      REFERENCES agencies(id)
)
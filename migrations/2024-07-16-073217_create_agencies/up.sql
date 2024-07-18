-- Your SQL goes here
CREATE TABLE agencies (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255),
  email VARCHAR(255),
  phone_number VARCHAR(20),
  address VARCHAR(255),
  tax_code VARCHAR(20),
  identifier_id INTEGER UNIQUE,
  api_key VARCHAR(255),
  invoice_address VARCHAR(500),
  invoice_email VARCHAR(255),
  invoice_company_name VARCHAR(255)
)
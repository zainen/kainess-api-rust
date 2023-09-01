CREATE TABLE accession_number (
  id SERIAL PRIMARY KEY,
  number VARCHAR(9),
  compound_id int,
  created_at Timestamp,
  updated_at Timestamp,
  source_id int,
  source_type VARCHAR(16)
);

INSERT INTO accession_number 
SELECT 
  (data->>'id')::integer, 
  (data->>'number')::VARCHAR(9), 
  (data->>'compound_id')::integer, 
  (data->>'created_at')::Timestamp,
  (data->>'updated_at')::Timestamp,
  (data->>'source_id')::integer,
  (data->>'source_type')::VARCHAR(16)
  from temp;

CREATE TABLE compound (
  (data->>'id')::integer,
  (data->>'public_id')::VARCHAR(9),
  data->>'name',
  (data->>'state')::VARCHAR(20),
  (data->>'')::
  (data->>'')::
  (data->>'')::
  (data->>'')::
)
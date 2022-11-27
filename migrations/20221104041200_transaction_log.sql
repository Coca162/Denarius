CREATE TABLE transaction_log(
  id UUID NOT NULL PRIMARY KEY,
  from_id UUID NOT NULL,
  to_id UUID NOT NULL,
  amount BIGINT NOT NULL,

  CONSTRAINT fk_from FOREIGN KEY(from_id) REFERENCES account(id),
  CONSTRAINT fk_to FOREIGN KEY(to_id) REFERENCES account(id)
);
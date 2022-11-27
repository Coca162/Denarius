CREATE TABLE organisation(
  id UUID NOT NULL PRIMARY KEY,
  owner UUID NOT NULL,
  public BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_id FOREIGN KEY(id) REFERENCES account(id)
);

CREATE TABLE organisation_member(
  member_id UUID NOT NULL,
  organisation_id UUID NOT NULL,
  --position TEXT NOT NULL DEFAULT '',
  inviter BOOLEAN NOT NULL DEFAULT FALSE,
  accountant BOOLEAN NOT NULL DEFAULT FALSE,
  administrator BOOLEAN NOT NULL DEFAULT FALSE,
  creation_time TIMESTAMP NOT NULL DEFAULT (NOW() AT TIME ZONE 'utc'),

  PRIMARY KEY(member_id, organisation_id),
  CONSTRAINT fk_member FOREIGN KEY(member_id) REFERENCES account(id),
  CONSTRAINT fk_organisation FOREIGN KEY(organisation_id) REFERENCES organisation(id)
);
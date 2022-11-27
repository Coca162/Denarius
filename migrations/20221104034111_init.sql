CREATE TABLE account(
  id UUID NOT NULL PRIMARY KEY,
  balance BIGINT NOT NULL DEFAULT 0
  --name VARCHAR(32) NOT NULL,
  --description TEXT NOT NULL DEFAULT '',
);

CREATE TABLE person(
  id UUID NOT NULL PRIMARY KEY,
  discord_id BYTEA NOT NULL UNIQUE,
  --account_in_use UUID NOT NULL,

  CONSTRAINT fk_id FOREIGN KEY(id) REFERENCES account(id)
  --CONSTRAINT fk_account_in_use FOREIGN KEY(account_in_use) REFERENCES account(id)
);

/*
CREATE TABLE organisation(
  id UUID NOT NULL PRIMARY KEY,
  owner UUID NOT NULL,
  public BOOLEAN NOT NULL DEFAULT FALSE,

  CONSTRAINT fk_id FOREIGN KEY(id) REFERENCES account(id)
);

CREATE TABLE friend(
  friend_id UUID NOT NULL,
  person_id UUID NOT NULL,
  accountant BOOLEAN NOT NULL DEFAULT FALSE,

  PRIMARY KEY(friend_id, person_id),
  CONSTRAINT fk_member FOREIGN KEY(friend_id) REFERENCES person(id),
  CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES person(id)
);

CREATE TABLE organisation_member(
  member_id UUID NOT NULL,
  organisation_id UUID NOT NULL,
  --position TEXT NOT NULL DEFAULT '',
  inviter BOOLEAN NOT NULL DEFAULT FALSE,
  accountant BOOLEAN NOT NULL DEFAULT FALSE,
  administrator BOOLEAN NOT NULL DEFAULT FALSE,

  PRIMARY KEY(member_id, organisation_id),
  CONSTRAINT fk_member FOREIGN KEY(member_id) REFERENCES account(id),
  CONSTRAINT fk_organisation FOREIGN KEY(organisation_id) REFERENCES organisation(id)
);
*/
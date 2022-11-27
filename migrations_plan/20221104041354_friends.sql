CREATE TABLE friend(
  friend_id UUID NOT NULL,
  person_id UUID NOT NULL,

  PRIMARY KEY(friend_id, person_id),
  CONSTRAINT fk_member FOREIGN KEY(friend_id) REFERENCES person(id),
  CONSTRAINT fk_user FOREIGN KEY(person_id) REFERENCES person(id)
);
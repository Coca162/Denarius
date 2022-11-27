ALTER TABLE friend
ADD accountant BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE person
ADD account_in_use UUID NOT NULL;

ALTER TABLE person
ADD CONSTRAINT fk_account_in_use FOREIGN KEY(account_in_use) REFERENCES account(id);
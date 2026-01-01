CREATE TABLE IF NOT EXISTS new_identities (
  address TEXT NOT NULL,
  identity TEXT NOT NULL CHECK (identity IN ('aci', 'pni')),
  record BLOB NOT NULL,
  PRIMARY KEY (address, identity)
);
INSERT INTO new_identities(address, identity, record) SELECT DISTINCT address, identity, record FROM identities;
DROP TABLE identities;
ALTER TABLE new_identities RENAME TO identities;

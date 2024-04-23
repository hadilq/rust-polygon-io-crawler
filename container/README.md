To launch the `postgres`

```shell
$ nix-shell
$ cd container
$ mkdir data
$ source .env
$ podman run --pod rust-crawler-pod\
  --volume=$PWD/data:$DB_DATA\
  -e POSTGRES_DB=$DB_NAME -e POSTGRES_USER="$DB_USER"\
  -e POSTGRES_PASSWORD="$DB_PASSWORD"\
  -e PGDATA=$DB_DATA -e DB_PORT=$DB_PORT\
  -e DB_TEST_DATA=$DB_TEST_DATA -e DB_TEST_NAME=$DB_TEST_NAME\
  --name postgresdb -d postgres
```

then run
```shell
$ tmux neww podman exec -it postgresdb bash
# mkdir -p $DB_TEST_DATA && chown postgres:postgres $DB_TEST_DATA

# cat <<EOF > init.sql.tmp
\db+
\l
CREATE TABLE IF NOT EXISTS ticker_details (
  id                             SERIAL PRIMARY KEY,
  active                         BOOL    NOT NULL,
  date                           DATE    NOT NULL,
  snp                            VARCHAR NOT NULL,
  share_class_shares_outstanding BIGINT  NOT NULL,
  total_employees                INT     NOT NULL,
  weighted_shares_outstanding    BIGINT  NOT NULL,
  UNIQUE (snp, date)
);

CREATE TABLESPACE ts_test
  LOCATION 'DB_TEST_DATA';
CREATE DATABASE DB_TEST_NAME
  TABLESPACE ts_test;
\db+ ts_test
\c DB_TEST_NAME
CREATE TABLE IF NOT EXISTS ticker_details (
  id                             SERIAL PRIMARY KEY,
  active                         BOOL    NOT NULL,
  date                           DATE    NOT NULL,
  snp                            VARCHAR NOT NULL,
  share_class_shares_outstanding BIGINT  NOT NULL,
  total_employees                INT     NOT NULL,
  weighted_shares_outstanding    BIGINT  NOT NULL,
  UNIQUE (snp, date)
);
EOF

# cat init.sql.tmp | \
	sed -e "s/DB_TEST_NAME/$DB_TEST_NAME/g" | \
	sed -e "s@DB_TEST_DATA@$DB_TEST_DATA@g" > init.sql

# psql -U $POSTGRES_USER $POSTGRES_DB -a -f init.sql

```

Done!


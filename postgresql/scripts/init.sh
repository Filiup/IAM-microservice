#!/bin/sh
export PGPASSWORD=$PG_PASSWORD

pg_restore --username "$PG_USER" --dbname "$PG_DB" --no-owner --no-acl --role="$PG_USER" --exit-on-error -j 8 --verbose /tmp/backups/iam.backup


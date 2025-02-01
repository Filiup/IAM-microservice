#!/bin/sh
export PGPASSWORD=$PG_PASSWORD

if [ -z "$1" ]; then
    echo "Please enter name of the backup file."
    exit
fi

psql --host "$PG_HOST" --port 5432 --username "$PG_USER" --dbname "postgres" --no-password --pset=format=unaligned -c "SELECT pid, pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '$PG_DB' AND pid <> pg_backend_pid();"
if [ $? -ne 0 ]; then
  echo "Failed to terminate database sessions"
  exit 1
fi

psql --host "$PG_HOST" --port 5432 --username "$PG_USER" --dbname "postgres" --no-password --pset=format=unaligned -c "DROP DATABASE IF EXISTS $PG_DB;"
if [ $? -ne 0 ]; then
  echo "Failed to drop the database"
  exit 1
fi

psql --host "$PG_HOST" --port 5432 --username "$PG_USER" --dbname "postgres" --no-password --pset=format=unaligned -c "CREATE DATABASE $PG_DB;"
if [ $? -ne 0 ]; then
  echo "Failed to create the database"
  exit 1
fi

pg_restore --username "$PG_USER" --dbname "$PG_DB" --no-owner --no-acl --role="$PG_USER" --exit-on-error -j 8 --verbose /tmp/backups/$1.backup
if [ $? -ne 0 ]; then
  echo "Database restore failed"
  exit 1
fi

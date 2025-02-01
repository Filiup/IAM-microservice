
#!/bin/sh
export PGPASSWORD=$PG_PASSWORD


if [ -z "$1" ]; then
    echo "Please enter name of the backup file."
    exit
fi

pg_dump --host "$PG_HOST" --port "$PG_PORT" --username "$PG_USER" --format custom --blobs --verbose --file /tmp/backups/$1.backup "$PG_DB"

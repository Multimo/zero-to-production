#! usr/bin/env bash

set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to postgres
DB_USER="${POSTGRES_USER:=postgress}"
# Check if a custom password has been set, otherwise default to password
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom db name has been set, otherwise default to newsletter
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to 5432
DB_PORT="${POSTGRES_PORT:=5432}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
    # ^ increased the number of connections for testing purposes

# keep pinging postgres until its ready
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres stil not ready sleeping 1 second"
    sleep 1
done

>&2 echo "Postgress is up and running on port ${DB_PORT}!"



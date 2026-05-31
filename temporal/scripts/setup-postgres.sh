#!/bin/sh
set -eu

: "${DB_PORT:?DB_PORT must be set}"
: "${DBNAME:?DBNAME must be set}"
: "${VISIBILITY_DBNAME:?VISIBILITY_DBNAME must be set}"
: "${POSTGRES_SEEDS:?POSTGRES_SEEDS must be set}"
: "${POSTGRES_USER:?POSTGRES_USER must be set}"
: "${POSTGRES_PWD:?POSTGRES_PWD must be set}"

export SQL_PASSWORD="${POSTGRES_PWD}"

until nc -z -w 5 "${POSTGRES_SEEDS%%,*}" "${DB_PORT}"; do
  sleep 2
done

run_sql_tool() {
  temporal-sql-tool --plugin postgres12 --ep "${POSTGRES_SEEDS}" -u "${POSTGRES_USER}" -p "${DB_PORT}" "$@"
}

setup_database() {
  database_name="$1"
  schema_dir="$2"
  run_sql_tool --db "${database_name}" create || true
  run_sql_tool --db "${database_name}" setup-schema -v 0.0
  run_sql_tool --db "${database_name}" update-schema -d "${schema_dir}"
}

setup_database "${DBNAME}" /etc/temporal/schema/postgresql/v12/temporal/versioned
setup_database "${VISIBILITY_DBNAME}" /etc/temporal/schema/postgresql/v12/visibility/versioned


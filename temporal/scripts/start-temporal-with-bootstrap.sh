#!/bin/sh
set -eu

: "${DB_PORT:?DB_PORT must be set}"
: "${DBNAME:?DBNAME must be set}"
: "${VISIBILITY_DBNAME:?VISIBILITY_DBNAME must be set}"
: "${POSTGRES_SEEDS:?POSTGRES_SEEDS must be set}"
: "${POSTGRES_USER:?POSTGRES_USER must be set}"
: "${POSTGRES_PWD:?POSTGRES_PWD must be set}"
: "${TEMPORAL_ADDRESS:?TEMPORAL_ADDRESS must be set}"
: "${TEMPORAL_NAMESPACES:?TEMPORAL_NAMESPACES must be set}"

server_pid=""

terminate() {
  if [ -n "${server_pid}" ] && kill -0 "${server_pid}" >/dev/null 2>&1; then
    kill "${server_pid}" >/dev/null 2>&1 || true
    wait "${server_pid}" || true
  fi
}

trap terminate INT TERM HUP

/opt/wara-temporal/scripts/setup-postgres.sh

/etc/temporal/entrypoint.sh &
server_pid=$!

if ! /opt/wara-temporal/scripts/create-namespace.sh; then
  terminate
  exit 1
fi

wait "${server_pid}"


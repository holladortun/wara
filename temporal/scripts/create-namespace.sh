#!/bin/sh
set -eu

: "${TEMPORAL_NAMESPACES:?TEMPORAL_NAMESPACES must be set}"
: "${TEMPORAL_ADDRESS:?TEMPORAL_ADDRESS must be set}"

TEMPORAL_NAMESPACE_RETENTION="168h"
host="${TEMPORAL_ADDRESS%%:*}"
port="${TEMPORAL_ADDRESS##*:}"

attempt=1
while [ "${attempt}" -le 60 ]; do
  if nc -z -w 5 "${host}" "${port}"; then
    break
  fi
  sleep 2
  attempt=$((attempt + 1))
done

attempt=1
while [ "${attempt}" -le 60 ]; do
  if temporal operator cluster health --address "${TEMPORAL_ADDRESS}" | grep -q SERVING; then
    break
  fi
  sleep 2
  attempt=$((attempt + 1))
done

OLD_IFS="${IFS}"
IFS=','
for namespace in ${TEMPORAL_NAMESPACES}; do
  namespace="$(printf '%s' "${namespace}" | xargs)"
  if [ -z "${namespace}" ]; then
    continue
  fi
  if temporal operator namespace describe --address "${TEMPORAL_ADDRESS}" -n "${namespace}" >/dev/null 2>&1; then
    continue
  fi
  temporal operator namespace create --address "${TEMPORAL_ADDRESS}" --retention "${TEMPORAL_NAMESPACE_RETENTION}" -n "${namespace}"
done
IFS="${OLD_IFS}"


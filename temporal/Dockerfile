ARG TEMPORAL_VERSION=1.30.1

FROM temporalio/admin-tools:${TEMPORAL_VERSION} AS admin-tools

FROM temporalio/server:${TEMPORAL_VERSION} AS base

COPY --from=admin-tools /usr/local/bin/temporal /usr/local/bin/temporal
COPY --from=admin-tools /usr/local/bin/temporal-sql-tool /usr/local/bin/temporal-sql-tool
COPY --from=admin-tools /etc/temporal/schema /etc/temporal/schema

COPY dynamicconfig /etc/temporal/config/dynamicconfig
COPY --chmod=755 scripts /opt/sango-temporal/scripts

FROM base AS dev
ENTRYPOINT ["/opt/sango-temporal/scripts/start-temporal-with-bootstrap.sh"]

FROM base AS prod
ENTRYPOINT ["/opt/sango-temporal/scripts/start-temporal-with-bootstrap.sh"]


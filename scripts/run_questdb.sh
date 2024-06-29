#!/bin/zsh

pulsar-daemon start standalone

pulsar-admin schemas upload test -f $PWD/pulsar/connectors/schema.avsc

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/postgres/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs $PULSAR_TOPIC_NAME \
    --name questdb-sink \
    --sink-config-file $PWD/pulsar/connectors/questdb/questdb-sink.yaml \
    --parallelism 1

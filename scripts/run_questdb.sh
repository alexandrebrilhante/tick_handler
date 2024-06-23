#!/bin/zsh

pulsar-daemon start standalone

pulsar-admin schemas upload test -f ./connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/postgres/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs test \
    --name questdb-sink \
    --sink-config-file $PWD/pulsar/connectors/questdb/questdb-sink.yaml \
    --parallelism 1

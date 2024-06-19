#!/bin/zsh

pulsar-daemon start standalone

pulsar-admin schemas upload postgres-sink-topic -f ./connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs test \
    --name postgres-sink \
    --sink-config-file $PWD/pulsar/connectors/postgres-sink.yaml \
    --parallelism 1
 
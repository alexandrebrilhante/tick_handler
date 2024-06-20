#!/bin/zsh

pulsar-daemon start standalone

pulsar-admin schemas upload test -f ./connectors/avro-schema

pulsar-admin sinks create \
    --tenant public \
    --namespace default \
    --name cassandra-sink \
    --archive $PWD/pulsar/connectors/pulsar-io-cassandra-3.2.2.nar \
    --sink-config-file $PWD/pulsar/connectors/cassandra-sink.yml \
    --inputs test

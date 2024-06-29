#!/bin/zsh

pulsar-daemon start standalone

pulsar-admin schemas upload test -f $PWD/pulsar/connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/cassandra/pulsar-io-cassandra-3.2.2.nar \
    --inputs $PULSAR_TOPIC_NAME \
    --name cassandra-sink \
    --sink-config-file $PWD/pulsar/connectors/cassandra/cassandra-sink.yml \
    --parallelism 1

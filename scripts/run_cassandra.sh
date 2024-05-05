#!/bin/zsh

pulsar-daemon start standalone

docker run -d --rm --name=cassandra -p 9042:9042 cassandra:latest

docker exec -ti cassandra cqlsh localhost

CREATE KEYSPACE pulsar_test_keyspace WITH replication = {'class':'SimpleStrategy', 'replication_factor':1};

USE pulsar_test_keyspace;

CREATE TABLE pulsar_test_table (key text PRIMARY KEY, col text);

pulsar-admin sinks create \
    --tenant public \
    --namespace default \
    --name cassandra-test-sink \
    --archive $PWD/pulsar/connectors/pulsar-io-cassandra-3.2.2.nar \
    --sink-config-file $PWD/pulsar/connectors/cassandra-sink.yml \
    --inputs test

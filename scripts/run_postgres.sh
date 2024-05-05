#!/bin/zsh

pulsar-daemon start standalone

docker pull postgres:12

docker run -d -it --rm \
    --name pulsar-postgres \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_USER=postgres \
    postgres:latest

docker exec -it pulsar-postgres /bin/bash

psql -U postgres postgres

create table if not exists pulsar_postgres_jdbc_sink
(
id serial PRIMARY KEY,
name VARCHAR(255) NOT NULL
);

pulsar-admin schemas upload pulsar-postgres-jdbc-sink-topic -f ./connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs test \
    --name pulsar-postgres-jdbc-sink \
    --sink-config-file $PWD/pulsar/connectors/pulsar-postgres-jdbc-sink.yaml \
    --parallelism 1

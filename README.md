# tick_handler

[![Build](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/build.yml/badge.svg)](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/build.yml)

This Rust application demonstrates how to set up an asynchronous Apache Pulsar producer that sends messages received from a TCP server and persists those same messages in Apache Cassandra, PostgreSQL or QuestDB using sink connectors.

This project was built as a proof-of-concept for a low-latency market data feed handler.

This software comes bundled with a Grafana dashboard to observe the message queue and measure latencies.

## Dependencies

Ensure your `Cargo.toml` file includes dependencies for the `pulsar` and `tokio` crates:

```toml
[dependencies]
pulsar = "6.1.0"
tokio = { version = "1.37.0", features = ["full"] }
```

## Usage

### Standalone

```bash
cargo build --release && cargo run --release
```

### Complete Setup

#### Cassandra

Ensure your Cassandra cluster is setup then start Apache Pulsar and the `tick_handler` executable:

```bash
CREATE KEYSPACE pulsar_keyspace WITH replication = {'class':'SimpleStrategy', 'replication_factor':1};

USE pulsar_keyspace;

CREATE TABLE pulsar_cassandra_sink (key text PRIMARY KEY, col text);
````

```bash
pulsar-daemon start standalone

pulsar-admin schemas upload $PULSAR_TOPIC_NAME -f $PWD/pulsar/connectors/schema.avsc

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/cassandra/pulsar-io-cassandra-3.2.2.nar \
    --inputs $PULSAR_TOPIC_NAME \
    --name cassandra-sink \
    --sink-config-file $PWD/pulsar/connectors/cassandra/cassandra-sink.yml \
    --parallelism 1
```

#### PostgreSQL

Ensure your PostgreSQL database is setup then start Apache Pulsar and the `tick_handler` executable:

```sql
CREATE TABLE IF NOT EXISTS pulsar_postgres_jdbc_sink (
    id serial PRIMARY KEY,
    name TEXT NOT NULL
);
```

```bash
pulsar-daemon start standalone

pulsar-admin schemas upload $PULSAR_TOPIC_NAME -f $PWD/pulsar/connectors/schema.avsc

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/postgres/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs $PULSAR_TOPIC_NAME \
    --name postgres-sink \
    --sink-config-file $PWD/pulsar/connectors/postgres/postgres-sink.yaml \
    --parallelism 1
```

#### QuestDB

Ensure your QuestDB database is setup then start Apache Pulsar and the `tick_handler` executable:

```sql
CREATE TABLE IF NOT EXISTS pulsar_questdb_sink (
    id serial PRIMARY KEY,
    name TEXT NOT NULL
);
```

```bash
pulsar-daemon start standalone

pulsar-admin schemas upload $PULSAR_TOPIC_NAME -f $PWD/pulsar/connectors/schema.avsc

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/postgres/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs $PULSAR_TOPIC_NAME \
    --name questdb-sink \
    --sink-config-file $PWD/pulsar/connectors/questdb/questdb-sink.yaml \
    --parallelism 1
```

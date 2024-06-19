# tick_handler

[![Rust](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/rust.yml/badge.svg)](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/rust.yml)

This Rust application demonstrates how to set up an asynchronous Apache Pulsar producer that sends messages received from a TCP server and persists those same messages in Apache Cassandra or PostgreSQL using a sink connector.

This project was built as a proof-of-concept for a low-latency market data feed handler.

This software comes bundled with a Grafana dashboard to observe the message queue and measure latencies.

## Dependencies

Ensure your `Cargo.toml` file includes dependencies for the `pulsar-rs` and `tokio` crates:

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

docker run -d --rm --name=cassandra -p 9042:9042 cassandra:latest

pulsar-admin sinks create \
    --tenant public \
    --namespace default \
    --name cassandra-sink \
    --archive $PWD/pulsar/connectors/pulsar-io-cassandra-3.2.2.nar \
    --sink-config-file $PWD/pulsar/connectors/cassandra-sink.yml \
    --inputs test
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

pulsar-admin schemas upload postgres-sink-topic -f ./connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs test \
    --name postgres-sink \
    --sink-config-file $PWD/pulsar/connectors/postgres-sink.yaml \
    --parallelism 1
```
# tick_handler

[![Rust](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/rust.yml/badge.svg)](https://github.com/alexandrebrilhante/tick_handler/actions/workflows/rust.yml)

This Rust application demonstrates how to set up an asynchronous Apache Pulsar producer that sends messages received from a TCP server and persists those same messages in Cassandra or PostgreSQL using a sink connector.

This project was built as a proof-of-concept for a low-latency market data feed handler.

This software comes bundled with a Grafana dashboard to observe the message queue and measure latencies.

## Dependencies

Ensure your `Cargo.toml` file includes dependencies for the `tokio` and `pulsar-rs` crates:

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

Ensure your Cassandra is setup:

```bash
CREATE KEYSPACE pulsar_test_keyspace WITH replication = {'class':'SimpleStrategy', 'replication_factor':1};

USE pulsar_test_keyspace;

CREATE TABLE pulsar_test_table (key text PRIMARY KEY, col text);
````

```bash
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

```

#### PostgreSQL

Ensure your PostgreSQL database is setup:

```sql
psql -U postgres postgres

CREATE TABLE IF NOT EXISTS pulsar_postgres_jdbc_sink (
    id serial PRIMARY KEY,
    name TEXT NOT NULL
);
```

```bash
pulsar-daemon start standalone

docker pull postgres:12

docker run -d -it --rm \
    --name pulsar-postgres \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_USER=postgres \
    postgres:latest

pulsar-admin schemas upload pulsar-postgres-jdbc-sink-topic -f ./connectors/avro-schema

pulsar-admin sinks create \
    --archive $PWD/pulsar/connectors/pulsar-io-jdbc-postgres-3.2.2.nar \
    --inputs test \
    --name pulsar-postgres-jdbc-sink \
    --sink-config-file $PWD/pulsar/connectors/pulsar-postgres-jdbc-sink.yaml \
    --parallelism 1
```
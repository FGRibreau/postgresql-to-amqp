PostgreSQL to AMQP [![Cargo version](https://img.shields.io/crates/v/postgresql-to-amqp.svg)](https://crates.io/crates/postgresql-to-amqp)
==================

PostgreSQL to AMQP, forward PostgreSQL notifications to an AMQP queue.

```
cargo install postgresql-to-amqp
```

## Configuration

Configuration is done through environment variables:

- **POSTGRESQL_URI**: e.g. `postgresql://username:password@domain.tld:port/database`
- **POSTGRESQL_CHANNEL**: e.g. `foo`
- **AMQP_HOST_PORT**: e.g. `127.0.0.1:5672`
- **AMQP_QUEUE_NAME**: e.g. `queueName` 

## Usage

Start the forwarder:

```bash
POSTGRESQL_URI="postgresql://username:password@domain.tld:port/database" POSTGRESQL_CHANNEL="foo" AMQP_HOST_PORT="127.0.0.1:5672" AMQP_QUEUE_NAME="queueName" postgresql-to-amqp
```


Execute in psql:

```sql
SELECT pg_notify('foo', 'payload');
```

The forwarder will log and forward the notification to the amqp queue:

```
Forwarding Notification { process_id: 31694, channel: "foo", payload: "payload" } to queue "queueName"
```


## Todo

- [ ] Refactor
- [ ] Docker support
- [ ] Kubernetes support
- [ ] Make a first release with tests

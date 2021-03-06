# :warning: Discontinued

### @subzerocloud took inspiration from this project and built [pg-amqp-bridge](https://github.com/subzerocloud/pg-amqp-bridge) which now has more options, use it instead :+1:

---------------------------------------------------------


# 🚇 PostgreSQL to AMQP gateway 
#### Forward PostgreSQL `pg_notify` notifications to an AMQP queue.

[![Cargo version](https://img.shields.io/crates/v/postgresql-to-amqp.svg)](https://crates.io/crates/postgresql-to-amqp) [![Crates.io](https://img.shields.io/crates/l/postgresql-to-amqp.svg)](https://crates.io/crates/postgresql-to-amqp) [![Crates.io](https://img.shields.io/crates/d/postgresql-to-amqp.svg)](https://crates.io/crates/postgresql-to-amqp) [![Docker Automated build](https://img.shields.io/docker/automated/fgribreau/postgresql-to-amqp.svg)](https://hub.docker.com/r/fgribreau/postgresql-to-amqp) [![Docker Pulls](https://img.shields.io/docker/pulls/fgribreau/postgresql-to-amqp.svg)](https://hub.docker.com/r/fgribreau/postgresql-to-amqp) [![Docker Stars](https://img.shields.io/docker/stars/fgribreau/postgresql-to-amqp.svg)](https://hub.docker.com/r/fgribreau/postgresql-to-amqp) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)
==================

<p align="center"><img src="https://cloud.githubusercontent.com/assets/138050/24724213/9c512220-1a4a-11e7-8a3e-9b8ad0945f51.gif"/></p>



## ⛴ Cargo

```shell
cargo install postgresql-to-amqp
```

## 🐳 Docker 
```shell
docker run --rm -it \
-e POSTGRESQL_URI=postgresql://username:password@domain.tld:port/database \
-e POSTGRESQL_CHANNEL=foo \
-e AMQP_URI=amqp://127.0.0.1:5672/ \
-e AMQP_QUEUE_NAME=queueName fgribreau/postgresql-to-amqp
```

## ⚙ Configuration

Configuration is done through environment variables:

- **POSTGRESQL_URI**: e.g. `postgresql://username:password@domain.tld:port/database`
- **POSTGRESQL_CHANNEL**: e.g. `foo`
- **AMQP_URI**: e.g. `amqp://127.0.0.1:5672/`
- **AMQP_QUEUE_NAME**: e.g. `queueName`

## 🎩 Usage

Start the forwarder:

```bash
POSTGRESQL_URI="postgresql://username:password@domain.tld:port/database" POSTGRESQL_CHANNEL="foo" AMQP_URI="amqp://127.0.0.1:5672/" AMQP_QUEUE_NAME="queueName" postgresql-to-amqp
```


Execute in psql:

```sql
SELECT pg_notify('foo', 'payload');
```

The forwarder will log and forward the notification to the amqp queue:

```
Forwarding Notification { process_id: 31694, channel: "foo", payload: "payload" } to queue "queueName"
```

## 👁 Philosophy

- Low memory consumption (1,9Mo)
- Single binary
- No dependency
- Predictable performance


## 🔫 Todo

I will happily accept PRs for this:

- [ ] AMQP connection string (AMQP authentication support) 👻
- [ ] Support JSON message
- [ ] Publish to exchange
- [ ] Add original channel as message property
- [ ] Add postgresql-to-amqp `version` as message property
- [ ] Let environment variables specify additional message properties
- [ ] Handle AMQP disconnection/reconnection
- [ ] Handle PostgreSQL disconnection/reconnection
- [ ] Health check route
- [ ] Metric route
- [x] Docker support
- [ ] Kubernetes support 😍
- [ ] Make a first major release with tests ☝️

## Related work

- [pgsql-listen-exchange](https://github.com/gmr/pgsql-listen-exchange) - RabbitMQ Exchange that publishes messages received from PostgreSQL Notifications

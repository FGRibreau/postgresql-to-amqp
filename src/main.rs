#[macro_use]
extern crate log;
extern crate env_logger;

extern crate amq_protocol;
extern crate futures;
extern crate tokio_core;
extern crate lapin_futures as lapin;
extern crate postgres;
extern crate fallible_iterator;

mod app_config;

use amq_protocol::types::FieldTable;
use futures::future::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use lapin::client::ConnectionOptions;
use lapin::channel::{BasicPublishOptions, BasicProperties, QueueDeclareOptions};

use postgres::{Connection, TlsMode};
use fallible_iterator::FallibleIterator;

fn main() {
    env_logger::init().unwrap();

    // load configuration
    let config = app_config::AppConfig::new();

    // create the reactor
    let mut core = Core::new().unwrap();
    let handle = core.handle();


    let amqp_addr = config.amqp_host_port.parse().expect("amqp_host_port should be in format '127.0.0.1:5672'");
    let pg_conn = Connection::connect(config.postgresql_uri.clone(), TlsMode::None).expect("Could not connect to PostgreSQL");

    core.run(
        TcpStream::connect(&amqp_addr, &handle).and_then(|stream| {
            // connect() returns a future of an AMQP Client
            // that resolves once the handshake is done
            lapin::client::Client::connect(
                stream,
                &ConnectionOptions::default()
            )
        })
            .and_then(|client| client.create_channel())
            .and_then(|channel| {
                let id = channel.id;
                info!("created channel with id: {}", id);

                channel.queue_declare(config.clone().amqp_queue_name.as_str(), &QueueDeclareOptions::default(), FieldTable::new()).and_then(move |_| {
                    info!("channel {} declared queue {}", id, config.amqp_queue_name.as_str());

                    // https://www.postgresql.org/docs/7.4/static/sql-listen.html
                    let listen_command = format!("LISTEN {}", config.postgresql_channel.as_str());
                    pg_conn.execute(listen_command.as_str(), &[]).expect("Could not send LISTEN");

                    let notifications = pg_conn.notifications();

                    // https://sfackler.github.io/rust-postgres/doc/v0.11.11/postgres/notification/struct.BlockingIter.html
                    let mut it = notifications.blocking_iter();

                    println!("Waiting for notifications...");

                    // could not use 'loop' here because it does not compile in --release mode
                    // since Ok() is unreachable.
                    #[allow(while_true)]
                    while true {

                        // it.next() -> Result<Option<Notification>>
                        match it.next() {
                            Ok(Some(notification)) => {
                                // https://github.com/sfackler/rust-postgres/blob/master/postgres-shared/src/lib.rs
                                println!("Forwarding {:?} to queue {:?}", notification, config.amqp_queue_name.as_str());
                                channel.basic_publish(
                                    config.amqp_queue_name.as_str(),
                                    // @todo we might want to send it as JSON (configurable)
                                    // https://doc.rust-lang.org/1.12.0/std/fmt/
                                    format!("{}! {}!", notification.channel, notification.payload).as_bytes(),
                                    &BasicPublishOptions::default(),
                                    // @todo make this configurable through environment variables
                                    BasicProperties::default().with_user_id("guest".to_string()).with_reply_to("foobar".to_string())
                                );
                            },
                            Err(err) => println!("Got err {:?}", err),
                            _ => panic!("Unexpected state.")
                        }
                    }


                    Ok(channel)
                })
            }).map_err(|err| {
            println!("Could not connect to AMQP: {}", err);
            err
        })
    ).expect("Could not run reactor");
}

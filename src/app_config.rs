use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub postgresql_uri: String,
    pub postgresql_channel: String,
    pub amqp_host_port: String,
    pub amqp_queue_name: String
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            postgresql_uri: env::var("POSTGRESQL_URI").expect("POSTGRESQL_URI environment variable must be defined"),
            postgresql_channel: env::var("POSTGRESQL_CHANNEL").expect("POSTGRESQL_CHANNEL environment variable must be defined"),
            amqp_host_port: env::var("AMQP_HOST_PORT").expect("AMQP_HOST_PORT environment variable must be defined"),
            amqp_queue_name: env::var("AMQP_QUEUE_NAME").expect("AMQP_QUEUE_NAME environment variable must be defined"),
        }
    }
}

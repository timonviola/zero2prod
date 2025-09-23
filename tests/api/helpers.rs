use once_cell::sync::Lazy;
use std::io::{sink, stdout};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;


use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::email_client::EmailClient;
use zero2prod::startup::{Application, get_connection_pool};
use zero2prod::telemetry::{get_subscriber, init_subscriber};


static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "debug".into(), stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), "debug".into(), sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        // Use the mock server as email API
        //c.email_client.base_url = email_server.uri();
        c
    };
    configure_database(&configuration.database).await;
    let application = Application::build(configuration.clone())
        .await
        .expect("failed");
    let address = format!("http://127.0.0.1:{}", application.port());
    let _ = tokio::spawn(application.run_until_stopped());
//    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");
//    let port = listener.local_addr().unwrap().port();
//    let address = format!("http://127.0.0.1:{}", port);
//
//    let mut configuration = get_configuration().expect("Failed to read configuration.");
//    configuration.database.database_name = Uuid::new_v4().to_string();
//    let connection_pool = configure_database(&configuration.database).await;
//    let sender_email = configuration
//        .email_client
//        .sender()
//        .expect("Invalid sender email address.");
//    let timeout = configuration.email_client.timeout();
//    let email_client = EmailClient::new(
//        configuration.email_client.base_url,
//        sender_email,
//        configuration.email_client.authorization_token,
//        timeout,
//    );
//
//    let server =
//        run(listener, connection_pool.clone(), email_client).expect("Failed to bind address.");
//    let _ = tokio::spawn(server);
//
    TestApp {
        address,
        db_pool: get_connection_pool(&configuration.database),
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to cennect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}


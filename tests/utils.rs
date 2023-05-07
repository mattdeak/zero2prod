use std::net::TcpListener;

use sea_orm::sea_query::TableCreateStatement;
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use sea_orm::DbBackend;
use sea_orm::Schema;
use tokio;
use tracing::Level;
use tracing_subscriber;
use zero2prod::entity::subscriptions::Entity;
use zero2prod::make_router;
use zero2prod::types::PostgresUrl;

async fn setup_db(main_db_url: &PostgresUrl) -> Result<PostgresUrl, Box<dyn std::error::Error>> {
    let random_db_name = format!("test_{}", uuid::Uuid::new_v4().to_string());

    // replace hyphens with underscores
    let random_db_name = random_db_name.replace("-", "_");

    let db = Database::connect(main_db_url.to_string()).await?;

    let stmt = sea_orm::Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE {}", random_db_name),
    );

    db.execute(stmt).await?;

    let db_url = PostgresUrl {
        host: main_db_url.host.clone(),
        port: main_db_url.port,
        username: main_db_url.username.clone(),
        password: main_db_url.password.clone(),
        database: random_db_name.clone(),
    };

    let db = Database::connect(db_url.to_string()).await?;

    let schema = Schema::new(DbBackend::Postgres);
    let stmt: TableCreateStatement = schema.create_table_from_entity(Entity);

    let result = db.execute(db.get_database_backend().build(&stmt)).await;

    match result {
        Ok(_) => Ok(db_url),
        Err(e) => Err(e.into()),
    }
}

fn get_primary_test_url() -> PostgresUrl {
    PostgresUrl {
        host: "127.0.0.1".to_string(),
        port: 5433,
        username: "postgres".to_string(),
        password: "test-pass".to_string(),
        database: "test".to_string(),
    }
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener
        .local_addr()
        .expect("Unable to get local addr")
        .port();

    println!("Spawning app on port {}", port);

    let primary_url = get_primary_test_url();

    let _ = tokio::spawn(async move {
        let connection_url = setup_db(&primary_url).await.unwrap_or_else(|e| {
            panic!("Failed to connect to create test db.\n{:?}", e);
        });

        let router = make_router(&connection_url).await.unwrap_or_else(|e| {
            panic!("Failed to connect to make router.\n{:?}", e);
        });

        axum::Server::from_tcp(listener)
            .expect("Unable to bind server to listener")
            .serve(router.into_make_service())
            .await
            .expect("Server Error");
    });

    format!("http://127.0.0.1:{}", port)
}

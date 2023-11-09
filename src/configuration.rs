use config::ConfigError;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_no_name(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub async fn connect_to_db() -> Pool<Postgres> {
    let config = get_configuration().expect("Failed to read configuration");

    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database.connection_string())
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub async fn connect_to_random_db() -> Pool<Postgres> {
    println!("Spinning up random db");

    let mut config = get_configuration().expect("Failed to read configuration");

    let database_name = format!("newsletter-{}", Uuid::new_v4());
    let connection_string = format!(
        "{}/{}",
        &config.database.connection_string_no_name(),
        database_name
    );

    config.database.database_name = database_name.clone();

    let connection = PgPoolOptions::new()
        .connect(&config.database.connection_string_no_name())
        .await
        .expect("Failed initial connect to db");

    let create_db = format!(r#"CREATE DATABASE "{}";"#, database_name);
    sqlx::query(&create_db)
        .execute(&connection)
        .await
        .expect("Failed to create test db");

    println!("Connection url: {}", connection_string);

    connection.close().await;

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&connection_string)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate the new database");

    println!("Completed migrations");

    pool
}

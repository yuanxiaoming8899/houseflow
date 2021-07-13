use actix_web::{web, HttpServer};
use houseflow_config::server::Config;
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{Sessions, SledTokenStore, TokenStore};
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;

const LOG_ENV: &str = "HOUSEFLOW_LOG";

#[actix_web::main]
async fn main() {
    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .expect(&format!("invalid `{}` environment variable", LOG_ENV))
        })
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt().with_max_level(level).init();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load server config");
    let config = web::Data::new(config);
    let token_store = SledTokenStore::new(&config.tokens_path).expect("cannot open token store");
    let token_store = web::Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);

    let database = SqliteDatabase::new(&config.database_path).expect("cannot open database");
    let database = web::Data::from(Arc::new(database) as Arc<dyn Database>);
    let sessions = web::Data::new(Sessions::default());
    let config_cloned = config.clone();
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(|cfg| {
                houseflow_server::configure(
                    cfg,
                    token_store.clone(),
                    database.clone(),
                    config_cloned.clone(),
                    sessions.clone(),
                )
            })
    });

    let server = if let Some(tls) = &config.tls {
        log::info!("Starting server with TLS");
        let mut rustls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());

        let certificate = &mut std::io::BufReader::new(
            std::fs::File::open(&tls.certificate_path).expect("read certificate fail"),
        );
        let private_key = &mut std::io::BufReader::new(
            std::fs::File::open(&tls.private_key_path).expect("read private key fail"),
        );
        let certificate_chain = rustls::internal::pemfile::certs(certificate).unwrap();
        let keys = rustls::internal::pemfile::pkcs8_private_keys(private_key).unwrap();
        rustls_config
            .set_single_cert(certificate_chain, keys.into_iter().next().unwrap())
            .unwrap();
        server.bind_rustls(config.address, rustls_config)
    } else {
        log::info!("Starting server without TLS");
        server.bind(config.address)
    };
    server
        .expect("bind server fail")
        .run()
        .await
        .expect("run server fail");
}

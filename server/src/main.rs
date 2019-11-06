use actix_files::Files;
use actix_threadpool::BlockingError;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web_middleware_redirect_https::RedirectHTTPS;
use futures::Future;
use log::error;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite;
use rustls::{
    internal::pemfile::{certs, rsa_private_keys},
    NoClientAuth, ServerConfig,
};
use serde::{self, Serialize};
use std::{env, fs, io};

#[derive(Serialize)]
pub struct VoteSuggestion {
    pub postcode: String,
    pub constituency: String,
    pub party: String,
    pub majority: i32,
    #[serde(rename = "majorityPercent")]
    pub majority_percent: f64,
}

const MAX_ACCEPTED_LEN: usize = 16;

fn api(
    path: web::Path<String>,
    db: web::Data<Pool<SqliteConnectionManager>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        // Don't process further if the path is too long
        if path.len() > MAX_ACCEPTED_LEN {
            return Err(rusqlite::Error::InvalidPath("".to_owned().into()));
        }

        let conn = db.get().unwrap();
        let mut stmt = conn.prepare_cached("\
        SELECT postcode, constituencies.name, parties.name, majority, majority_percent \
        FROM postcodes \
            LEFT JOIN constituencies ON postcodes.constituency_id=constituencies.id \
            LEFT JOIN vote_suggestions ON postcodes.constituency_id=vote_suggestions.constituency_id \
            LEFT JOIN parties ON vote_suggestions.party_id=parties.id \
        WHERE postcode=$1 \
            ")?;
        let postcode = path.replace(" ", "").to_uppercase();
        stmt.query_row(&[&postcode], |row| {
            Ok(VoteSuggestion {
                postcode: row.get(0)?,
                constituency: row.get(1)?,
                party: row.get(2)?,
                majority: row.get(3)?,
                majority_percent: row.get(4)?,
            })
        })
    })
    .then(|res| match res {
        Ok(suggestion) => Ok(HttpResponse::Ok().json(suggestion)),
        Err(BlockingError::Error(rusqlite::Error::QueryReturnedNoRows)) => Ok(HttpResponse::NotFound().into()),
        Err(BlockingError::Error(rusqlite::Error::InvalidPath(_))) => Ok(HttpResponse::BadRequest().into()),
        Err(e) => {
            error!("{}", e);
            Ok(HttpResponse::InternalServerError().into())
        },
    })
}

fn main() -> io::Result<()> {
    // Logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    // Environment
    let db_path = env::var("DB_PATH").expect("DB_PATH not set");
    let static_path = env::var("STATIC_PATH").expect("STATIC_PATH not set");
    let hostname = env::var("HOSTNAME").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let port = env::var("PORT").expect("PORT not set");
    let ssl_port = env::var("SSL_PORT").expect("SSL_PORT not set");
    let cert_file = env::var("CERT_FILE").expect("CERT_FILE not set");
    let key_file = env::var("KEY_FILE").expect("KEY_FILE not set");

    // System runner
    let sys = actix_rt::System::new("voteabc");

    // DB Connection Pool Manager
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).unwrap();

    // SSL
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut io::BufReader::new(fs::File::open(cert_file)?);
    let key_file = &mut io::BufReader::new(fs::File::open(key_file)?);
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // Redirect
    let redirect_middleware = RedirectHTTPS::with_replacements(&[(port.clone(), ssl_port.clone())]);

    // HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(redirect_middleware.clone())
            .service(
                web::scope("/api")
                    .data(pool.clone())
                    .route("/{postcode}", web::get().to_async(api)),
            )
            .wrap(middleware::Compress::default())
            // For ACME requests - Actix otherwise rejects URIs starting with '.'
            // Must be above the other files to ensure it matches first
            .service(Files::new(
                "/.well-known/",
                format!("{}/.well-known", static_path.clone()),
            ))
            .service(Files::new("/", static_path.clone()).index_file("index.html"))
    })
    .bind(format!("{}:{}", hostname, port))?
    .bind_rustls(format!("{}:{}", hostname, ssl_port), config)?
    .start();

    sys.run()
}

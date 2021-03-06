extern crate bb8;
extern crate bb8_postgres;
extern crate chrono;
extern crate config as config_crate;
extern crate either;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate futures_cpupool;
extern crate futures_state_stream;
extern crate geo;
extern crate hyper;
#[macro_use]
extern crate log as log_crate;
extern crate postgres;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate stq_acl;
extern crate stq_api;
extern crate stq_db;
#[macro_use]
extern crate stq_http;
extern crate stq_logging;
extern crate stq_roles;
extern crate stq_router;
extern crate stq_static_resources;
extern crate stq_types;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_postgres;
extern crate tokio_signal;
extern crate uuid;
extern crate validator;
#[macro_use]
extern crate sentry;
extern crate csv;
extern crate rusoto_core;
extern crate rusoto_s3;

use std::net::SocketAddr;
use std::process::exit;

use bb8_postgres::PostgresConnectionManager;
use futures::future;
use futures::prelude::*;
use hyper::server::Http;
use tokio_core::reactor::{Core, Remote};
use tokio_postgres::TlsMode;

use stq_http::controller::Application;

pub mod acl;
pub mod config;
pub mod controller;
pub mod errors;
pub mod loaders;
pub mod models;
pub mod repos;
pub mod sentry_integration;
pub mod services;
pub mod types;

pub use config::*;
use errors::*;
use types::*;

pub fn prepare_db(remote: Remote) -> Box<Future<Item = bb8::Pool<PostgresConnectionManager>, Error = tokio_postgres::Error>> {
    let config = config::Config::new().unwrap();
    let manager = PostgresConnectionManager::new(config.db.dsn.clone(), || TlsMode::None).unwrap();

    bb8::Pool::builder().min_idle(Some(10)).build(manager, remote)
}

/// Starts web server with the provided configuration
pub fn start_server<F: FnOnce() + 'static>(config: config::Config, port: Option<u16>, callback: F) {
    let mut core = Core::new().expect("Unexpected error creating event loop core");

    let manager = PostgresConnectionManager::new(config.db.dsn.clone(), || TlsMode::None).unwrap();
    let db_pool = {
        let remote = core.remote();
        DbPool::from(
            core.run(
                bb8::Pool::builder()
                    .min_idle(Some(10))
                    .build(manager, remote)
                    .map_err(|e| format_err!("{}", e)),
            )
            .expect("Failed to create connection pool"),
        )
    };

    let listen_address = {
        let port = port.unwrap_or(config.listen.port);
        SocketAddr::new(config.listen.host, port)
    };

    let serve = Http::new()
        .serve_addr_handle(&listen_address, &core.handle(), move || {
            let controller = controller::ControllerImpl::new(&db_pool, &config);

            // Prepare application
            let app = Application::<Error>::new(controller);

            Ok(app)
        })
        .unwrap_or_else(|why| {
            error!("Http Server Initialization Error: {}", why);
            exit(1);
        });

    let handle = core.handle();
    handle.spawn(
        serve
            .for_each({
                let handle = handle.clone();
                move |conn| {
                    handle.spawn(conn.map(|_| ()).map_err(|why| error!("Server Error: {:?}", why)));
                    Ok(())
                }
            })
            .map_err(|_| ()),
    );

    info!("Listening on http://{}", listen_address);
    handle.spawn_fn(move || {
        callback();
        future::ok(())
    });

    core.run(tokio_signal::ctrl_c().flatten_stream().take(1u64).for_each(|()| {
        info!("Ctrl+C received. Exit");
        Ok(())
    }))
    .unwrap();
}

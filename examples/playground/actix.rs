// use std::sync::mpsc;
// use std::thread;

use actix_rt;
use actix_web::{web, HttpResponse, HttpServer};
use clap;
// use futures::future::Future;

//------------------------------------------------------------------------------------------------//

mod api {
    use actix_web::{web, HttpResponse, Responder};
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct User {
        name: String,
    }

    fn sth(user: web::Path<User>) -> impl Responder {
        HttpResponse::Ok().body(format!("Welcome {}", user.name))
    }

    fn foo() -> impl Responder {
        HttpResponse::Ok().body("foo")
    }

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.route("/foo", web::get().to(foo)).service(
            web::resource("/{name}")
                .route(web::get().to(sth))
                .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
        );
    }
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn run_server() {
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {
    let addr = "127.0.0.1:8080";
    let sys = actix_rt::System::new("http-server");

    let _server = HttpServer::new(|| {
        actix_web::App::new()
            .configure(config)
            .service(web::scope("/api").configure(api::config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind(addr)
    .unwrap()
    .shutdown_timeout(60)
    .start();

    println!(
        "{}",
        &[
            "Server is available.".to_owned(),
            format!("Try {}", addr),
            format!("Try {}/app", addr),
            format!("Try {}/api", addr),
            format!("Try {}/api/YOUR_NAME", addr),
        ]
        .join("\n")
    );

    // tx.send(server).unwrap();
    sys.run().unwrap();
    // });

    // let server = rx.recv().unwrap();
    // server
    //     .pause()
    //     .wait()
    //     .map(|_| println!("actix_server::ServerCommand::Pause"))
    //     .unwrap();
    // server
    //     .resume()
    //     .wait()
    //     .map(|_| println!("actix_server::ServerCommand::Resume"))
    //     .unwrap();
    // server
    //     .stop(true)
    //     .wait()
    //     .map(|_| println!("actix_server::ServerCommand::Stop"))
    //     .unwrap();
}

//------------------------------------------------------------------------------------------------//

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,osmgraphing=info,parser=info,dijkstra=info'",
                "for getting 'warn's per default, but 'info' about the others (e.g. 'parser').",
                "RUST_LOG is set up automatically, setting RUST_LOG to 'info'",
                "for relevant parts of the software, but consider the flag '--quiet'.",
                "",
                "In case you're using cargo, please use",
                "    cargo run --example",
                "for all supported example files",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(
            clap::Arg::with_name("quiet").short("q").long("quiet").help(
                &[
                    "Logs `info` in addition to `warn` and `error`.",
                    "The env-variable `RUST_LOG` has precedence.",
                ]
                .join("\n"),
            ),
        )
        .get_matches()
}

fn setup_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: doesn't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some("actix"), log::LevelFilter::Info);
        builder.filter(Some("osmgraphing"), log::LevelFilter::Info);
    }
    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }
    // init
    builder.init();
}

fn main() {
    let matches = parse_cmdline();
    setup_logging(matches.is_present("quiet"));
    run_server();
}

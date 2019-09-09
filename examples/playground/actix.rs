use actix_web::{web, HttpResponse, HttpServer};
use clap;

//------------------------------------------------------------------------------------------------//

mod api {
    use actix_web::{web, HttpResponse, Responder};

    fn sth() -> impl Responder {
        HttpResponse::Ok().body("sth")
    }

    fn foo() -> impl Responder {
        HttpResponse::Ok().body("foo")
    }

    pub fn config(cfg: &mut web::ServiceConfig) {
        cfg.route("/foo", web::get().to(foo)).service(
            web::resource("/{sth}")
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
    HttpServer::new(|| {
        actix_web::App::new()
            .configure(config)
            .service(web::scope("/api").configure(api::config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
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
                "For all supported example files, please use",
                "    `cargo run --example`",
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,osmgraphing=info,parser=info,dijkstra=info'",
                "",
                "for getting `warn`s per default, but `info` about the others (e.g. `parser`).",
            ]
            .join("\n"))
                .as_ref(),
        )
        .get_matches()
}

fn main() {
    env_logger::Builder::from_env("RUST_LOG").init();
    let _matches = parse_cmdline();
    run_server();
}

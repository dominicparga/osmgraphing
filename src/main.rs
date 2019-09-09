use actix_web::{web, HttpResponse, HttpServer, Responder};
use clap;

//------------------------------------------------------------------------------------------------//

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn run_server() {
    HttpServer::new(|| {
        actix_web::App::new()
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .service(
                web::scope("/blub")
                    .route("", web::to(index))
                    .route("/", web::to(index))
                    .route("/again", web::to(index2))
                    .route("/hello", web::to(index3)),
            )
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

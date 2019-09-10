use actix_files;
use actix_web::guard;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, HttpRequest, HttpResponse, HttpServer, Result};

//------------------------------------------------------------------------------------------------//

fn _index(_req: HttpRequest) -> Result<HttpResponse> {
    // println!("{:?}", _req);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("client/index.html")))
}

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

fn code404() -> Result<actix_files::NamedFile> {
    let f = actix_files::NamedFile::open("src/ui/client/code404.html");
    if f.is_err() {
        log::error!("code404-file not found");
    }
    Ok(f?.set_status_code(StatusCode::NOT_FOUND))
}

pub fn run() {
    let domain = std::env::var("DOMAIN").unwrap_or("localhost:8080".to_owned());

    HttpServer::new(|| {
        actix_web::App::new()
            .wrap(middleware::Logger::default()) // always last
            // https://stackoverflow.com/questions/57500023/served-javascript-file-is-blank
            .service(web::scope("/api").configure(api::config))
            .service(actix_files::Files::new("", "./src/ui/").index_file("client/index.html"))
            // default service
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(code404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(domain)
    .unwrap()
    .run()
    .unwrap();
}

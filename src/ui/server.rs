use actix_web::{web, HttpResponse, HttpServer};

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

fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("client/index.html"))
}

pub fn run() {
    HttpServer::new(|| {
        actix_web::App::new()
            // .route("", web::get().to(index))
            .route("/", web::get().to(index))
            .service(web::scope("/api").configure(api::config))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .unwrap();
}

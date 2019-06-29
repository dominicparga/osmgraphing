use futures::{future, Future, Stream};
use hyper::header::{HeaderName, HeaderValue};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server, Chunk};
use hyper::{Method, StatusCode};
use lazy_static::lazy_static;
use maplit::btreemap;
use std::collections::BTreeMap;
use std::path::Path;

lazy_static! {
    static ref MIME_BY_EXTENSION: BTreeMap<String, String> = {
        let owned_version = btreemap![
            "css" => "text/css",
            "js" => "text/javascript",
        ];

        owned_version
            .iter()
            .map(|(key, val)| (String::from(*key), String::from(*val)))
            .collect()
    };
}

// Just a simple type alias
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn serve_static_files(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => {
            // first, we serve static files
            let fs_path = match path {
                "" | "/" => String::from("index.html"),
                _ => format!(".{}", path),
            };

            //illicit request
            if fs_path.contains("../") {
                *response.status_mut() = StatusCode::NOT_FOUND;
                return Box::new(future::ok(response));
            }

            // Set content type here...
            let path_creator = fs_path.clone();
            let as_path = Path::new(&path_creator);
            if as_path.is_file() {
                let text = vec![std::fs::read(fs_path).unwrap()];

                if let Some(extension) = as_path.extension() {
                    if let Some(non_html_mime) = MIME_BY_EXTENSION.get(extension.to_str().unwrap())
                    {
                        (*response.headers_mut()).insert(
                            HeaderName::from_static("content-type"),
                            HeaderValue::from_static(non_html_mime),
                        );
                    };
                } else {
                    eprintln!("Content type unset for {:?}", as_path);
                }

                *response.body_mut() =
                    Body::wrap_stream(futures::stream::iter_ok::<_, ::std::io::Error>(text));
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}

pub fn start_server() {
    //socket address
    let addr = ([127, 0, 0, 1], 8080).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(serve_static_files))
        .map_err(|e| eprintln!("server error: {}", e));

    println!(
        "{}",
        format!("🚀 Listening at http://{}", addr)
    );
    hyper::rt::run(server)
}
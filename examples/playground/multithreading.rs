use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use clap;
use log::info;

//------------------------------------------------------------------------------------------------//

fn run_first_threads() {
    info!("Run: First threads");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn run_channels() {
    info!("Run: channels");
    let (tx0, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx0);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx0.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("Waiting for receiver ..");
    for received in rx {
        println!("Got: {}", received);
    }
}

fn run_mutex() {
    info!("Run: mutex");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
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
    run_first_threads();
    run_channels();
    run_mutex();
}

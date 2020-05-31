use std::process::Command;

fn run_example(example: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&format!("cargo run --example {}", example))
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        panic!("{}", std::str::from_utf8(&output.stderr).unwrap());
    }
}

#[test]
fn dijkstra() {
    run_example("dijkstra");
}

#[test]
fn parser() {
    run_example("parser");
}

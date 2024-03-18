use minigrep::{run, visit_dir, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {err}");
        process::exit(1);
    });

    let all_path = visit_dir(config.file_path.clone()).unwrap_or_else(|err| {
        eprintln!("Problem with searchin in directory: {err}");
        process::exit(1);
    });

    for path in all_path {
        config.file_path = path.to_string_lossy().into_owned();

        if let Err(e) = run(&config) {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }

}

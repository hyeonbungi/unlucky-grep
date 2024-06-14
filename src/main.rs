use std::env;
use std::process;

use unlucky_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("[오류] {err}");
        process::exit(1);
    });

    if let Err(err) = unlucky_grep::run(config) {
        eprintln!("[오류] {err}");
        process::exit(1);
    }
}

use std::{
    fs::File,
    io::{stderr, stdout},
};

use slog::{o, Logger};
use slog_scope::{error, info, scope, set_global_logger};
use step_3_8::drain;

fn access(method: &str, path: &str) {
    info!("http"; "path" => path, "method" => method);
}

fn main() {
    let _global = set_global_logger(Logger::root(
        drain(stderr(), stdout()),
        o!("file" => "app.log"),
    ));

    let access_log = File::options()
        .append(true)
        .create(true)
        .open("access.log")
        .expect("cannot open file access.log");

    scope(
        &Logger::root(
            drain(access_log.try_clone().unwrap(), access_log),
            slog::o!("file" => "access.log"),
        ),
        || {
            access("POST", "/path");
        },
    );

    error!("Error occurred");
}

use lsr::{get_arg_paths, get_dir_files, log_err, log_strings};

fn main() {
    let paths = get_arg_paths().unwrap_or_else(|err| {
        log_err(err);
        std::process::exit(2);
    });
    for path in paths {
        let strings = get_dir_files(path).unwrap_or_else(|err| {
            log_err(err);
            std::process::exit(1);
        });
        log_strings(strings);
    }
}

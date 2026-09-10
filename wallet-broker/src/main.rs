#![forbid(unsafe_code)]

mod runtime;

fn main() {
    std::process::exit(runtime::run());
}

fn main() {
    if let Err(e) = ch07::get_args().and_then(ch07::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

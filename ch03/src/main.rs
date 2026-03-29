fn main() {
    if let Err(e) = ch03::get_args().and_then(ch03::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

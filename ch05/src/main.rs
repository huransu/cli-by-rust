fn main() {
    if let Err(e) = ch05::get_args().and_then(ch05::run) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

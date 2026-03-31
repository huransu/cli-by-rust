fn main() {
    if let Err(e) = ch04::get_args().and_then(ch04::run) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

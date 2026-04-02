fn main() {
    if let Err(e) = ch06::get_args().and_then(ch06::run) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

//! Window process. SDL stays on this thread; [`galactic_armadas::app`] starts
//! the sim thread beside it.

fn main() {
    if let Err(err) = galactic_armadas::app::run() {
        eprintln!("galactic-armadas: {err}");
        std::process::exit(1);
    }
}

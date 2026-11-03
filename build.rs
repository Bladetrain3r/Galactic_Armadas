// Static SDL2 does not tell rustc which system libraries its video backend
// needs. On Linux those are the X11 client libraries the bundled build
// detected while compiling SDL. Desktop machines already have the shared
// objects; the headers are only required at build time (see the README).
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "linux" {
        return;
    }
    for lib in [
        "dl", "m", "pthread", "X11", "Xext", "Xcursor", "Xinerama", "Xi", "Xrandr", "Xss",
        "Xxf86vm", "Xfixes", "Xrender",
    ] {
        println!("cargo:rustc-link-lib={lib}");
    }
}

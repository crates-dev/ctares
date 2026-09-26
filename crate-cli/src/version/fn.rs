/// Print version
pub fn print_version() {
    log::info!("cc {}", env!("CARGO_PKG_VERSION"));
}

/// Print version
pub fn print_version() {
    log::info!("crate {}", env!("CARGO_PKG_VERSION"));
}

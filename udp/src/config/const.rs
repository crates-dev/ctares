/// Default host address.
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// Default UDP port.
pub const DEFAULT_PORT: u16 = 60000;

/// Default buffer size for UDP packets (512KB).
pub const DEFAULT_BUFFER_SIZE: usize = 524288;

/// Default `TCP_NODELAY` setting.
pub(crate) const DEFAULT_NODELAY: Option<bool> = None;

/// Default `IP_TTL` setting.
pub(crate) const DEFAULT_TTL: Option<u32> = None;

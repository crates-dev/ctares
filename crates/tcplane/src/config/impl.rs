use super::*;

/// Provides a default implementation for ServerConfigData.
impl Default for ServerConfigData {
    /// Creates a new ServerConfigData instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_PORT,
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }
}

/// Provides a default implementation for ServerConfig.
impl Default for ServerConfig {
    /// Creates a new ServerConfig instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance wrapping default ServerConfigData.
    #[inline(always)]
    fn default() -> Self {
        Self(Arc::new(RwLock::new(ServerConfigData::default())))
    }
}

impl ServerConfig {
    /// Creates a new ServerConfig instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new ServerConfig instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the inner configuration data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockReadGuard<ServerConfigData>` - The read guard.
    pub(crate) async fn read(&self) -> ArcRwLockReadGuard<'_, ServerConfigData> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner configuration data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockWriteGuard<ServerConfigData>` - The write guard.
    pub(crate) async fn write(&self) -> ArcRwLockWriteGuard<'_, ServerConfigData> {
        self.0.write().await
    }

    /// Gets a clone of the inner configuration data.
    ///
    /// # Returns
    ///
    /// - `ServerConfigData` - A clone of the configuration data.
    pub(crate) async fn get_data(&self) -> ServerConfigData {
        self.read().await.clone()
    }

    /// Gets the host address.
    ///
    /// # Returns
    ///
    /// - `String` - The host address.
    pub async fn get_host(&self) -> String {
        self.read().await.host.clone()
    }

    /// Gets the port number.
    ///
    /// # Returns
    ///
    /// - `u16` - The port number.
    pub async fn get_port(&self) -> u16 {
        self.read().await.port
    }

    /// Gets the buffer size.
    ///
    /// # Returns
    ///
    /// - `usize` - The buffer size in bytes.
    pub async fn get_buffer_size(&self) -> usize {
        self.read().await.buffer_size
    }

    /// Sets the host address.
    ///
    /// # Arguments
    ///
    /// - `Into<String>` - Type that can be converted into String.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn host<H>(&self, host: H) -> &Self
    where
        H: Into<String>,
    {
        self.write().await.host = host.into();
        self
    }

    /// Sets the port number.
    ///
    /// # Arguments
    ///
    /// - `u16` - The port number.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn port(&self, port: u16) -> &Self {
        self.write().await.port = port;
        self
    }

    /// Sets the buffer size.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn buffer_size(&self, buffer_size: usize) -> &Self {
        self.write().await.buffer_size = buffer_size;
        self
    }
}

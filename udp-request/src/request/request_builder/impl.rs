use super::*;

/// Implements the `Default` trait for `RequestBuilder`.
impl Default for RequestBuilder {
    /// Creates a default `RequestBuilder` instance.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestBuilder` with default values.
    #[inline(always)]
    fn default() -> Self {
        Self {
            udp_request: UdpRequest::default(),
            builder: UdpRequest::default(),
        }
    }
}

/// Implementation of `RequestBuilder`.
impl RequestBuilder {
    /// Creates a new `RequestBuilder`.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `RequestBuilder` instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the host for the UDP request.
    ///
    /// # Arguments
    ///
    /// - `T` - The host address, which can be any type that implements `Into<String>`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `RequestBuilder` for method chaining.
    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        let _: Result<(), PoisonError<RwLockWriteGuard<'_, Config>>> =
            self.udp_request.config.write().map(|mut data| {
                data.host = host.into();
            });
        self
    }

    /// Sets the port for the UDP request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The port number.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `RequestBuilder` for method chaining.
    pub fn port(&mut self, port: usize) -> &mut Self {
        let _: Result<(), PoisonError<RwLockWriteGuard<'_, Config>>> =
            self.udp_request.config.write().map(|mut data| {
                data.port = port;
            });
        self
    }

    /// Sets the buffer size for the UDP request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `RequestBuilder` for method chaining.
    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        let _: Result<(), PoisonError<RwLockWriteGuard<'_, Config>>> =
            self.udp_request.config.write().map(|mut data| {
                data.buffer_size = buffer_size;
            });
        self
    }

    /// Sets the timeout for the UDP request.
    ///
    /// # Arguments
    ///
    /// - `u64` - The timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to the `RequestBuilder` for method chaining.
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        let _: Result<(), PoisonError<RwLockWriteGuard<'_, Config>>> =
            self.udp_request.config.write().map(|mut data| {
                data.timeout = timeout;
            });
        self
    }

    /// Builds the `UdpRequest` and returns a boxed trait object.
    ///
    /// # Returns
    ///
    /// - `BoxRequestTrait` - A boxed `RequestTrait` object that can be used to send the request.
    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.udp_request.clone();
        self.udp_request = UdpRequest::default();
        Box::new(self.builder.clone())
    }
}

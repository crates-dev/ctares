use super::*;

/// Default implementation for RequestBuilder.
impl Default for RequestBuilder {
    /// Creates a default RequestBuilder instance.
    ///
    /// # Returns
    ///
    /// - `RequestBuilder` - A new RequestBuilder with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            tcp_request: TcpRequest::default(),
            builder: TcpRequest::default(),
        }
    }
}

/// Implementation for RequestBuilder methods.
impl RequestBuilder {
    /// Creates a new RequestBuilder instance.
    ///
    /// # Returns
    ///
    /// - `RequestBuilder` - A new RequestBuilder with default configuration.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target host for the request.
    ///
    /// # Arguments
    ///
    /// - `T` - The host address (any type that implements Into<String>).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining.
    pub fn host<T>(&mut self, host: T) -> &mut Self
    where
        T: Into<String>,
    {
        let _ = self.tcp_request.config.write().map(|mut data| {
            data.host = host.into();
        });
        self
    }

    /// Sets the target port for the request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The port number.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining.
    pub fn port(&mut self, port: usize) -> &mut Self {
        let _ = self.tcp_request.config.write().map(|mut data| {
            data.port = port;
        });
        self
    }

    /// Sets the buffer size for the request.
    ///
    /// # Arguments
    ///
    /// - `usize` - The buffer size in bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining.
    pub fn buffer(&mut self, buffer_size: usize) -> &mut Self {
        let _ = self.tcp_request.config.write().map(|mut data| {
            data.buffer_size = buffer_size;
        });
        self
    }

    /// Sets the timeout for the request in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - The timeout duration in milliseconds.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining.
    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        let _ = self.tcp_request.config.write().map(|mut data| {
            data.timeout = timeout;
        });
        self
    }

    /// Builds and returns the configured request.
    ///
    /// # Returns
    ///
    /// - `BoxRequestTrait` - A boxed request trait object ready for use.
    pub fn build(&mut self) -> BoxRequestTrait {
        self.builder = self.tcp_request.clone();
        self.tcp_request = TcpRequest::default();
        Box::new(self.builder.clone())
    }
}

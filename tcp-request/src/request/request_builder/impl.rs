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
    /// Gets a reference to the TCP request being configured.
    ///
    /// # Returns
    ///
    /// - `&TcpRequest` - Reference to the TCP request.
    pub(crate) fn get_tcp_request(&self) -> &TcpRequest {
        &self.tcp_request
    }

    /// Gets a reference to the built TCP request.
    ///
    /// # Returns
    ///
    /// - `&TcpRequest` - Reference to the built TCP request.
    pub(crate) fn get_builder(&self) -> &TcpRequest {
        &self.builder
    }

    /// Sets the TCP request being configured.
    ///
    /// # Arguments
    ///
    /// - `TcpRequest` - The TCP request to configure.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub(crate) fn set_tcp_request(&mut self, tcp_request: TcpRequest) -> &mut Self {
        self.tcp_request = tcp_request;
        self
    }

    /// Sets the built TCP request.
    ///
    /// # Arguments
    ///
    /// - `TcpRequest` - The built TCP request.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub(crate) fn set_builder(&mut self, builder: TcpRequest) -> &mut Self {
        self.builder = builder;
        self
    }

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
        let _ = self.get_tcp_request().get_config().write().map(|mut data| {
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
        let _ = self.get_tcp_request().get_config().write().map(|mut data| {
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
        let _ = self.get_tcp_request().get_config().write().map(|mut data| {
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
        let _ = self.get_tcp_request().get_config().write().map(|mut data| {
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
        let tcp_request: TcpRequest = self.get_tcp_request().clone();
        self.set_builder(tcp_request);
        self.set_tcp_request(TcpRequest::default());
        Box::new(self.get_builder().clone())
    }
}

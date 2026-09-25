use super::*;

/// Provides a default implementation for ContextData.
impl Default for ContextData {
    /// Creates a new ContextData instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            aborted: false,
            closed: false,
            stream: None,
            request: Request::new(),
            response: Response::default(),
            attributes: HashMap::new(),
        }
    }
}

impl ContextData {
    /// Creates a new ContextData instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Provides a default implementation for Context.
impl Default for Context {
    /// Creates a new Context instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Context wrapping default ContextData.
    #[inline(always)]
    fn default() -> Self {
        Self(Arc::new(RwLock::new(ContextData::default())))
    }
}

/// Implementation of `From<ContextData>` for `Context`.
impl From<ContextData> for Context {
    /// Converts a `ContextData` into a `Context`.
    ///
    /// # Arguments
    ///
    /// - `ContextData` - The context data to wrap.
    ///
    /// # Returns
    ///
    /// - `Context` - A new Context instance.
    #[inline(always)]
    fn from(data: ContextData) -> Self {
        Self(Arc::new(RwLock::new(data)))
    }
}

/// Implementation of `From<ArcRwLockStream>` for `Context`.
impl From<ArcRwLockStream> for Context {
    /// Converts an `ArcRwLockStream` into a `Context`.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream to set in the context.
    ///
    /// # Returns
    ///
    /// - `Context` - A new Context instance with the stream set.
    #[inline(always)]
    fn from(stream: ArcRwLockStream) -> Self {
        let data: ContextData = ContextData {
            stream: Some(stream),
            ..Default::default()
        };
        Self::from(data)
    }
}

/// Implementation of methods for the Context structure.
impl Context {
    /// Creates a new Context with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Context instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockReadGuard<ContextData>` - The read guard.
    pub(crate) async fn read(&self) -> ArcRwLockReadGuard<'_, ContextData> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner context data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockWriteGuard<ContextData>` - The write guard.
    pub(crate) async fn write(&self) -> ArcRwLockWriteGuard<'_, ContextData> {
        self.0.write().await
    }

    /// Checks if the context has been marked as aborted.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the context is aborted, otherwise false.
    pub async fn is_aborted(&self) -> bool {
        self.read().await.aborted
    }

    /// Sets the aborted flag for the context.
    ///
    /// # Arguments
    ///
    /// - `bool` - The aborted state to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_aborted(&self, aborted: bool) -> &Self {
        self.write().await.aborted = aborted;
        self
    }

    /// Marks the context as aborted.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn abort(&self) -> &Self {
        self.set_aborted(true).await
    }

    /// Cancels the aborted state of the context.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn cancel_abort(&self) -> &Self {
        self.set_aborted(false).await
    }

    /// Checks if the connection has been closed.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is closed, otherwise false.
    pub async fn is_closed(&self) -> bool {
        self.read().await.closed
    }

    /// Sets the closed flag for the connection.
    ///
    /// # Arguments
    ///
    /// - `bool` - The closed state to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_closed(&self, closed: bool) -> &Self {
        self.write().await.closed = closed;
        self
    }

    /// Marks the connection as closed.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn close(&self) -> &Self {
        self.set_closed(true).await
    }

    /// Opens the connection (clears the closed flag).
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to the modified context.
    pub async fn open(&self) -> &Self {
        self.set_closed(false).await
    }

    /// Checks if the connection has been terminated (aborted or closed).
    ///
    /// # Returns
    ///
    /// - `bool` - True if the connection is either aborted or closed, otherwise false.
    pub async fn is_terminated(&self) -> bool {
        self.is_aborted().await || self.is_closed().await
    }

    /// Gets the stream from the context.
    ///
    /// # Returns
    ///
    /// - `Option<ArcRwLockStream>` - The stream if available.
    pub async fn try_get_stream(&self) -> Option<ArcRwLockStream> {
        self.read().await.stream.clone()
    }

    /// Gets the stream from the context.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockStream` - The stream.
    ///
    /// # Panics
    ///
    /// Panics if the stream is not set.
    pub async fn get_stream(&self) -> ArcRwLockStream {
        self.try_get_stream().await.unwrap()
    }

    /// Sets the stream in the context.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_stream(&self, stream: ArcRwLockStream) -> &Self {
        self.write().await.stream = Some(stream);
        self
    }

    /// Gets the request from the context.
    ///
    /// # Returns
    ///
    /// - `Request` - A clone of the request.
    pub async fn get_request(&self) -> Request {
        self.read().await.request.clone()
    }

    /// Sets the request in the context.
    ///
    /// # Arguments
    ///
    /// - `Request` - The request to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_request(&self, request: Request) -> &Self {
        self.write().await.request = request;
        self
    }

    /// Gets the response from the context.
    ///
    /// # Returns
    ///
    /// - `Response` - A clone of the response.
    pub async fn get_response(&self) -> Response {
        self.read().await.response.clone()
    }

    /// Sets the response in the context.
    ///
    /// # Arguments
    ///
    /// - `Response` - The response to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_response(&self, response: Response) -> &Self {
        self.write().await.response = response;
        self
    }

    /// Attempts to get the socket address from the stream.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - The socket address if available.
    pub async fn try_get_socket_addr(&self) -> OptionSocketAddr {
        if let Some(stream) = self.try_get_stream().await {
            return stream.try_get_peer_addr().await;
        }
        None
    }

    /// Gets the socket address.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The socket address.
    ///
    /// # Panics
    ///
    /// Panics if the socket address is not available.
    pub async fn get_socket_addr(&self) -> SocketAddr {
        self.try_get_socket_addr().await.unwrap()
    }

    /// Gets the socket address as a string.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The socket address as a string if available.
    pub async fn try_get_socket_addr_string(&self) -> Option<String> {
        self.try_get_socket_addr()
            .await
            .map(|addr| addr.to_string())
    }

    /// Gets the socket address as a string.
    ///
    /// # Returns
    ///
    /// - `String` - The socket address as a string.
    ///
    /// # Panics
    ///
    /// Panics if the socket address is not available.
    pub async fn get_socket_addr_string(&self) -> String {
        self.get_socket_addr().await.to_string()
    }

    /// Attempts to get the socket host (IP address).
    ///
    /// # Returns
    ///
    /// - `OptionSocketHost` - The socket host if available.
    pub async fn try_get_socket_host(&self) -> OptionSocketHost {
        self.try_get_socket_addr().await.map(|addr| addr.ip())
    }

    /// Gets the socket host.
    ///
    /// # Returns
    ///
    /// - `std::net::IpAddr` - The socket host.
    ///
    /// # Panics
    ///
    /// Panics if the socket host is not available.
    pub async fn get_socket_host(&self) -> std::net::IpAddr {
        self.try_get_socket_host().await.unwrap()
    }

    /// Attempts to get the socket port.
    ///
    /// # Returns
    ///
    /// - `OptionSocketPort` - The socket port if available.
    pub async fn try_get_socket_port(&self) -> OptionSocketPort {
        self.try_get_socket_addr().await.map(|addr| addr.port())
    }

    /// Gets the socket port.
    ///
    /// # Returns
    ///
    /// - `u16` - The socket port.
    ///
    /// # Panics
    ///
    /// Panics if the socket port is not available.
    pub async fn get_socket_port(&self) -> u16 {
        self.try_get_socket_port().await.unwrap()
    }

    /// Sets a data value in the context's data map.
    ///
    /// # Arguments
    ///
    /// - `Into<String>` - The key for the data.
    /// - `Any + Send + Sync + Clone` - The value to set, which must be cloneable and thread-safe.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn set_data<K, V>(&self, key: K, value: V) -> &Self
    where
        K: Into<String>,
        V: Any + Send + Sync + Clone,
    {
        self.write()
            .await
            .attributes
            .insert(key.into(), Arc::new(value));
        self
    }

    /// Gets a data value from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key for the data.
    ///
    /// # Returns
    ///
    /// - `Option<V>` - The data value if found and successfully downcasted, otherwise `None`.
    pub async fn try_get_data<V, K>(&self, key: K) -> Option<V>
    where
        V: Any + Send + Sync + Clone,
        K: AsRef<str>,
    {
        self.read()
            .await
            .attributes
            .get(key.as_ref())
            .and_then(|arc| arc.downcast_ref::<V>())
            .cloned()
    }

    /// Gets a data value from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key for the data.
    ///
    /// # Returns
    ///
    /// - `V` - The data value.
    ///
    /// # Panics
    ///
    /// Panics if the data is not found or cannot be downcasted.
    pub async fn get_data_value<V, K>(&self, key: K) -> V
    where
        V: Any + Send + Sync + Clone,
        K: AsRef<str>,
    {
        self.try_get_data(key).await.unwrap()
    }

    /// Removes a data value from the context's data map.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The key of the data to remove.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn remove_data<K>(&self, key: K) -> &Self
    where
        K: AsRef<str>,
    {
        self.write().await.attributes.remove(key.as_ref());
        self
    }

    /// Clears all data from the context's data map.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn clear_data(&self) -> &Self {
        self.write().await.attributes.clear();
        self
    }

    /// Attempts to send data through the stream.
    ///
    /// # Arguments
    ///
    /// - `D` - Data that can be converted to a byte slice.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_send<D>(&self, data: D) -> ResponseResult
    where
        D: AsRef<[u8]>,
    {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream().await {
            return stream.try_send(data).await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Sends data through the stream.
    ///
    /// # Arguments
    ///
    /// - `D` - Data that can be converted to a byte slice.
    ///
    /// # Panics
    ///
    /// Panics if the send operation fails.
    pub async fn send<D>(&self, data: D)
    where
        D: AsRef<[u8]>,
    {
        self.try_send(data).await.unwrap();
    }

    /// Attempts to flush the stream.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_flush(&self) -> ResponseResult {
        if self.is_terminated().await {
            return Err(ResponseError::Terminated);
        }
        if let Some(stream) = self.try_get_stream().await {
            return stream.try_flush().await;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Flushes the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&self) {
        self.try_flush().await.unwrap();
    }

    /// Attempts to shut down the stream.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_shutdown(&self) -> ResponseResult {
        if let Some(stream) = self.try_get_stream().await {
            let result: ResponseResult = stream.try_shutdown().await;
            self.close().await;
            return result;
        }
        Err(ResponseError::NotFoundStream)
    }

    /// Shuts down the stream.
    ///
    /// # Panics
    ///
    /// Panics if the shutdown operation fails.
    pub async fn shutdown(&self) {
        self.try_shutdown().await.unwrap();
    }
}

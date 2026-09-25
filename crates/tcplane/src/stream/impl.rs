use super::*;

impl ArcRwLockStream {
    /// Creates a new `ArcRwLockStream` from an existing `Arc<RwLock<TcpStream>>`.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLock<TcpStream>` - The Arc-wrapped RwLock containing the TCP stream.
    ///
    /// # Returns
    ///
    /// - `Self` - A new ArcRwLockStream instance.
    pub fn from(arc_rw_lock_stream: ArcRwLock<TcpStream>) -> Self {
        Self(arc_rw_lock_stream)
    }

    /// Creates a new `ArcRwLockStream` from a raw `TcpStream`.
    ///
    /// # Arguments
    ///
    /// - `TcpStream` - The TCP stream to wrap.
    ///
    /// # Returns
    ///
    /// - `Self` - A new ArcRwLockStream instance.
    pub fn from_stream(stream: TcpStream) -> Self {
        Self(Arc::new(RwLock::new(stream)))
    }

    /// Acquires a read lock on the inner TCP stream.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockReadGuard<'_, TcpStream>` - A read guard for the TCP stream.
    pub async fn read(&self) -> ArcRwLockReadGuard<'_, TcpStream> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner TCP stream.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockWriteGuard<'_, TcpStream>` - A write guard for the TCP stream.
    pub async fn write(&self) -> ArcRwLockWriteGuard<'_, TcpStream> {
        self.0.write().await
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
        let mut stream: ArcRwLockWriteGuard<'_, TcpStream> = self.write().await;
        stream
            .write_all(data.as_ref())
            .await
            .map_err(|error: std::io::Error| ResponseError::WriteError(error.to_string()))?;
        Ok(())
    }

    /// Sends data through the stream.
    ///
    /// # Arguments
    ///
    /// - `D` - Data that can be converted to a byte slice.
    ///
    /// # Panics
    ///
    /// Panics if the write operation fails.
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
        let mut stream: ArcRwLockWriteGuard<'_, TcpStream> = self.write().await;
        stream
            .flush()
            .await
            .map_err(|error: std::io::Error| ResponseError::FlushError(error.to_string()))?;
        Ok(())
    }

    /// Flushes the stream.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&self) {
        self.try_flush().await.unwrap();
    }

    /// Attempts to get the peer address.
    ///
    /// # Returns
    ///
    /// - `OptionSocketAddr` - The peer address if available.
    pub async fn try_get_peer_addr(&self) -> OptionSocketAddr {
        let stream: ArcRwLockReadGuard<'_, TcpStream> = self.read().await;
        stream.peer_addr().ok()
    }

    /// Gets the peer address.
    ///
    /// # Returns
    ///
    /// - `SocketAddr` - The peer address.
    ///
    /// # Panics
    ///
    /// Panics if the peer address is not available.
    pub async fn get_peer_addr(&self) -> SocketAddr {
        self.try_get_peer_addr().await.unwrap()
    }

    /// Attempts to shut down the stream.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_shutdown(&self) -> ResponseResult {
        let mut stream: ArcRwLockWriteGuard<'_, TcpStream> = self.write().await;
        stream
            .shutdown()
            .await
            .map_err(|error: std::io::Error| ResponseError::WriteError(error.to_string()))?;
        Ok(())
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

/// Implementation of `From<TcpStream>` for `ArcRwLockStream`.
impl From<TcpStream> for ArcRwLockStream {
    /// Converts a `TcpStream` into an `ArcRwLockStream`.
    ///
    /// # Arguments
    ///
    /// - `TcpStream` - The TCP stream to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new ArcRwLockStream instance.
    fn from(stream: TcpStream) -> Self {
        Self::from_stream(stream)
    }
}

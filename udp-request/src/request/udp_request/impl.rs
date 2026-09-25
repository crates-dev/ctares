use super::*;

/// Implements the `Default` trait for `UdpRequest`.
impl Default for UdpRequest {
    /// Creates a default `UdpRequest` instance.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `UdpRequest` with default configuration and an empty response.
    #[inline(always)]
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            response: Arc::new(RwLock::new(UdpResponseBinary::default())),
        }
    }
}

/// Implementation of `UdpRequest`.
impl UdpRequest {
    /// Sends a UDP request and reads the response.
    ///
    /// # Arguments
    ///
    /// - `&mut UdpSocket` - The UDP socket to use for sending and receiving data.
    /// - `&[u8]` - The data to send in the request.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` - A `Result` containing the response as a boxed trait object or a `RequestError`.
    fn send_request(
        &mut self,
        socket: &mut UdpSocket,
        data: &[u8],
    ) -> Result<BoxResponseTrait, RequestError> {
        socket
            .send(data)
            .map_err(|err| RequestError::SendResponseError(err.to_string()))?;
        self.read_response(socket)
    }

    /// Reads the response from the UDP socket.
    ///
    /// # Arguments
    ///
    /// - `&mut UdpSocket` - The UDP socket to read the response from.
    ///
    /// # Returns
    ///
    /// - `Result<BoxResponseTrait, RequestError>` - A `Result` containing the response as a boxed trait object or a `RequestError`.
    fn read_response(&mut self, socket: &mut UdpSocket) -> Result<BoxResponseTrait, RequestError> {
        let cfg_buffer_size: usize = self
            .config
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |data| data.buffer_size);
        let mut tmp_buf: Vec<u8> = vec![0u8; cfg_buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(cfg_buffer_size);
        if let Ok(n) = socket.recv(&mut tmp_buf) {
            response_bytes.extend_from_slice(&tmp_buf[..n]);
        }
        self.response = Arc::new(RwLock::new(<UdpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        Ok(Box::new(
            self.response.read().map_or(Vec::new(), |data| data.clone()),
        ))
    }

    /// Creates and configures a UDP socket for the connection.
    ///
    /// # Arguments
    ///
    /// - `String` - The host address to connect to.
    /// - `usize` - The port number to connect to.
    ///
    /// # Returns
    ///
    /// - `Result<UdpSocket, RequestError>` - A `Result` containing the configured `UdpSocket` or a `RequestError`.
    fn get_connection_socket(&self, host: String, port: usize) -> Result<UdpSocket, RequestError> {
        let host_port: String = format!("{}:{}", host.clone(), port);
        let cfg_timeout: u64 = self
            .config
            .read()
            .map_or(DEFAULT_TIMEOUT, |data: RwLockReadGuard<'_, Config>| {
                data.timeout
            });
        let timeout: Duration = Duration::from_millis(cfg_timeout);
        let socket: UdpSocket =
            UdpSocket::bind("0.0.0.0:0").map_err(|_| RequestError::UdpSocketCreateError)?;
        socket
            .connect(host_port)
            .map_err(|_| RequestError::UdpSocketConnectError)?;
        socket
            .set_read_timeout(Some(timeout))
            .map_err(|_| RequestError::SetReadTimeoutError)?;
        socket
            .set_write_timeout(Some(timeout))
            .map_err(|_| RequestError::SetWriteTimeoutError)?;
        let socket_result: Result<UdpSocket, RequestError> = Ok(socket);
        socket_result
    }
}

/// Implements the `RequestTrait` for `UdpRequest`.
impl RequestTrait for UdpRequest {
    type RequestResult = RequestResult;

    /// Sends a UDP request with the given data.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The data to send in the request.
    ///
    /// # Returns
    ///
    /// - `Self::RequestResult` - The result of the request, containing either the response or an error.
    fn send(&mut self, data: &[u8]) -> Self::RequestResult {
        let cfg_timeout: Config = self
            .config
            .read()
            .map_or(Config::default(), |data| data.clone());
        let host: String = cfg_timeout.host.clone();
        let port: usize = cfg_timeout.port;
        let mut socket: UdpSocket = self.get_connection_socket(host, port)?;
        let res: Result<BoxResponseTrait, RequestError> = self.send_request(&mut socket, data);
        res
    }
}

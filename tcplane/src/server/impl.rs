use super::*;

/// Provides a default implementation for ServerData.
impl Default for ServerData {
    /// Creates a new ServerData instance with empty collections.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default values.
    fn default() -> Self {
        Self {
            server_config: ServerConfigData::default(),
            hook: vec![],
            task_panic: vec![],
            read_error: vec![],
        }
    }
}

/// Provides a default implementation for ServerControlHook.
impl Default for ServerControlHook {
    /// Creates no-op wait and shutdown hooks.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with inert control hooks.
    fn default() -> Self {
        Self {
            wait_hook: Arc::new(|| Box::pin(async {})),
            shutdown_hook: Arc::new(|| Box::pin(async {})),
        }
    }
}

/// Provides a default implementation for Server.
impl Default for Server {
    /// Creates a new Server instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    fn default() -> Self {
        Self(Arc::new(RwLock::new(ServerData::default())))
    }
}

/// Implementation of methods for the Server structure.
impl Server {
    /// Creates a new Server instance with default settings.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Server instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockReadGuard<'_, ServerData>` - The read guard.
    pub async fn read(&self) -> ArcRwLockReadGuard<'_, ServerData> {
        self.0.read().await
    }

    /// Acquires a write lock on the inner server data.
    ///
    /// # Returns
    ///
    /// - `ArcRwLockWriteGuard<'_, ServerData>` - The write guard.
    pub(crate) async fn write(&self) -> ArcRwLockWriteGuard<'_, ServerData> {
        self.0.write().await
    }

    /// Sets the server configuration.
    ///
    /// # Arguments
    ///
    /// - `ServerConfig` - The server configuration.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn server_config(&self, config: ServerConfig) -> &Self {
        *self.write().await.get_mut_server_config() = config.get_data().await;
        self
    }

    /// Constructs a bind address string from host and port。
    ///
    /// # Arguments
    ///
    /// - `H` - Type that can be referenced as a string slice.
    /// - `u16` - The port number.
    ///
    /// # Returns
    ///
    /// - `String` - The formatted bind address.
    #[inline(always)]
    pub fn get_bind_addr<H>(host: H, port: u16) -> String
    where
        H: AsRef<str>,
    {
        format!("{}{}{}", host.as_ref(), COLON, port)
    }

    /// Adds a typed hook to the server's hook list.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn hook<H>(&self) -> &Self
    where
        H: ServerHook,
    {
        self.write()
            .await
            .get_mut_hook()
            .push(server_hook_factory::<H>());
        self
    }

    /// Adds a panic handler to the server's task panic handler list.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn task_panic<H>(&self) -> &Self
    where
        H: ServerHook,
    {
        self.write()
            .await
            .get_mut_task_panic()
            .push(server_hook_factory::<H>());
        self
    }

    /// Adds an error handler to the server's error handler list.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    pub async fn read_error<H>(&self) -> &Self
    where
        H: ServerHook,
    {
        self.write()
            .await
            .get_mut_read_error()
            .push(server_hook_factory::<H>());
        self
    }

    /// Creates a TCP listener bound to the configured address。
    ///
    /// # Returns
    ///
    /// - `Result<TcpListener, ServerError>` - The listener on success, or an error on failure.
    async fn create_tcp_listener(&self) -> Result<TcpListener, ServerError> {
        let config: ServerConfigData = self.read().await.get_server_config().clone();
        let host: String = config.host;
        let port: u16 = config.port;
        let addr: String = Self::get_bind_addr(&host, port);
        TcpListener::bind(&addr)
            .await
            .map_err(|error: io::Error| ServerError::TcpBind(error.to_string()))
    }

    /// Spawns a new task to handle an incoming connection.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream for the incoming connection.
    async fn spawn_connection_handler(&self, stream: ArcRwLockStream) {
        let server: Server = self.clone();
        let hook: ServerHookList = self.read().await.get_hook().clone();
        let task_panic: ServerHookList = self.read().await.get_task_panic().clone();
        let buffer_size: usize = self.read().await.get_server_config().buffer_size;
        spawn(async move {
            server
                .handle_connection(stream, hook, task_panic, buffer_size)
                .await;
        });
    }

    /// Handles an incoming connection by processing it through the hook chain.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream for the connection.
    /// - `ServerHookList` - The list of hooks to process.
    /// - `ServerHookList` - The list of panic handlers.
    /// - `usize` - The buffer size for reading data.
    async fn handle_connection(
        &self,
        stream: ArcRwLockStream,
        hook: ServerHookList,
        task_panic: ServerHookList,
        buffer_size: usize,
    ) {
        let request: Request = match self.read_stream(&stream, buffer_size).await {
            Ok(data) => data,
            Err(e) => {
                self.read_error_handle(e.to_string()).await;
                return;
            }
        };
        let ctx: Context = self.create_context(stream, request).await;

        for h in hook.iter() {
            let ctx_clone: Context = ctx.clone();
            let h_clone: ServerHookHandler = Arc::clone(h);
            let join_handle: JoinHandle<()> = spawn(async move {
                h_clone(ctx_clone).await;
            });

            match join_handle.await {
                Ok(()) => {}
                Err(e) if e.is_panic() => {
                    for panic_handler in task_panic.iter() {
                        panic_handler(ctx.clone()).await;
                    }
                    break;
                }
                Err(_) => break,
            }
        }
    }

    /// Reads data from the stream into a request.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to read from.
    /// - `usize` - The buffer size for reading.
    ///
    /// # Returns
    ///
    /// - `Result<Request, ServerError>` - The request data on success, or an error on failure.
    async fn read_stream(
        &self,
        stream: &ArcRwLockStream,
        buffer_size: usize,
    ) -> Result<Request, ServerError> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut tmp_buf: Vec<u8> = vec![0u8; buffer_size];
        let mut stream_guard: ArcRwLockWriteGuard<'_, TcpStream> = stream.write().await;
        loop {
            match stream_guard.read(&mut tmp_buf).await {
                Ok(0) => break,
                Ok(n) => {
                    buffer.extend_from_slice(&tmp_buf[..n]);
                    if tmp_buf[..n].ends_with(SPLIT_REQUEST_BYTES) {
                        let end_pos: usize = buffer.len().saturating_sub(SPLIT_REQUEST_BYTES.len());
                        buffer.truncate(end_pos);
                        break;
                    }
                    if n < tmp_buf.len() {
                        break;
                    }
                }
                Err(e) => {
                    return Err(ServerError::TcpRead(e.to_string()));
                }
            }
        }
        Ok(buffer)
    }

    /// Creates a context for processing a request.
    ///
    /// # Arguments
    ///
    /// - `ArcRwLockStream` - The stream for the connection.
    /// - `Request` - The request data.
    ///
    /// # Returns
    ///
    /// - `Context` - The created context.
    async fn create_context(&self, stream: ArcRwLockStream, request: Request) -> Context {
        let mut data: ContextData = ContextData::new();
        data.stream = Some(stream);
        data.request = request;
        Context::from(data)
    }

    /// Handles an read error by invoking the configured error handlers.
    ///
    /// # Arguments
    ///
    /// - `String` - The error message.
    async fn read_error_handle(&self, error: String) {
        let error_handlers: ServerHookList = self.read().await.get_read_error().clone();
        let ctx: Context = Context::new();
        ctx.set_data(CONTEXT_ERROR_KEY, error).await;
        for handler in error_handlers.iter() {
            handler(ctx.clone()).await;
        }
    }

    /// Starts the server and begins accepting connections.
    ///
    /// # Returns
    ///
    /// - `Result<ServerControlHook, ServerError>` - The control hook on success, or an error on failure.
    pub async fn run(&self) -> Result<ServerControlHook, ServerError> {
        let tcp_listener: TcpListener = self.create_tcp_listener().await?;
        let server: Server = self.clone();
        let (wait_sender, wait_receiver) = channel(());
        let (shutdown_sender, mut shutdown_receiver) = channel(());
        let accept_connections: JoinHandle<()> = spawn(async move {
            loop {
                tokio::select! {
                    result = tcp_listener.accept() => {
                        match result {
                            Ok((stream, _)) => {
                                let stream: ArcRwLockStream = ArcRwLockStream::from_stream(stream);
                                server.spawn_connection_handler(stream).await;
                            }
                            Err(_) => break,
                        }
                    }
                    _ = shutdown_receiver.changed() => {
                        break;
                    }
                }
            }
            let _: Result<(), tokio::sync::watch::error::SendError<()>> = wait_sender.send(());
        });
        let wait_hook: ServerControlHookFn = Arc::new(move || {
            let mut wait_receiver_clone: Receiver<()> = wait_receiver.clone();
            Box::pin(async move {
                let _: Result<(), tokio::sync::watch::error::RecvError> =
                    wait_receiver_clone.changed().await;
            }) as ServerControlFuture
        });
        let shutdown_hook: ServerControlHookFn = Arc::new(move || {
            let shutdown_sender_clone: Sender<()> = shutdown_sender.clone();
            Box::pin(async move {
                let _: Result<(), tokio::sync::watch::error::SendError<()>> =
                    shutdown_sender_clone.send(());
            }) as ServerControlFuture
        });
        spawn(async move {
            let _: Result<(), JoinError> = accept_connections.await;
        });
        Ok(ServerControlHook {
            wait_hook,
            shutdown_hook,
        })
    }
}

/// Implementation of methods for the ServerControlHook structure.
impl ServerControlHook {
    /// Waits for the server to finish.
    pub async fn wait(&self) {
        (self.get_wait_hook())().await;
    }

    /// Initiates a graceful shutdown of the server.
    pub async fn shutdown(&self) {
        (self.get_shutdown_hook())().await;
    }
}

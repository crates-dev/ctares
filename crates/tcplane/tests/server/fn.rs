use super::*;

#[tokio::test]
async fn test_server_basic_usage() {
    let server_config: ServerConfig = ServerConfig::new();
    server_config.host("0.0.0.0").await;
    server_config.port(60000).await;
    server_config.buffer_size(100_024_000).await;
    let server: Server = Server::new();
    server.task_panic::<PanicHandler>().await;
    server.hook::<GreetingHandler>().await;
    server.hook::<EchoHandler>().await;
    server.hook::<DefaultHook>().await;
    server.read_error::<ErrorHandler>().await;
    let server_control_hook_1: ServerControlHook = server.run().await.unwrap_or_default();
    let server_control_hook_2: ServerControlHook = server_control_hook_1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        server_control_hook_2.shutdown().await;
    });
    server_control_hook_1.wait().await;
}

#[tokio::test]
async fn test_server_handler_trait() {
    #[derive(Clone, Copy, Debug)]
    struct CustomHandler;
    impl ServerHook for CustomHandler {
        async fn new(_: &Context) -> Self {
            Self
        }

        async fn handle(self, ctx: &Context) {
            let _: () = ctx.send("CustomHandler executed").await;
        }
    }
    let server: Server = Server::new();
    server.hook::<CustomHandler>().await;
    let hook_count: usize = server.read().await.get_hook().len();
    assert_eq!(hook_count, 1);
}

#[tokio::test]
async fn test_server_task_panic() {
    #[derive(Clone, Copy, Debug)]
    struct TestPanicHandler;
    impl ServerHook for TestPanicHandler {
        async fn new(_: &Context) -> Self {
            Self
        }

        async fn handle(self, _: &Context) {}
    }
    let server: Server = Server::new();
    server.task_panic::<TestPanicHandler>().await;
    let panic_count: usize = server.read().await.get_task_panic().len();
    assert_eq!(panic_count, 1);
}

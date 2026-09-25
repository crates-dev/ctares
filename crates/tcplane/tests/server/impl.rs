use super::*;

impl ServerHook for GreetingHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let _: () = ctx.send("Hello from GreetingHandler!").await;
    }
}

impl ServerHook for EchoHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: Request = ctx.get_request().await;
        let response: String = format!("Echo: {request:?}");
        let _: () = ctx.send(response).await;
    }
}

impl ServerHook for PanicHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let _: () = ctx.send("Panic occurred!").await;
    }
}

impl ServerHook for ErrorHandler {
    async fn new(_: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        if let Some(error) = ctx.try_get_data::<String, _>("error").await {
            eprintln!("{error}");
            let _: Result<(), Error> = Write::flush(&mut stderr());
        }
    }
}

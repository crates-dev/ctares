use super::*;

impl ServerHook for DefaultHook {
    /// Creates a new `DefaultHook` instance.
    ///
    /// # Arguments
    ///
    /// - `&Context` - The context (unused).
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance of `DefaultHook`.
    async fn new(_: &Context) -> Self {
        Self
    }

    /// Handles the hook execution (no-op).
    ///
    /// # Arguments
    ///
    /// - `&Context` - The context (unused).
    async fn handle(self, _: &Context) {}
}

use super::*;

/// Implementation of methods for the Response structure.
impl Response {
    /// Creates a new Response from data that can be converted into ResponseData.
    ///
    /// # Arguments
    ///
    /// - `Into<ResponseData>` - Data that can be converted into ResponseData.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Response instance.
    pub fn from<T>(data: T) -> Self
    where
        T: Into<ResponseData>,
    {
        Self(data.into())
    }

    /// Gets a reference to the response data.
    ///
    /// # Returns
    ///
    /// - `&ResponseData` - Reference to the response data.
    pub fn get_data(&self) -> &ResponseData {
        &self.0
    }

    /// Gets the response data as a mutable reference.
    ///
    /// # Returns
    ///
    /// - `&mut ResponseData` - Mutable reference to the response data.
    pub fn get_mut_data(&mut self) -> &mut ResponseData {
        &mut self.0
    }

    /// Sets the response data.
    ///
    /// # Arguments
    ///
    /// - `Into<ResponseData>` - Data that can be converted into ResponseData.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub fn set_data<T>(&mut self, data: T) -> &mut Self
    where
        T: Into<ResponseData>,
    {
        self.0 = data.into();
        self
    }

    /// Clears the response data.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference to self for method chaining.
    pub fn clear(&mut self) -> &mut Self {
        self.0.clear();
        self
    }

    /// Checks if the response data is empty.
    ///
    /// # Returns
    ///
    /// - `bool` - True if the response data is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the length of the response data.
    ///
    /// # Returns
    ///
    /// - `usize` - The length of the response data in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Attempts to send the response through the provided stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to send the response through.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_send(&self, stream: &ArcRwLockStream) -> ResponseResult {
        stream.try_send(&self.0).await
    }

    /// Sends the response through the provided stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to send the response through.
    ///
    /// # Panics
    ///
    /// Panics if the send operation fails.
    pub async fn send(&self, stream: &ArcRwLockStream) {
        self.try_send(stream).await.unwrap();
    }

    /// Attempts to flush the stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to flush.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_flush(&self, stream: &ArcRwLockStream) -> ResponseResult {
        stream.try_flush().await
    }

    /// Flushes the stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to flush.
    ///
    /// # Panics
    ///
    /// Panics if the flush operation fails.
    pub async fn flush(&self, stream: &ArcRwLockStream) {
        self.try_flush(stream).await.unwrap();
    }

    /// Attempts to shut down the stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to shut down.
    ///
    /// # Returns
    ///
    /// - `ResponseResult` - Ok(()) on success, or an error on failure.
    pub async fn try_close(&self, stream: &ArcRwLockStream) -> ResponseResult {
        stream.try_shutdown().await
    }

    /// Shuts down the stream.
    ///
    /// # Arguments
    ///
    /// - `&ArcRwLockStream` - The stream to shut down.
    ///
    /// # Panics
    ///
    /// Panics if the shutdown operation fails.
    pub async fn close(&self, stream: &ArcRwLockStream) {
        self.try_close(stream).await.unwrap();
    }
}

/// Implementation of `From<ResponseData>` for `Response`.
impl From<ResponseData> for Response {
    /// Converts ResponseData into a Response.
    ///
    /// # Arguments
    ///
    /// - `ResponseData` - The response data to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Response instance.
    fn from(data: ResponseData) -> Self {
        Self(data)
    }
}

/// Implementation of `From<&[u8]>` for `Response`.
impl From<&[u8]> for Response {
    /// Converts a byte slice into a Response.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The byte slice to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Response instance.
    fn from(data: &[u8]) -> Self {
        Self(data.to_owned())
    }
}

/// Implementation of `From<String>` for `Response`.
impl From<String> for Response {
    /// Converts a String into a Response.
    ///
    /// # Arguments
    ///
    /// - `String` - The string to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Response instance.
    fn from(data: String) -> Self {
        Self(data.into_bytes())
    }
}

/// Implementation of `From<&str>` for `Response`.
impl From<&str> for Response {
    /// Converts a string slice into a Response.
    ///
    /// # Arguments
    ///
    /// - `&str` - The string slice to convert.
    ///
    /// # Returns
    ///
    /// - `Self` - A new Response instance.
    fn from(data: &str) -> Self {
        Self(data.as_bytes().to_owned())
    }
}

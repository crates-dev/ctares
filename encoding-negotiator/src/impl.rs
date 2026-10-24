use super::*;

impl Default for CompressionPolicy {
    /// Builds the policy that is used when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default policy.
    fn default() -> Self {
        Self {
            min_size: DEFAULT_MIN_SIZE,
            encodings: vec![ENCODING_GZIP.to_owned(), ENCODING_BROTLI.to_owned()],
        }
    }
}

impl CompressionPolicy {
    /// The encoding the client asked for, when one of the offered encodings
    /// is accepted.
    ///
    /// The offered encodings are tried in the order they are preferred, so the
    /// first one the client accepts is the one that is used.
    ///
    /// # Arguments
    ///
    /// - `&str` - The `accept-encoding` header of the request.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The encoding to answer with, when the client
    ///   accepts one of the offered encodings.
    pub fn pick(&self, accept_encoding: &str) -> Option<String> {
        let tokens: Vec<String> = self.accepted(accept_encoding);
        for encoding in self.get_encodings().iter() {
            if encoding.is_empty() {
                continue;
            }
            if tokens
                .iter()
                .any(|token: &String| token == TOKEN_ANY || token == encoding)
            {
                return Some(encoding.clone());
            }
        }
        None
    }

    /// The encodings the client accepts, without the ones it refused.
    ///
    /// An encoding that is refused with a quality of zero is dropped, and the
    /// tokens are read in the case the client wrote them in.
    ///
    /// # Arguments
    ///
    /// - `&str` - The `accept-encoding` header of the request.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - The tokens the client accepts.
    pub fn accepted(&self, accept_encoding: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        for part in accept_encoding.split(SEPARATOR_COMMA) {
            let token: String = part.split(';').next().unwrap_or("").trim().to_lowercase();
            if token.is_empty() {
                continue;
            }
            let refused: bool = part.split(';').skip(1).any(|parameter: &str| {
                let quality: Option<&str> = parameter.trim().strip_prefix(PARAMETER_QUALITY);
                match quality {
                    Some(quality) => quality
                        .parse::<f32>()
                        .map(|value: f32| value <= 0.0)
                        .unwrap_or(false),
                    None => false,
                }
            });
            if !refused {
                tokens.push(token);
            }
        }
        tokens
    }

    /// Whether a body of that content type is worth compressing.
    ///
    /// A body that is already compressed is sent as it is, and the stream of
    /// events is never compressed. The rule does not depend on the encodings
    /// that are offered, so it is asked of the policy itself.
    ///
    /// # Arguments
    ///
    /// - `&str` - The content type of the answer.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the body may be compressed.
    pub fn is_compressible(content_type: &str) -> bool {
        let content_type: String = content_type.trim().to_lowercase();
        if content_type.is_empty() {
            return true;
        }
        if content_type == TEXT_EVENT_STREAM {
            return false;
        }
        if CONTENT_TYPES_COMPRESSED.contains(&content_type.as_str()) {
            return false;
        }
        !CONTENT_TYPE_PREFIXES_COMPRESSED
            .iter()
            .any(|prefix: &&str| content_type.starts_with(*prefix))
    }
}

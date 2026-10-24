use super::*;

impl Default for CorsPolicy {
    /// Builds the policy that is used when none is given.
    ///
    /// The policy allows every origin, every method and every request header,
    /// and it writes no credentials.
    ///
    /// # Returns
    ///
    /// - `Self` - The default policy.
    fn default() -> Self {
        Self {
            allowed_origins: vec![WILDCARD_ANY.to_owned()],
            allowed_methods: ALL_METHODS
                .iter()
                .map(|method: &&str| method.to_string())
                .collect(),
            allowed_headers: vec![WILDCARD_ANY.to_owned()],
            max_age: None,
            allow_credentials: false,
        }
    }
}

impl CorsPolicy {
    /// The origin that is written back, when the origin is allowed.
    ///
    /// A wildcard answers with the wildcard itself, a list answers with the
    /// origin that was asked for.
    ///
    /// # Arguments
    ///
    /// - `&str` - The origin the caller asked with.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The origin to write back, when the origin is
    ///   allowed.
    pub fn allowed_origin(&self, origin: &str) -> Option<String> {
        if self
            .get_allowed_origins()
            .iter()
            .any(|allowed: &String| allowed == WILDCARD_ANY)
        {
            return Some(WILDCARD_ANY.to_owned());
        }
        if self
            .get_allowed_origins()
            .iter()
            .any(|allowed: &String| allowed == origin)
        {
            return Some(origin.to_owned());
        }
        None
    }

    /// The cross origin headers of the answer, when the origin is allowed.
    ///
    /// The headers are the ones a browser reads to learn whether it may read
    /// the answer of another origin.
    ///
    /// # Arguments
    ///
    /// - `&str` - The origin the caller asked with.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<(&'static str, String)>>` - The headers to write, when
    ///   the origin is allowed.
    pub fn headers(&self, origin: &str) -> Option<Vec<(&'static str, String)>> {
        let allowed: String = self.allowed_origin(origin)?;
        let mut headers: Vec<(&'static str, String)> = vec![
            (HEADER_ALLOW_ORIGIN, allowed),
            (
                HEADER_ALLOW_METHODS,
                self.get_allowed_methods().join(SEPARATOR),
            ),
            (
                HEADER_ALLOW_HEADERS,
                self.get_allowed_headers().join(SEPARATOR),
            ),
        ];
        if let Some(max_age) = self.get_max_age() {
            headers.push((HEADER_MAX_AGE, max_age.to_string()));
        }
        if self.get_allow_credentials() {
            headers.push((HEADER_ALLOW_CREDENTIALS, VALUE_TRUE.to_owned()));
        }
        headers.push((HEADER_VARY, VALUE_ORIGIN.to_owned()));
        Some(headers)
    }
}

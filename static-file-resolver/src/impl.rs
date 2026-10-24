use super::*;

impl Default for StaticFileResolver<'_> {
    /// Builds the resolver that is used when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default resolver.
    fn default() -> Self {
        Self {
            root: Path::new("."),
            index: INDEX_DEFAULT,
        }
    }
}

impl StaticFileResolver<'_> {
    /// The file a path names under the root, when the path is allowed.
    ///
    /// A path that climbs out of the root is refused, so a request cannot
    /// reach a file the root does not hold.
    ///
    /// # Arguments
    ///
    /// - `&str` - The path of the file, relative to the root.
    ///
    /// # Returns
    ///
    /// - `Option<PathBuf>` - The file, when the path is allowed.
    pub fn resolve(&self, path: &str) -> Option<PathBuf> {
        let mut file: PathBuf = self.get_root().to_path_buf();
        if path.is_empty() {
            file.push(self.get_index());
            return Some(file);
        }
        for segment in path.split(SEPARATOR_SLASH) {
            if segment.is_empty()
                || segment == SEGMENT_CURRENT
                || segment == SEGMENT_PARENT
                || segment.contains(SEPARATOR_BACKSLASH)
            {
                return None;
            }
            file.push(segment);
        }
        Some(file)
    }
}

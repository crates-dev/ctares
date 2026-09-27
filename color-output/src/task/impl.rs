use super::*;

/// Default implementation for Task with empty text list.
impl<'a> Default for Task<'a> {
    #[inline(always)]
    fn default() -> Self {
        Self { text_list: vec![] }
    }
}

/// Implementation of task operations.
impl<'a> Task<'a> {
    /// Gets a reference to the text list.
    ///
    /// # Returns
    ///
    /// - `&Vec<Text<'a>>` - Reference to the text list.
    #[inline(always)]
    pub fn get_text_list(&self) -> &Vec<Text<'a>> {
        &self.text_list
    }

    /// Gets a mutable reference to the text list.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<Text<'a>>` - Mutable reference to the text list.
    #[inline(always)]
    pub fn get_mut_text_list(&mut self) -> &mut Vec<Text<'a>> {
        &mut self.text_list
    }

    /// Adds a text configuration to the task list.
    ///
    /// # Arguments
    ///
    /// - `Text` - The text configuration to add
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The task for method chaining
    #[inline(always)]
    pub fn add(&mut self, new_text: Text<'a>) -> &mut Self {
        self.get_mut_text_list().push(new_text);
        self
    }

    /// Clears all text configurations from the task list.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The cleared task for method chaining
    #[inline(always)]
    pub(crate) fn clear(&mut self) -> &mut Self {
        self.get_mut_text_list().clear();
        self
    }

    /// Runs all tasks in the list.
    ///
    /// # Arguments
    ///
    /// - `&mut Self` - The mutable task instance.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The task instance after execution.
    pub fn run_all(&mut self) -> &mut Self {
        let text_list: Vec<Text<'_>> = self.get_text_list().clone();
        self.clear();
        let mut output_str: String = String::with_capacity(text_list.len());
        for text in text_list.iter() {
            output_str.push_str(&Text::new_from(text).get_display_str_cow());
        }
        print!("{output_str}");
        std::io::stdout().flush().unwrap();
        self
    }
}

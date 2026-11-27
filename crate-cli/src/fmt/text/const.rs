/// Single space character used for indentation and padding.
pub(crate) const TEXT_SPACE: char = ' ';

/// Horizontal tab character.
pub(crate) const TEXT_TAB: char = '\t';

/// Carriage return character normalized away during LF conversion.
pub(crate) const TEXT_CR: char = '\r';

/// Line feed character enforced across all formatted text files.
pub(crate) const TEXT_LF: char = '\n';

/// Standard Prettier tab width configured in VS Code settings (`prettier.tabWidth: 2`).
pub(crate) const PRETTIER_TAB_WIDTH: usize = 2;

/// Standard Prettier print width default (`80` columns).
pub(crate) const PRETTIER_PRINT_WIDTH: usize = 80;

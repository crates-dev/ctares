/// Read a line from standard input as a `String`.
///
/// # Returns
/// - A `String` containing the input line (including trailing newline if present).
#[macro_export]
macro_rules! cin {
    () => {{
        let mut input: String = String::new();
        let _: ::std::io::Result<usize> = std::io::stdin().read_line(&mut input);
        input
    }};
}

/// Parse input string into a value or a vector of values of a specified type.
///
/// # Parameters
/// - `input`: The input `&str` to be parsed.
/// - `type`: The target type to parse into.
///
/// # Returns
/// - A single value of the specified type if used in scalar mode.
/// - A `Vec` of values if used in vector mode.
#[macro_export]
macro_rules! cin_parse {
    ($input: expr, Vec<$type: ty>) => {{
        let mut res: Vec<$type> = Vec::<$type>::new();
        for token in $input.split_whitespace() {
            if let Ok(num) = token.parse::<$type>() {
                res.push(num);
            }
        }
        res
    }};
    ($input: expr, $type: ty) => {{
        let mut res: $type = Default::default();
        if let Ok(parse_res) = $input.parse::<$type>() {
            res = parse_res;
        }
        res
    }};
}

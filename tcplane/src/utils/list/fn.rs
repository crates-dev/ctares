/// Removes trailing zero bytes from a byte vector.
///
/// This function truncates the vector at the last non-zero byte,
/// or clears it entirely if all bytes are zero.
///
/// # Arguments
///
/// - `&mut Vec<u8>` - The byte vector to process.
///
/// # Returns
///
/// - `Vec<u8>` - A clone of the modified vector.
pub fn remove_trailing_zeros(data: &mut Vec<u8>) -> Vec<u8> {
    if let Some(last_non_zero_pos) = data.iter().rposition(|&x| x != 0) {
        data.truncate(last_non_zero_pos + 1);
    } else {
        data.clear();
    }
    data.clone()
}

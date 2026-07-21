use super::*;

#[test]
fn test() {
    let combined_path: String = join_paths!("/home/", "/user/", "/documents", "file.txt");
    assert_eq!(combined_path, "/home/user/documents/file.txt");
    let another_path: String = join_paths!("C:/", "/Program Files", "App");
    assert_eq!(another_path, "C:/Program Files/App");
}

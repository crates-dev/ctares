use super::*;

#[tokio::test]
async fn test_clone() {
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(s1, s2 => {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    });
    assert_eq!(res, format!("{s1} {s2}"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res: String = clone!(s1, s2 => async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}")
    })
    .await;
    assert_eq!(res, format!("{s1} {s2}"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 =>  |data| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!").await, String::from("Hello World!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data: &str| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => |data: String| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!").await, format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data: &str| {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!"), format!("{} {}{}", s1, s2, "!"));
    let s1: String = String::from("Hello");
    let s2: String = String::from("World");
    let res = clone!(s1, s2 => move |data: String| async move {
        assert_eq!(s1, String::from("Hello"));
        assert_eq!(s2, String::from("World"));
        format!("{s1} {s2}{data}")
    });
    assert_eq!(res("!".to_owned()).await, format!("{} {}{}", s1, s2, "!"));
}

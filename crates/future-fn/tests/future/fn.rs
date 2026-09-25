use super::*;

#[tokio::test]
async fn test() {
    let string: String = String::from("test");
    let number: i32 = 1;
    let future = future_fn!(string, number, {
        let tmp_string: String = String::from("test");
        assert_eq!(string, tmp_string);
        assert_eq!(number, 1);
    });
    future().await;
    let future = future_fn!(string, number, |data| {
        let tmp_string: String = String::from("test");
        sleep(Duration::from_millis(360)).await;
        assert_eq!(string, tmp_string);
        assert_eq!(data, 1);
        assert_eq!(number, 1);
    });
    future(1).await;
    let future = future_fn!(string, number, |data: i32| {
        let tmp_string: String = String::from("test");
        sleep(Duration::from_millis(360)).await;
        assert_eq!(string, tmp_string);
        assert_eq!(data, 1);
        assert_eq!(number, 1);
        sleep(Duration::from_millis(360)).await;
    });
    future(1).await;
    let future = future_fn!(string, number, |data: i32| {
        let tmp_string: String = String::from("test");
        sleep(Duration::from_millis(360)).await;
        assert_eq!(string, tmp_string);
        assert_eq!(data, 1);
        assert_eq!(number, 1);
    });
    future(1).await;
}

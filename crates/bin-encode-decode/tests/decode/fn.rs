use super::*;

#[test]
fn test_decode() {
    let mut charset: String = String::new();
    for i in 0..26 {
        let ch: char = (b'a' + i) as char;
        charset.push(ch);
    }
    for i in 0..26 {
        let ch: char = (b'A' + i) as char;
        charset.push(ch);
    }
    for i in 0..10 {
        let ch: char = (b'0' + i) as char;
        charset.push(ch);
    }
    charset.push_str("_=");
    let encode_res: Result<String, DecodeError> = Decode::execute(&charset, "aab0aabLaabZaab0");
    assert_eq!(encode_res.unwrap(), "test");
}

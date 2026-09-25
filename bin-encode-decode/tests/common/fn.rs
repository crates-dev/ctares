use super::*;

#[test]
fn test_crypt_decode() {
    let mut en_decode: Charset<'_> = Charset::new();
    let test_str: &str = "test";
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
    en_decode.charset(&charset);
    let encode: Result<String, EncodeError> = en_decode.encode(test_str);
    let decode: Result<String, DecodeError> = en_decode.decode(&encode.clone().unwrap());
    assert_eq!(decode.unwrap(), test_str);
}

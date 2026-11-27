use super::*;

#[test]
fn test_text() {
    let text_default: Text<'_> = Text::default();
    let text_default_str: &String = &text_default.get_display_str_cow().into_owned();
    let mut text: Text<'_> = Text::default();
    text.set_text("")
        .set_color(ColorType::default())
        .set_bg_color(ColorType::default())
        .set_bold(false)
        .set_endl(false);
    let text_str: &String = &text.get_display_str_cow().into_owned();
    assert_eq!(text_default_str, text_str);
}

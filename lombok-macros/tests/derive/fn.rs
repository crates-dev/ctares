use super::*;

fn read_fn_rs() -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generate/fn.rs");
    read_to_string(&path).expect("generate/fn.rs is readable")
}

#[test]
fn new_uses_field_shorthand_initialisers() {
    let source: String = read_fn_rs();
    assert!(
        !source.contains("quote! { #original_name: #original_name }"),
        "New derive must emit the shorthand `Self {{ name }}`, not `Self {{ name: name }}`"
    );
}

#[test]
fn skipped_field_still_defaults_explicitly() {
    let source: String = read_fn_rs();
    assert!(
        source.contains("quote! { #original_name: Default::default() }"),
        "a #[new(skip)] field must be initialised with Default::default()"
    );
}

#[test]
fn new_constructor_is_usable() {
    let plain: NoSkip = NoSkip::new("hello".to_string(), 3);
    assert_eq!(plain.name, "hello");
    assert_eq!(plain.count, 3);
    let skipped: Skipped = Skipped::new("world".to_string());
    assert_eq!(skipped.name, "world");
    assert_eq!(skipped.schema, 0);
}

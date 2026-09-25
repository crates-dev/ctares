use super::*;

#[test]
fn test_args_default_values() {
    let args: Args = Args {
        command: CommandType::Help,
        check: false,
        manifest_path: None,
        bump_type: None,
        max_retries: 8,
    };
    assert!(!args.check);
    assert_eq!(args.max_retries, 8);
    assert!(args.manifest_path.is_none());
    assert!(args.bump_type.is_none());
}

#[test]
fn test_args_with_values() {
    let args: Args = Args {
        command: CommandType::Bump,
        check: true,
        manifest_path: Some("./test/Cargo.toml".to_string()),
        bump_type: Some(BumpVersionType::Minor),
        max_retries: 5,
    };
    assert!(args.check);
    assert_eq!(args.max_retries, 5);
    assert_eq!(args.manifest_path, Some("./test/Cargo.toml".to_string()));
    assert_eq!(args.bump_type, Some(BumpVersionType::Minor));
}

#[test]
fn test_command_type_enum_values() {
    let _: CommandType = CommandType::Fmt;
    let _: CommandType = CommandType::Bump;
    let _: CommandType = CommandType::Publish;
    let _: CommandType = CommandType::Sync;
    let _: CommandType = CommandType::Help;
    let _: CommandType = CommandType::Version;
}

#[test]
fn test_args_clone() {
    let args: Args = Args {
        command: CommandType::Bump,
        check: true,
        manifest_path: Some("./test/Cargo.toml".to_string()),
        bump_type: Some(BumpVersionType::Minor),
        max_retries: 5,
    };
    let cloned: Args = args.clone();
    assert_eq!(cloned.check, args.check);
    assert_eq!(cloned.max_retries, args.max_retries);
    assert_eq!(cloned.manifest_path, args.manifest_path);
    assert_eq!(cloned.bump_type, args.bump_type);
}

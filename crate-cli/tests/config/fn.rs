use super::*;

#[test]
fn test_args_default_values() {
    let args: Args = Args::new(CommandType::Help, false, None, None, 8);
    assert!(!args.get_check());
    assert_eq!(args.get_max_retries(), 8);
    assert!(args.try_get_manifest_path().is_none());
    assert!(args.try_get_bump_type().is_none());
}

#[test]
fn test_args_with_values() {
    let args: Args = Args::new(
        CommandType::Bump,
        true,
        Some("./test/Cargo.toml".to_string()),
        Some(BumpVersionType::Minor),
        5,
    );
    assert!(args.get_check());
    assert_eq!(args.get_max_retries(), 5);
    assert_eq!(
        args.try_get_manifest_path().as_deref(),
        Some("./test/Cargo.toml")
    );
    assert_eq!(args.try_get_bump_type(), &Some(BumpVersionType::Minor));
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
    let args: Args = Args::new(
        CommandType::Bump,
        true,
        Some("./test/Cargo.toml".to_string()),
        Some(BumpVersionType::Minor),
        5,
    );
    let cloned: Args = args.clone();
    assert_eq!(cloned.get_check(), args.get_check());
    assert_eq!(cloned.get_max_retries(), args.get_max_retries());
    assert_eq!(cloned.try_get_manifest_path(), args.try_get_manifest_path());
    assert_eq!(cloned.try_get_bump_type(), args.try_get_bump_type());
}

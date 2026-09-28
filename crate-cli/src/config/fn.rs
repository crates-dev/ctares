use super::*;

/// Parse command line arguments
///
/// # Returns
///
/// - `Args` - Parsed arguments
pub fn parse_args() -> Args {
    let raw_args: Vec<String> = args().collect();
    let mut command: CommandType = CommandType::Help;
    let mut check: bool = false;
    let mut manifest_path: Option<String> = None;
    let mut bump_type: Option<BumpVersionType> = None;
    let mut max_retries: u32 = 8;
    let mut i: usize = 1;
    while i < raw_args.len() {
        let arg: &str = raw_args[i].as_str();
        match arg {
            "-h" | CLI_FLAG_HELP => {
                command = CommandType::Help;
            }
            "-v" | CLI_FLAG_VERSION => {
                command = CommandType::Version;
            }
            "fmt" if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Fmt;
            }
            CLI_BUMP if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Bump;
            }
            CLI_PUBLISH if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Publish;
            }
            CLI_SYNC if (command == CommandType::Help || command == CommandType::Version) => {
                command = CommandType::Sync;
            }
            CLI_FLAG_PATCH => {
                bump_type = Some(BumpVersionType::Patch);
            }
            CLI_FLAG_MINOR => {
                bump_type = Some(BumpVersionType::Minor);
            }
            CLI_FLAG_MAJOR => {
                bump_type = Some(BumpVersionType::Major);
            }
            CLI_FLAG_RELEASE => {
                bump_type = Some(BumpVersionType::Release);
            }
            CLI_FLAG_ALPHA => {
                bump_type = Some(BumpVersionType::Alpha);
            }
            CLI_FLAG_BETA => {
                bump_type = Some(BumpVersionType::Beta);
            }
            CLI_FLAG_RC => {
                bump_type = Some(BumpVersionType::Rc);
            }
            CLI_FLAG_CHECK => {
                check = true;
            }
            CLI_FLAG_MANIFEST_PATH => {
                i += 1;
                if i < raw_args.len() {
                    manifest_path = Some(raw_args[i].clone());
                }
            }
            CLI_FLAG_MAX_RETRIES => {
                i += 1;
                if i < raw_args.len()
                    && let Ok(n) = raw_args[i].parse::<u32>()
                {
                    max_retries = n;
                }
            }
            _ => {}
        }
        i += 1;
    }
    Args {
        command,
        check,
        manifest_path,
        bump_type,
        max_retries,
    }
}

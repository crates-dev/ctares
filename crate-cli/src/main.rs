use crate_cli::*;

use std::process::exit;

/// Run the requested subcommand and exit non-zero on failure.
#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Info);
    let args: Args = parse_args();
    match args.get_command() {
        CommandType::Fmt => {
            if let Err(error) = execute_fmt(&args).await {
                log::error!("fmt failed: {error}");
                exit(1);
            }
        }
        CommandType::Bump => {
            let manifest_path: &str = args
                .try_get_manifest_path()
                .as_deref()
                .unwrap_or(CARGO_TOML);
            let bump_type: &BumpVersionType = args
                .try_get_bump_type()
                .as_ref()
                .unwrap_or(&BumpVersionType::Patch);
            match execute_bump(manifest_path, bump_type).await {
                Ok(result) => {
                    log::info!("bump: {result}");
                }
                Err(error) => {
                    log::error!("bump failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::Publish => {
            let manifest_path: &str = args
                .try_get_manifest_path()
                .as_deref()
                .unwrap_or(CARGO_TOML);
            let max_retries: u32 = args.get_max_retries();
            match execute_publish(manifest_path, max_retries).await {
                Ok(results) => {
                    let failed_count: usize = results
                        .iter()
                        .filter(|r: &&PublishResult| !r.get_success())
                        .count();
                    if failed_count > 0 {
                        log::error!("Publish completed with {failed_count} failures");
                        exit(1);
                    } else {
                        log::info!("All packages published successfully");
                    }
                }
                Err(error) => {
                    log::error!("publish failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::Sync => {
            let manifest_path: &str = args
                .try_get_manifest_path()
                .as_deref()
                .unwrap_or(CARGO_TOML);
            match execute_sync(manifest_path).await {
                Ok(report) => {
                    log::info!(
                        "sync complete: v{} (renamed {}, versioned {}, file_changed {})",
                        report.get_workspace_version(),
                        report.get_renamed_entries().len(),
                        report.get_versioned_entries().len(),
                        report.get_file_changed(),
                    );
                }
                Err(error) => {
                    log::error!("sync failed: {error}");
                    exit(1);
                }
            }
        }
        CommandType::Help => print_help(),
        CommandType::Version => print_version(),
    }
}

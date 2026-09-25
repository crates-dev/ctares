//! crates-cli
//!
//! A command-line tool for managing Cargo package lifecycles:
//! version bump, workspace dependency sync, ordered publish and
//! code formatting.

use crates_cli::*;

use std::process::exit;

#[tokio::main]
async fn main() {
    Logger::init(log::LevelFilter::Info);
    let args: Args = parse_args();
    match args.command {
        CommandType::Fmt => {
            if let Err(error) = execute_fmt(&args).await {
                log::error!("fmt failed: {error}");
                exit(1);
            }
        }
        CommandType::Bump => {
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let bump_type: BumpVersionType = args.bump_type.unwrap_or(BumpVersionType::Patch);
            match execute_bump(&manifest_path, &bump_type).await {
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
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            let max_retries: u32 = args.max_retries;
            match execute_publish(&manifest_path, max_retries).await {
                Ok(results) => {
                    let failed_count: usize = results
                        .iter()
                        .filter(|r: &&PublishResult| !r.success)
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
            let manifest_path: String = args
                .manifest_path
                .unwrap_or_else(|| "Cargo.toml".to_string());
            match execute_sync(&manifest_path).await {
                Ok(report) => {
                    log::info!(
                        "sync complete: v{} (renamed {}, versioned {}, file_changed {})",
                        report.workspace_version,
                        report.renamed_entries.len(),
                        report.versioned_entries.len(),
                        report.file_changed,
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

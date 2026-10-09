use super::*;

/// Discover all packages in the workspace: every `[workspace.members]`
/// entry in declaration order, with the root package (when the workspace
/// root manifest also declares `[package]`) appended last.
///
/// # Arguments
///
/// - `&Path` - Path to workspace root Cargo.toml
///
/// # Returns
///
/// - `Result<(Vec<Package>, bool), PublishError>` - Packages and whether a
///   root package was appended
async fn discover_packages(
    workspace_manifest: &Path,
) -> Result<(Vec<Package>, bool), PublishError> {
    let content: String = read_to_string(workspace_manifest).await?;
    let doc: Value = toml::from_str(&content)
        .map_err(|_error: toml::de::Error| PublishError::ManifestParseError)?;
    let workspace_version: Option<String> = doc
        .get(TOML_WORKSPACE)
        .and_then(|workspace: &Value| workspace.get(TOML_PACKAGE))
        .and_then(|package: &Value| package.get(TOML_VERSION))
        .and_then(|version: &Value| version.as_str())
        .map(|version: &str| version.to_string());
    let mut packages: Vec<Package> = Vec::new();
    if let Some(workspace) = doc.get(TOML_WORKSPACE)
        && let Some(members) = workspace
            .get(TOML_MEMBERS)
            .and_then(|members_value: &Value| members_value.as_array())
    {
        for member in members {
            if let Some(pattern) = member.as_str() {
                let base_path: &Path = workspace_manifest.parent().unwrap_or(workspace_manifest);
                expand_pattern(
                    base_path,
                    pattern,
                    &mut packages,
                    workspace_version.as_deref(),
                )
                .await?;
            }
        }
    }
    let has_root_package: bool = doc.get(TOML_PACKAGE).is_some();
    if has_root_package {
        let root_package: Package =
            read_package_manifest(workspace_manifest, workspace_version.as_deref()).await?;
        packages.push(root_package);
    }
    Ok((packages, has_root_package))
}

/// Expand glob pattern to find package directories
///
/// # Arguments
///
/// - `&Path` - Base path for expansion
/// - `&str` - Glob pattern
/// - `&mut Vec<Package>` - Output vector for found packages
/// - `Option<&str>` - Workspace root version for `version.workspace = true`
///
/// # Returns
///
/// - `Result<(), PublishError>` - Success or error
async fn expand_pattern(
    base_path: &Path,
    pattern: &str,
    packages: &mut Vec<Package>,
    workspace_version: Option<&str>,
) -> Result<(), PublishError> {
    if pattern.contains('*') {
        let parent: &Path = Path::new(pattern).parent().unwrap_or(Path::new("."));
        let full_parent: PathBuf = base_path.join(parent);
        if full_parent.is_dir() {
            let mut entries: ReadDir = read_dir(&full_parent).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    let cargo_toml: PathBuf = path.join(CARGO_TOML);
                    if cargo_toml.exists() {
                        let package: Package =
                            read_package_manifest(&cargo_toml, workspace_version).await?;
                        packages.push(package);
                    }
                }
            }
        }
    } else {
        let cargo_toml: PathBuf = base_path.join(pattern).join(CARGO_TOML);
        if cargo_toml.exists() {
            let package: Package = read_package_manifest(&cargo_toml, workspace_version).await?;
            packages.push(package);
        }
    }
    Ok(())
}

/// Read package manifest and extract information
///
/// `version.workspace = true` resolves to the workspace root version
/// passed in `workspace_version`.
///
/// # Arguments
///
/// - `&Path` - Path to package Cargo.toml
/// - `Option<&str>` - Workspace root `[workspace.package].version`, if any
///
/// # Returns
///
/// - `Result<Package, PublishError>` - Package info or error
async fn read_package_manifest(
    manifest_path: &Path,
    workspace_version: Option<&str>,
) -> Result<Package, PublishError> {
    let content: String = read_to_string(manifest_path).await?;
    let doc: Value = toml::from_str(&content)
        .map_err(|_error: toml::de::Error| PublishError::ManifestParseError)?;
    let package_table: &Value = doc
        .get(TOML_PACKAGE)
        .ok_or(PublishError::ManifestParseError)?;
    let name: String = package_table
        .get(TOML_NAME)
        .and_then(|n: &Value| n.as_str())
        .ok_or(PublishError::ManifestParseError)?
        .to_string();
    let version: String = match package_table.get(TOML_VERSION) {
        Some(version_value) => {
            if let Some(version_str) = version_value.as_str() {
                version_str.to_string()
            } else if version_value
                .get(TOML_WORKSPACE)
                .and_then(|workspace_value: &Value| workspace_value.as_bool())
                .unwrap_or(false)
            {
                workspace_version
                    .ok_or(PublishError::ManifestParseError)?
                    .to_string()
            } else {
                return Err(PublishError::ManifestParseError);
            }
        }
        None => workspace_version
            .ok_or(PublishError::ManifestParseError)?
            .to_string(),
    };
    let publish: bool = package_table
        .get(TOML_PUBLISH_KEY)
        .and_then(|publish_value: &Value| publish_value.as_bool())
        .unwrap_or(true);
    let path: PathBuf = manifest_path
        .parent()
        .filter(|p: &&Path| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), |p: &Path| p.to_path_buf());
    let local_dependencies: Vec<String> = extract_local_dependencies(&doc, manifest_path)?;
    Ok(Package {
        name,
        version,
        path,
        local_dependencies,
        publish,
    })
}

/// Extract local workspace dependencies that constrain publish order
///
/// `[dependencies]` and `[build-dependencies]` entries with `path` or
/// `workspace = true` always constrain. `[dev-dependencies]` are stripped
/// from the published manifest, so they constrain only when they carry a
/// `version` field (cargo publish registry-checks versioned dev-deps);
/// path-only dev-deps skip the registry and impose no order constraint.
///
/// # Arguments
///
/// - `&Value` - Parsed manifest
/// - `&Path` - Path to manifest for resolving relative paths
///
/// # Returns
///
/// - `Result<Vec<String>, PublishError>` - List of local dependency names
fn extract_local_dependencies(
    doc: &Value,
    _manifest_path: &Path,
) -> Result<Vec<String>, PublishError> {
    let mut deps: Vec<String> = Vec::new();
    let dep_sections: [&str; 3] = [
        TOML_DEPENDENCIES,
        TOML_BUILD_DEPENDENCIES,
        TOML_DEV_DEPENDENCIES,
    ];
    for section in &dep_sections {
        if let Some(table) = doc
            .get(section)
            .and_then(|section_value: &Value| section_value.as_table())
        {
            for (dep_name, dep_value) in table {
                let is_local: bool = match dep_value {
                    Value::Table(t) => {
                        let has_path_or_workspace: bool = t.get(TOML_PATH).is_some()
                            || t.get(TOML_WORKSPACE)
                                .and_then(|workspace_value: &Value| workspace_value.as_bool())
                                .unwrap_or(false);
                        let versioned: bool = t.get(TOML_VERSION).is_some();
                        has_path_or_workspace && (*section != TOML_DEV_DEPENDENCIES || versioned)
                    }
                    _ => false,
                };
                if is_local {
                    deps.push(dep_name.clone());
                }
            }
        }
    }
    Ok(deps)
}

/// Validate that the publish order satisfies every package's local
/// dependency constraints: a package must never appear before a
/// workspace-local dependency of its own.
///
/// # Arguments
///
/// - `&[Package]` - Packages in intended publish order
///
/// # Returns
///
/// - `Result<(), PublishError>` - `InvalidPublishOrder` naming the first
///   offending pair when the order violates a local dependency.
fn validate_publish_order(packages: &[Package]) -> Result<(), PublishError> {
    let position: HashMap<String, usize> = packages
        .iter()
        .enumerate()
        .map(|(index, package): (usize, &Package)| (package.name.clone(), index))
        .collect();
    for package in packages {
        let Some(package_position) = position.get(&package.name) else {
            continue;
        };
        for dep in &package.local_dependencies {
            if let Some(dep_position) = position.get(dep)
                && dep_position > package_position
            {
                return Err(PublishError::InvalidPublishOrder(format!(
                    "{} depends on {} but is listed before it in [workspace.members]",
                    package.name, dep
                )));
            }
        }
    }
    Ok(())
}

/// Move the root package (appended last by `discover_packages`) to its
/// topological position when workspace members depend on it
///
/// When no member depends on the root package the root stays last (the
/// conventional facade-last layout). When members do depend on the root
/// (e.g. `ui` / `engine` crates depending on a root facade crate), the
/// root is inserted right before the earliest such member, provided all
/// of the root's own local dependencies appear earlier in the members
/// order; otherwise the members order cannot satisfy both constraints
/// and `InvalidPublishOrder` is returned.
///
/// # Arguments
///
/// - `&mut Vec<Package>` - Packages with the root package as last element
///
/// # Returns
///
/// - `Result<(), PublishError>` - Success or `InvalidPublishOrder`
fn position_root_package(packages: &mut Vec<Package>) -> Result<(), PublishError> {
    let Some(root) = packages.pop() else {
        return Ok(());
    };
    let earliest_dependent: Option<usize> = packages
        .iter()
        .enumerate()
        .filter(|(_index, package): &(usize, &Package)| {
            package.local_dependencies.contains(&root.name)
        })
        .map(|(index, _item): (usize, &Package)| index)
        .min();
    let Some(earliest) = earliest_dependent else {
        packages.push(root);
        return Ok(());
    };
    let member_positions: HashMap<&str, usize> = packages
        .iter()
        .enumerate()
        .map(|(index, package): (usize, &Package)| (package.name.as_str(), index))
        .collect();
    if let Some(max_dep) = root
        .local_dependencies
        .iter()
        .filter_map(|dep: &String| member_positions.get(dep.as_str()))
        .max()
        && max_dep >= &earliest
    {
        return Err(PublishError::InvalidPublishOrder(format!(
            "{} must publish after its dependency at members position {} but before dependent at position {}; reorder [workspace.members]",
            root.name, max_dep, earliest
        )));
    }
    packages.insert(earliest, root);
    Ok(())
}

/// Resolve the publish order for a workspace: `[workspace.members]`
/// declaration order, with the root package (if any) placed at its
/// topological position, validated against local dependency constraints.
///
/// # Arguments
///
/// - `&str` - Path to the workspace root Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<Package>, PublishError>` - Ordered packages, or an
///   error when the members order violates a local dependency.
pub async fn resolve_publish_order(manifest_path: &str) -> Result<Vec<Package>, PublishError> {
    let workspace_manifest: &Path = Path::new(manifest_path);
    let (mut packages, has_root_package) = discover_packages(workspace_manifest).await?;
    if has_root_package {
        position_root_package(&mut packages)?;
    }
    validate_publish_order(&packages)?;
    Ok(packages)
}

/// Check whether `cargo publish` stderr indicates the package version is
/// already present on the registry (a success case for idempotent
/// re-runs).
///
/// A version that is already live is not a failure: the artifact the caller
/// asked for is on the registry, so the release has achieved what it came
/// for. Republishing a live version is refused by the registry rather than
/// allowed to overwrite it, which makes a re-run fail on the 24 crates that
/// shipped before the rate limit refused the remaining three. Treating the
/// refusal as success is what lets a re-run converge on the whole workspace
/// instead of stalling on work already done.
///
/// # Arguments
///
/// - `&str` - cargo publish stderr output
///
/// # Returns
///
/// - `bool` - True when the output reports the version is already published
pub fn is_already_published(stderr: &str) -> bool {
    stderr.contains(STDERR_ALREADY_EXISTS_ON)
        || stderr.contains(STDERR_ALREADY_BEEN_UPLOADED)
        || stderr.contains(STDERR_IS_ALREADY_PUBLISHED)
}

/// Check whether `cargo publish` stderr reports a registry rate-limit
/// refusal rather than a fault that retrying cannot fix.
///
/// crates.io meters **new** crate names on its own schedule: a small burst
/// allowance, then roughly one name per ten minutes. Once a workspace
/// publishes more new names than the burst allows, every further name is
/// refused with a deadline embedded in the message. That deadline is
/// already longer than any exponential backoff, so a retry that ignores it
/// only spends the budget and fails again — this is the class of failure
/// that must wait out the deadline the registry handed back.
///
/// # Arguments
///
/// - `&str` - cargo publish stderr output
///
/// # Returns
///
/// - `bool` - True when the output reports a registry rate limit
pub fn is_rate_limited(stderr: &str) -> bool {
    stderr.contains(STDERR_TOO_MANY_REQUESTS) || stderr.contains(STDERR_TOO_MANY_NEW_CRATES)
}

/// Convert a civil date to a day count since 1970-01-01.
///
/// Counts from 0000-03-01 so that a leap day lands at the end of a year
/// and the month positions become a fixed-length stride.
///
/// # Arguments
///
/// - `i64` - Proleptic Gregorian year
/// - `i64` - Month, 1 through 12
/// - `i64` - Day of month
///
/// # Returns
///
/// - `i64` - Days since the Unix epoch
fn civil_to_days(year: i64, month: i64, day: i64) -> i64 {
    let shifted_year: i64 = if month <= 2 { year - 1 } else { year };
    let era: i64 = shifted_year.div_euclid(ERA_YEARS);
    let year_of_era: i64 = shifted_year.rem_euclid(ERA_YEARS);
    let month_position: i64 = (month + 9) % 12;
    let day_of_year: i64 =
        (MONTH_POSITION_SCALE * month_position + MONTH_POSITION_ROUNDING) / MONTH_POSITION_DIVISOR;
    let day_of_era: i64 = year_of_era * DAYS_PER_YEAR + year_of_era.div_euclid(LEAP_CYCLE_YEARS)
        - year_of_era.div_euclid(CENTURY_YEARS)
        + day_of_year
        + day
        - 1;
    era * ERA_DAYS + day_of_era - CIVIL_EPOCH_OFFSET_DAYS
}

/// Parse the RFC 2822 retry deadline crates.io embeds in a refusal.
///
/// The message reads `Please try again after Tue, 29 Sep 2026 04:55:57 GMT`,
/// where the day-of-week prefix and the `GMT` suffix are noise. A timestamp
/// that does not match that shape yields `None` so the caller falls back to
/// the conservative floor instead of guessing a shorter wait.
///
/// # Arguments
///
/// - `&str` - cargo publish stderr output
///
/// # Returns
///
/// - `Option<u64>` - Seconds to wait, or `None` when no deadline is present
pub fn parse_rate_limit_wait_secs(stderr: &str) -> Option<u64> {
    let start: usize = stderr.find(STDERR_TRY_AGAIN_AFTER)? + STDERR_TRY_AGAIN_AFTER.len();
    let end: usize = start + stderr[start..].find(STDERR_TRY_AGAIN_AFTER_END)?;
    let timestamp: &str = stderr[start..end].trim();
    let mut tokens: SplitWhitespace<'_> = timestamp.split_whitespace();
    let _: &str = tokens.next()?;
    let day: i64 = tokens
        .next()?
        .trim_end_matches(DAY_FIELD_SUFFIX)
        .parse()
        .ok()?;
    let month_token: &str = tokens.next()?;
    let month: i64 = MONTH_ABBREVIATIONS
        .iter()
        .position(|name: &&str| *name == month_token)? as i64
        + 1;
    let year: i64 = tokens.next()?.parse().ok()?;
    let mut clock_fields: Split<'_, char> = tokens.next()?.split(CLOCK_FIELD_SEPARATOR);
    let hour: i64 = clock_fields.next()?.parse().ok()?;
    let minute: i64 = clock_fields.next()?.parse().ok()?;
    let second: i64 = clock_fields.next()?.parse().ok()?;
    let day_start: i64 = civil_to_days(year, month, day) * SECONDS_PER_DAY;
    let deadline: i64 = day_start + hour * SECONDS_PER_HOUR + minute * SECONDS_PER_MINUTE + second;
    let now: i64 = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
    let remaining: i64 = deadline - now;
    // A parsed deadline is honoured as given, in either direction: a window
    // the registry says is shorter than the skew allowance is already open
    // on arrival, and a guess would only extend a wait it priced precisely.
    // The floor is for the unreadable message, handled by the caller.
    let bounded: i64 = remaining.clamp(0, RATE_LIMIT_MAX_WAIT_SECS as i64);
    Some(bounded as u64 + RATE_LIMIT_SKEW_SECS)
}

/// Seconds to wait before retrying a refused publish.
///
/// # Arguments
///
/// - `&str` - cargo publish stderr output
///
/// # Returns
///
/// - `u64` - Wait bounded by the registry deadline when it carried one
fn rate_limit_wait_secs(stderr: &str) -> u64 {
    parse_rate_limit_wait_secs(stderr).unwrap_or(RATE_LIMIT_FLOOR_SECS + RATE_LIMIT_SKEW_SECS)
}

/// Publish a single package with retry logic
///
/// A registry rate limit is not a fault: the registry named a deadline, and
/// retrying before it only spends the budget to be refused again. Those
/// refusals wait out the deadline the registry itself reported and are not
/// charged against `max_retries`, because the budget exists to survive
/// transient errors — a metered limit is neither transient nor shorter than
/// any backoff this loop could choose. Every other failure keeps the original
/// exponential backoff and still counts.
///
/// # Arguments
///
/// - `&Package` - Package to publish
/// - `u32` - Maximum retry attempts
///
/// # Returns
///
/// - `PublishResult` - Result with success status and retry count
async fn publish_package_with_retry(package: &Package, max_retries: u32) -> PublishResult {
    let mut attempt: u32 = 0;
    let mut last_error: String;
    let mut rate_limited_attempts: u32 = 0;
    loop {
        match publish_single_package(package).await {
            Ok(()) => {
                return PublishResult {
                    package_name: package.name.clone(),
                    success: true,
                    error: None,
                    retries: attempt + rate_limited_attempts,
                };
            }
            Err(error) => {
                let stderr: String = error.to_string();
                if is_rate_limited(&stderr) && rate_limited_attempts < RATE_LIMIT_MAX_WAITS {
                    let wait: u64 = rate_limit_wait_secs(&stderr);
                    log::info!(
                        "{}: rate limited by the registry, waiting {}s for the deadline it reported",
                        package.name,
                        wait
                    );
                    rate_limited_attempts += 1;
                    sleep(Duration::from_secs(wait)).await;
                    continue;
                }
                attempt += 1;
                last_error = stderr;
                if attempt <= max_retries {
                    sleep(Duration::from_secs(2_u64.pow(attempt))).await;
                    continue;
                }
                break;
            }
        }
    }
    PublishResult {
        package_name: package.name.clone(),
        success: false,
        error: Some(last_error),
        retries: attempt + rate_limited_attempts,
    }
}

/// Execute cargo publish command for a single package
///
/// # Arguments
///
/// - `&Package` - Package to publish
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Success or error
async fn publish_single_package(package: &Package) -> Result<(), Box<dyn Error>> {
    let output: Output = Command::new(CARGO)
        .arg(CARGO_PUBLISH)
        .arg(CLI_FLAG_ALLOW_DIRTY)
        .arg(CLI_FLAG_NO_VERIFY)
        .current_dir(&package.path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    if output.status.success() {
        return Ok(());
    }
    let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
    if is_already_published(&stderr) {
        log::info!("{} is already published, treating as success", package.name);
        return Ok(());
    }
    Err(stderr.into())
}

/// Execute publish command for all packages in workspace
///
/// Publishes in `[workspace.members]` declaration order with the root
/// package (if any) last, after validating the order against local
/// dependency constraints.
///
/// # Arguments
///
/// - `&str` - Path to workspace Cargo.toml
/// - `u32` - Maximum retry attempts per package
///
/// # Returns
///
/// - `Result<Vec<PublishResult>, PublishError>` - Results for all packages
pub async fn execute_publish(
    manifest_path: &str,
    max_retries: u32,
) -> Result<Vec<PublishResult>, PublishError> {
    let path: &Path = Path::new(manifest_path);
    let path: &Path = match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    };
    let workspace_manifest: PathBuf = path.join(CARGO_TOML);
    let sync_report: SyncReport =
        match execute_sync(workspace_manifest.to_str().unwrap_or(CARGO_TOML)).await {
            Ok(report) => report,
            Err(error) => return Err(PublishError::SyncFailed(error)),
        };
    if sync_report.file_changed {
        log::info!(
            "publish: synced workspace dependencies ({} renamed, {} versioned) to v{}",
            sync_report.renamed_entries.len(),
            sync_report.versioned_entries.len(),
            sync_report.workspace_version,
        );
    }
    let ordered_packages: Vec<Package> =
        resolve_publish_order(workspace_manifest.to_str().unwrap_or(CARGO_TOML)).await?;
    if ordered_packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut results: Vec<PublishResult> = Vec::new();
    for package in ordered_packages {
        if !package.publish {
            log::info!("Skipping {} (publish = false)", package.name);
            continue;
        }
        log::info!("Publishing {} v{}...", package.name, package.version);
        let result: PublishResult = publish_package_with_retry(&package, max_retries).await;
        if result.success {
            if result.retries == 0 {
                log::info!("Successfully published {}", result.package_name,);
            } else {
                log::info!(
                    "Successfully published {} (retried {} times)",
                    result.package_name,
                    result.retries
                );
            }
        } else if let Some(error) = &result.error {
            log::error!("Failed to publish {}: {error}", result.package_name);
        } else {
            log::error!("Failed to publish {}", result.package_name);
        }
        results.push(result);
    }
    Ok(results)
}

use super::*;

/// Parse a version string into Version struct
///
/// # Arguments
///
/// - `&str`: The version string to parse (e.g., "0.1.0" or "0.1.0-alpha")
///
/// # Returns
///
/// - `Option<Version>`: Parsed version if successful, None otherwise
fn parse_version(version_str: &str) -> Option<Version> {
    let parts: Vec<&str> = version_str.split('-').collect();
    let version_part: &str = parts.first()?;
    let prerelease: Option<String> = parts.get(1).map(|s: &&str| s.to_string());
    let nums: Vec<&str> = version_part.split('.').collect();
    if nums.len() != 3 {
        return None;
    }
    let major: u64 = nums.first()?.parse().ok()?;
    let minor: u64 = nums.get(1)?.parse().ok()?;
    let patch: u64 = nums.get(2)?.parse().ok()?;
    Some(Version {
        major,
        minor,
        patch,
        prerelease,
    })
}

/// Parse pre-release identifier to extract type and number
///
/// # Arguments
///
/// - `&str`: The pre-release string (e.g., "alpha", "alpha.1", "beta.2")
///
/// # Returns
///
/// - `Option<(&str, u64)>`: Tuple of (pre_release_type, number) if parsed successfully
fn parse_prerelease(prerelease: &str) -> Option<(&str, u64)> {
    let parts: Vec<&str> = prerelease.split('.').collect();
    let pre_type: &str = parts.first()?;
    let number: u64 = parts
        .get(1)
        .and_then(|s: &&str| s.parse().ok())
        .unwrap_or(0);
    Some((pre_type, number))
}

/// Get the next pre-release version string
///
/// # Arguments
///
/// - `Option<&String>`: Current pre-release identifier
/// - `&str`: Target pre-release type ("alpha", "beta", "rc")
///
/// # Returns
///
/// - `String`: The new pre-release identifier
fn get_next_prerelease(current: Option<&String>, target_type: &str) -> String {
    match current {
        Some(pre) => {
            if let Some((pre_type, number)) = parse_prerelease(pre)
                && pre_type == target_type
                && number > 0
            {
                return format!("{}.{}", target_type, number + 1);
            }
            format!("{target_type}.1")
        }
        None => target_type.to_string(),
    }
}

/// Convert Version back to string representation
///
/// # Arguments
///
/// - `&Version`: The Version struct to convert
///
/// # Returns
///
/// - `String`: Version string (e.g., "0.1.0" or "0.1.0-alpha")
fn version_to_string(version: &Version) -> String {
    let base: String = format!("{}.{}.{}", version.major, version.minor, version.patch);
    match &version.prerelease {
        Some(pre) => format!("{base}-{pre}"),
        None => base,
    }
}

/// Apply version bump according to the specified type
///
/// # Arguments
///
/// - `&Version`: The current version
/// - `&BumpVersionType`: The type of version bump to apply
///
/// # Returns
///
/// - `Version`: The new version after bumping
fn bump_version(version: &Version, bump_type: &BumpVersionType) -> Version {
    match bump_type {
        BumpVersionType::Patch => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch + 1,
            prerelease: None,
        },
        BumpVersionType::Minor => Version {
            major: version.major,
            minor: version.minor + 1,
            patch: 0,
            prerelease: None,
        },
        BumpVersionType::Major => Version {
            major: version.major + 1,
            minor: 0,
            patch: 0,
            prerelease: None,
        },
        BumpVersionType::Release => Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            prerelease: None,
        },
        BumpVersionType::Alpha => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "alpha");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
        BumpVersionType::Beta => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "beta");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
        BumpVersionType::Rc => {
            let prerelease: String = get_next_prerelease(version.prerelease.as_ref(), "rc");
            Version {
                major: version.major,
                minor: version.minor,
                patch: version.patch,
                prerelease: Some(prerelease),
            }
        }
    }
}

/// Parse, bump and re-render a version string in one step
///
/// # Arguments
///
/// - `&str`: Current version string (e.g., "0.1.0" or "0.1.0-alpha.1")
/// - `&BumpVersionType`: The type of version bump to apply
///
/// # Returns
///
/// - `Option<String>`: New version string, or None if parsing fails
fn bump_version_str(version_str: &str, bump_type: &BumpVersionType) -> Option<String> {
    let version: Version = parse_version(version_str)?;
    Some(version_to_string(&bump_version(&version, bump_type)))
}

/// Expand one `[workspace.members]` entry into member directories
///
/// Plain relative paths resolve against the workspace root; a trailing
/// `/*` glob expands to every immediate subdirectory that contains a
/// Cargo.toml, sorted by path for deterministic behavior.
///
/// # Arguments
///
/// - `&Path`: Workspace root directory
/// - `&str`: Raw members entry (e.g., "core" or "crates/*")
///
/// # Returns
///
/// - `Vec<PathBuf>`: Resolved member directories
fn expand_member_entry(root_dir: &Path, entry: &str) -> Vec<PathBuf> {
    match entry.strip_suffix("/*") {
        Some(prefix) => {
            let mut dirs: Vec<PathBuf> = Vec::new();
            if let Ok(entries) = std::fs::read_dir(root_dir.join(prefix)) {
                for entry in entries.flatten() {
                    let path: PathBuf = entry.path();
                    if path.is_dir() && path.join("Cargo.toml").exists() {
                        dirs.push(path);
                    }
                }
            }
            dirs.sort();
            dirs
        }
        None => vec![root_dir.join(entry)],
    }
}

/// Realign the `version` field of every local path dependency in a
/// dependency table with freshly bumped member versions
///
/// Only entries whose `path` resolves to a bumped member directory and
/// that already carry a `version` field are rewritten; path-only entries
/// (deliberate dev-dependency style) are left untouched.
///
/// # Arguments
///
/// - `&mut dyn TableLike`: Dependency table to scan
/// - `&Path`: Base directory that relative `path` values resolve against
/// - `&[(PathBuf, String)]`: Bumped members as (canonical dir, new version)
///
/// # Returns
///
/// - `bool`: True if at least one entry was rewritten
fn realign_dep_versions(
    deps: &mut dyn TableLike,
    base_dir: &Path,
    bumped: &[(PathBuf, String)],
) -> bool {
    let mut changed: bool = false;
    for (_alias, entry) in deps.iter_mut() {
        let Some(dep_path) = entry
            .get("path")
            .and_then(|path_item: &Item| path_item.as_str())
        else {
            continue;
        };
        let Ok(canonical) = base_dir.join(dep_path).canonicalize() else {
            continue;
        };
        let Some((_, new_version)) = bumped.iter().find(|(dir, _)| *dir == canonical) else {
            continue;
        };
        if entry.get("version").is_none() {
            continue;
        }
        let Some(version_slot) = entry.get_mut("version") else {
            continue;
        };
        if version_slot.as_str() == Some(new_version.as_str()) {
            continue;
        }
        set_item_string_preserving_decor(version_slot, new_version);
        changed = true;
    }
    changed
}

/// Realign member-to-member path dependency versions inside one member
/// manifest, across `[dependencies]` / `[dev-dependencies]` /
/// `[build-dependencies]` and their `[target.*]` variants
///
/// # Arguments
///
/// - `&Path`: Member directory that relative dep paths resolve against
/// - `&Path`: Member Cargo.toml path
/// - `&[(PathBuf, String)]`: Bumped members as (canonical dir, new version)
async fn realign_member_manifest(
    member_dir: &Path,
    member_manifest_path: &Path,
    bumped: &[(PathBuf, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let member_content: String = read_to_string(member_manifest_path).await?;
    let mut member_doc: DocumentMut = member_content.parse().map_err(|e: TomlError| {
        format!("failed to parse {}: {}", member_manifest_path.display(), e)
    })?;
    let mut changed: bool = false;
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(deps) = member_doc
            .get_mut(section)
            .and_then(|deps_item: &mut Item| deps_item.as_table_like_mut())
        {
            changed |= realign_dep_versions(deps, member_dir, bumped);
        }
    }
    if let Some(targets) = member_doc
        .get_mut("target")
        .and_then(|target_item: &mut Item| target_item.as_table_like_mut())
    {
        let target_keys: Vec<String> = targets.iter().map(|(key, _)| key.to_string()).collect();
        for target_key in target_keys {
            let Some(target_table) = targets
                .get_mut(&target_key)
                .and_then(|target_item: &mut Item| target_item.as_table_like_mut())
            else {
                continue;
            };
            for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(deps) = target_table
                    .get_mut(section)
                    .and_then(|deps_item: &mut Item| deps_item.as_table_like_mut())
                {
                    changed |= realign_dep_versions(deps, member_dir, bumped);
                }
            }
        }
    }
    if changed {
        write(member_manifest_path, member_doc.to_string()).await?;
    }
    Ok(())
}

/// Bump every workspace member's own `[package].version` and realign all
/// local path dependency versions with the results
///
/// Used for virtual-workspace repositories where members carry
/// independent versions (no `[workspace.package].version` and no root
/// `[package]`). Members using `version.workspace = true` are skipped.
///
/// # Arguments
///
/// - `&Path`: Workspace root Cargo.toml path
/// - `&mut DocumentMut`: Parsed root manifest, updated and written when a
///   `[workspace.dependencies]` entry needs realigning
/// - `&BumpVersionType`: Type of version bump to apply to each member
///
/// # Returns
///
/// - `Result<String, Box<dyn std::error::Error>>`: Summary string
async fn bump_workspace_members(
    root_path: &Path,
    doc: &mut DocumentMut,
    bump_type: &BumpVersionType,
) -> Result<String, Box<dyn std::error::Error>> {
    let root_dir: &Path = root_path.parent().unwrap_or_else(|| Path::new("."));
    let member_entries: Vec<String> = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("members"))
        .and_then(|members_item: &Item| members_item.as_array())
        .map(|members: &toml_edit::Array| {
            members
                .iter()
                .filter_map(|member: &TomlEditValue| member.as_str().map(|s: &str| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let mut member_dirs: Vec<PathBuf> = Vec::new();
    for entry in &member_entries {
        member_dirs.extend(expand_member_entry(root_dir, entry));
    }
    member_dirs.sort();
    member_dirs.dedup();
    let mut bumped: Vec<(PathBuf, String)> = Vec::new();
    for dir in &member_dirs {
        let member_manifest_path: PathBuf = dir.join("Cargo.toml");
        if !member_manifest_path.exists() {
            return Err(format!(
                "member manifest not found: {}",
                member_manifest_path.display()
            )
            .into());
        }
        let member_content: String = read_to_string(&member_manifest_path).await?;
        let mut member_doc: DocumentMut = member_content.parse().map_err(|e: TomlError| {
            format!("failed to parse {}: {}", member_manifest_path.display(), e)
        })?;
        let version_slot: &mut Item = member_doc
            .get_mut("package")
            .and_then(|package: &mut Item| package.get_mut("version"))
            .ok_or_else(|| -> Box<dyn std::error::Error> {
                format!(
                    "package.version not found in {}",
                    member_manifest_path.display()
                )
                .into()
            })?;
        let Some(old_version) = version_slot.as_str().map(|s: &str| s.to_string()) else {
            continue;
        };
        let new_version: String = bump_version_str(&old_version, bump_type).ok_or_else(
            || -> Box<dyn std::error::Error> {
                format!("failed to parse version: {}", old_version).into()
            },
        )?;
        set_item_string_preserving_decor(version_slot, &new_version);
        write(&member_manifest_path, member_doc.to_string()).await?;
        log::info!(
            "bump: {} -> {}",
            member_manifest_path.display(),
            new_version
        );
        bumped.push((dir.canonicalize()?, new_version));
    }
    if bumped.is_empty() {
        return Ok("0 workspace members".to_string());
    }
    let root_changed: bool = doc
        .get_mut("workspace")
        .and_then(|workspace: &mut Item| workspace.get_mut("dependencies"))
        .and_then(|deps_item: &mut Item| deps_item.as_table_like_mut())
        .is_some_and(|deps: &mut dyn TableLike| realign_dep_versions(deps, root_dir, &bumped));
    if root_changed {
        write(root_path, doc.to_string()).await?;
    }
    for dir in &member_dirs {
        realign_member_manifest(dir, &dir.join("Cargo.toml"), &bumped).await?;
    }
    Ok(format!("{} workspace members", bumped.len()))
}

/// Read and update version in Cargo.toml
///
/// Three repository architectures are supported:
///
/// - Shared-version monorepo (`[workspace.package].version` present): the
///   workspace root version is bumped; run `sync` to propagate it.
/// - Virtual workspace (no root `[package]`, members carry independent
///   versions): every member's own version is bumped and local path
///   dependency versions are realigned in one pass.
/// - Non-monorepo single crate (plain `[package].version`): the crate
///   version is bumped.
///
/// The edit is applied on a `toml_edit::DocumentMut`, so comments, key
/// order and inline-table formatting outside the version value are
/// preserved byte-for-byte.
///
/// # Arguments
///
/// - `&str`: Path to Cargo.toml file
/// - `&BumpVersionType`: Type of version bump to apply
///
/// # Returns
///
/// - `Result<String, Box<dyn std::error::Error>>`: The new version string, a
///   workspace bump summary, or an error
pub async fn execute_bump(
    manifest_path: &str,
    bump_type: &BumpVersionType,
) -> Result<String, Box<dyn std::error::Error>> {
    let path: &Path = Path::new(manifest_path);
    let content: String = read_to_string(path).await?;
    let mut doc: DocumentMut = content
        .parse()
        .map_err(|e: TomlError| format!("failed to parse {}: {}", manifest_path, e))?;
    let has_workspace_version: bool = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("package"))
        .and_then(|package: &Item| package.get("version"))
        .is_some();
    let has_root_package: bool = doc.get("package").is_some();
    let has_members: bool = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("members"))
        .and_then(|members_item: &Item| members_item.as_array())
        .is_some_and(|members: &toml_edit::Array| !members.is_empty());
    if !has_workspace_version && !has_root_package && has_members {
        return bump_workspace_members(path, &mut doc, bump_type).await;
    }
    let version_slot: &mut Item = if has_workspace_version {
        doc.get_mut("workspace")
            .and_then(|workspace: &mut Item| workspace.get_mut("package"))
            .and_then(|package: &mut Item| package.get_mut("version"))
            .ok_or_else(|| -> Box<dyn std::error::Error> {
                "workspace.package.version not found".into()
            })?
    } else if has_root_package {
        doc.get_mut("package")
            .and_then(|package: &mut Item| package.get_mut("version"))
            .ok_or_else(|| -> Box<dyn std::error::Error> { "package.version not found".into() })?
    } else {
        return Err("neither [package] nor [workspace.package] found in Cargo.toml".into());
    };
    let version_str: String = version_slot
        .as_str()
        .ok_or_else(|| -> Box<dyn std::error::Error> { "version field is not a string".into() })?
        .to_string();
    let version: Version =
        parse_version(&version_str).ok_or_else(|| -> Box<dyn std::error::Error> {
            format!("failed to parse version: {}", version_str).into()
        })?;
    let bumped: Version = bump_version(&version, bump_type);
    let version_string: String = version_to_string(&bumped);
    set_item_string_preserving_decor(version_slot, &version_string);
    write(path, doc.to_string()).await?;
    Ok(version_string)
}

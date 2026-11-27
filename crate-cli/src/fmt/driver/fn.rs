use super::*;

/// Resolves the unified formatting strategy for `path` based on filename and extension.
///
/// # Arguments
///
/// - `&Path` - File path to classify.
///
/// # Returns
///
/// - `FileFormatKind` - Matched formatting strategy or `FileFormatKind::Unsupported`.
pub fn detect_format_kind(path: &Path) -> FileFormatKind {
    if let Some(name) = path.file_name().and_then(OsStr::to_str) {
        if FMT_NAME_PRESERVE_TABS.contains(&name) {
            return FileFormatKind::PlainTextPreserveTabs;
        }
        if FMT_NAME_PLAIN_TEXT.contains(&name) {
            return FileFormatKind::PlainText;
        }
    }
    let ext_lower: String = match path.extension().and_then(OsStr::to_str) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return FileFormatKind::Unsupported,
    };
    let ext: &str = ext_lower.as_str();
    if ext == FMT_EXT_RUST {
        return FileFormatKind::Rust;
    }
    if FMT_EXT_MARKDOWN.contains(&ext) {
        return FileFormatKind::Markdown;
    }
    if FMT_EXT_YAML.contains(&ext) {
        return FileFormatKind::Yaml;
    }
    if FMT_EXT_JSON.contains(&ext) {
        return FileFormatKind::Json;
    }
    if FMT_EXT_JSONC.contains(&ext) {
        return FileFormatKind::Jsonc;
    }
    if FMT_EXT_TOML.contains(&ext) {
        return FileFormatKind::Toml;
    }
    if FMT_EXT_PLAIN_TEXT.contains(&ext) {
        return FileFormatKind::PlainText;
    }
    FileFormatKind::Unsupported
}

/// Formats text content in memory according to the file type inferred from `path`.
///
/// # Arguments
///
/// - `&Path` - File path used to determine the target formatter.
/// - `&str` - Raw UTF-8 file content.
///
/// # Returns
///
/// - `String` - Formatted text content (or unchanged input when unsupported).
pub fn format_text_by_path(path: &Path, input: &str) -> String {
    match detect_format_kind(path) {
        FileFormatKind::Markdown => format_markdown(input),
        FileFormatKind::Yaml => format_yaml(input),
        FileFormatKind::Json => {
            format_json(input).unwrap_or_else(|_: io::Error| format_jsonc(input))
        }
        FileFormatKind::Jsonc => format_jsonc(input),
        FileFormatKind::Toml => format_toml(input),
        FileFormatKind::PlainText => format_plain_text(input, false),
        FileFormatKind::PlainTextPreserveTabs => format_plain_text(input, true),
        FileFormatKind::Rust | FileFormatKind::Unsupported => input.to_string(),
    }
}

/// Recursively discovers and formats all supported non-Rust text files under `root` (or a single file).
///
/// # Arguments
///
/// - `&Path` - Root workspace directory or individual file path.
/// - `bool` - When `true`, verifies formatting without modifying disk and returns an error if changes are needed.
///
/// # Returns
///
/// - `Result<UnifiedFmtReport, io::Error>` - Structured report of scanned and formatted files.
pub fn format_workspace_text_files(
    root: &Path,
    check: bool,
) -> Result<UnifiedFmtReport, io::Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_text_files(root, true, &mut files)?;
    files.sort();
    let mut report: UnifiedFmtReport = UnifiedFmtReport::new(files.len(), 0, check, Vec::new());
    for file_path in files {
        let original: String = match fs::read_to_string(&file_path) {
            Ok(text) => text,
            Err(_) => continue,
        };
        let formatted: String = format_text_by_path(&file_path, &original);
        if formatted != original {
            let next_count: usize = report.get_files_formatted() + 1;
            let _: &mut UnifiedFmtReport = report.set_files_formatted(next_count);
            report
                .get_mut_changed_files()
                .push(file_path.display().to_string());
            if !check {
                fs::write(&file_path, formatted)?;
            }
        }
    }
    if check && report.get_files_formatted() > 0 {
        return Err(io::Error::other(format!(
            "{ERROR_TEXT_FMT_CHECK_FAILED}: {}",
            report.get_changed_files().join(", ")
        )));
    }
    Ok(report)
}

/// Executes the unified text and Rust formatting pipeline (all text files, derive sorting, and `cargo fmt`) without clippy-fix.
///
/// Designed as the shared first pass called by both `crate fmt` and framework CLIs such as `euv fmt` and `hyperlane fmt`.
///
/// # Arguments
///
/// - `&Path` - Workspace root directory or Cargo.toml manifest path.
/// - `bool` - When `true`, checks formatting without writing files.
///
/// # Returns
///
/// - `Result<UnifiedFmtReport, io::Error>` - Unified text formatting report or error.
pub async fn execute_unified_fmt(root: &Path, check: bool) -> Result<UnifiedFmtReport, io::Error> {
    let (workspace_root, manifest_path): (PathBuf, PathBuf) = if root.is_file() {
        let parent: PathBuf = root
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));
        (parent, root.to_path_buf())
    } else {
        (root.to_path_buf(), root.join(CARGO_TOML))
    };
    let report: UnifiedFmtReport = format_workspace_text_files(&workspace_root, check)?;
    if manifest_path.exists() {
        let manifest_str: String = manifest_path.display().to_string();
        if !check {
            format_derive_attributes(&manifest_str).await?;
        }
        run_cargo_fmt(Some(&manifest_str), check).await?;
    }
    Ok(report)
}

/// Recursively collects all supported non-Rust text files beneath `path`.
///
/// # Arguments
///
/// - `&Path` - Directory or single file path.
/// - `bool` - Whether `path` is the top-level target (never skipped by directory name filters).
/// - `&mut Vec<PathBuf>` - Output accumulator.
///
/// # Returns
///
/// - `Result<(), io::Error>` - Traversal status.
fn collect_text_files(path: &Path, is_root: bool, out: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    if path.is_file() {
        let kind: FileFormatKind = detect_format_kind(path);
        if !matches!(kind, FileFormatKind::Rust | FileFormatKind::Unsupported) {
            out.push(path.to_path_buf());
        }
        return Ok(());
    }
    if !path.is_dir() {
        return Ok(());
    }
    if !is_root
        && let Some(name) = path.file_name().and_then(OsStr::to_str)
        && FMT_SKIP_DIRS.contains(&name)
    {
        return Ok(());
    }
    for entry_res in fs::read_dir(path)? {
        let entry: DirEntry = entry_res?;
        let child: PathBuf = entry.path();
        if child.is_dir() {
            collect_text_files(&child, false, out)?;
        } else if child.is_file() {
            let kind: FileFormatKind = detect_format_kind(&child);
            if !matches!(kind, FileFormatKind::Rust | FileFormatKind::Unsupported) {
                out.push(child);
            }
        }
    }
    Ok(())
}

/// Sort derive traits in a single line
///
/// # Arguments
///
/// - `&str` - The line containing derive attribute
///
/// # Returns
///
/// - `Option<String>` - Sorted line if derive found, None otherwise
fn sort_derive_in_line(line: &str) -> Option<String> {
    let captures: Captures<'_> = DERIVE_REGEX.captures(line)?;
    let derive_content: &str = captures.get(1)?.as_str();
    let mut traits: Vec<String> = derive_content
        .split(',')
        .map(|s: &str| s.trim().to_string())
        .filter(|s: &String| !s.is_empty())
        .collect();
    traits.sort_by_key(|a: &String| a.to_lowercase());
    let sorted_traits: String = traits.join(", ");
    let result: String = line.replace(derive_content, &sorted_traits);
    Some(result)
}

/// Format derive attributes in a file
///
/// # Arguments
///
/// - `&Path` - Path to the Rust file
///
/// # Returns
///
/// - `Result<bool, io::Error>` - True if file was modified, false otherwise
async fn format_derive_in_file(file_path: &Path) -> Result<bool, io::Error> {
    let content: String = read_to_string(file_path).await?;
    let lines: Lines<'_> = content.lines();
    let mut modified: bool = false;
    let mut new_content: String = String::new();
    for line in lines {
        let trimmed: &str = line.trim();
        let new_line: String = if trimmed.starts_with(DERIVE_PREFIX) {
            if let Some(sorted) = sort_derive_in_line(line) {
                if sorted != line {
                    modified = true;
                }
                sorted
            } else {
                line.to_string()
            }
        } else {
            line.to_string()
        };
        new_content.push_str(&new_line);
        new_content.push('\n');
    }
    if modified {
        write(file_path, new_content).await?;
    }
    Ok(modified)
}

/// Find all Rust files in workspace
///
/// # Arguments
///
/// - `&Path` - Path to Cargo.toml
///
/// # Returns
///
/// - `Result<Vec<PathBuf>, io::Error>` - List of Rust file paths
async fn find_rust_files(manifest_path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    let mut files: Vec<PathBuf> = Vec::new();
    let workspace_root: &Path = manifest_path.parent().unwrap_or(Path::new("."));
    let src_dir: PathBuf = workspace_root.join("src");
    if src_dir.exists() {
        find_rust_files_in_dir(&src_dir, &mut files).await?;
    }
    let content: String = read_to_string(manifest_path).await?;
    if let Ok(doc) = from_str::<Value>(&content)
        && let Some(workspace) = doc.get(TOML_WORKSPACE)
        && let Some(members) = workspace
            .get(TOML_MEMBERS)
            .and_then(|m: &Value| m.as_array())
    {
        for member in members {
            if let Some(pattern) = member.as_str() {
                let member_src: PathBuf = workspace_root.join(pattern).join("src");
                if member_src.exists() {
                    find_rust_files_in_dir(&member_src, &mut files).await?;
                }
            }
        }
    }
    Ok(files)
}

/// Recursively find Rust files in directory
///
/// # Arguments
///
/// - `&Path` - Directory to search
/// - `&mut Vec<PathBuf>` - Vector to collect file paths
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
async fn find_rust_files_in_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    let mut entries: ReadDir = read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path: PathBuf = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|ext: &OsStr| ext == FMT_EXT_RUST)
        {
            files.push(path);
        } else if path.is_dir() {
            Box::pin(find_rust_files_in_dir(&path, files)).await?;
        }
    }
    Ok(())
}

/// Format derive attributes in all workspace files
///
/// # Arguments
///
/// - `&str` - Path to Cargo.toml
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
async fn format_derive_attributes(manifest_path: &str) -> Result<(), io::Error> {
    let path: &Path = Path::new(manifest_path);
    let files: Vec<PathBuf> = find_rust_files(path).await?;
    let modified_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<Result<(), io::Error>>> = Vec::new();
    for file in files {
        let counter: Arc<Mutex<usize>> = Arc::clone(&modified_count);
        let handle: JoinHandle<Result<(), io::Error>> = spawn(async move {
            if format_derive_in_file(&file).await? {
                let mut count: MutexGuard<'_, usize> = counter.lock().await;
                *count += 1;
            }
            Ok(())
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await??;
    }
    let count: usize = *modified_count.lock().await;
    if count > 0 {
        log::info!("Sorted derive attributes in {count} files");
    }
    Ok(())
}

/// Runs `cargo fmt` with optional manifest path and `--check` flag.
///
/// # Arguments
///
/// - `Option<&str>` - Optional `--manifest-path` argument.
/// - `bool` - Whether to pass `--check`.
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error if `cargo fmt` fails.
async fn run_cargo_fmt(manifest_path: Option<&str>, check: bool) -> Result<(), io::Error> {
    let mut cmd: Command = Command::new(CARGO);
    cmd.arg(CARGO_FMT);
    if check {
        cmd.arg(CLI_FLAG_CHECK);
    }
    if let Some(manifest) = manifest_path {
        cmd.arg(CLI_FLAG_MANIFEST_PATH).arg(manifest);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output: Output = cmd.output().await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other(ERROR_FMT_FAILED));
    }
    Ok(())
}

/// Check if cargo-clippy is installed
///
/// # Returns
///
/// - `bool` - True if cargo-clippy is available
fn is_cargo_clippy_installed() -> bool {
    which(CARGO_CLIPPY).is_ok()
}

/// Install cargo-clippy using rustup
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
async fn install_cargo_clippy() -> Result<(), io::Error> {
    log::warn!("cargo-clippy not found, installing...");
    let output: Output = Command::new(RUSTUP)
        .arg(RUSTUP_COMPONENT)
        .arg("add")
        .arg(CLIPPY)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other(ERROR_CLIPPY_INSTALL_FAILED));
    }
    Ok(())
}

/// Execute clippy fix command
///
/// # Arguments
///
/// - `&Args` - The parsed arguments
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
async fn execute_clippy_fix(args: &Args) -> Result<(), io::Error> {
    if !is_cargo_clippy_installed() {
        install_cargo_clippy().await?;
    }
    let mut cmd: Command = Command::new(CARGO);
    cmd.arg(CLIPPY)
        .arg(CLI_FLAG_FIX)
        .arg(CLI_FLAG_WORKSPACE)
        .arg(CLI_FLAG_ALL_TARGETS)
        .arg(CLI_FLAG_ALLOW_DIRTY);
    if let Some(ref manifest_path) = args.manifest_path {
        cmd.arg(CLI_FLAG_MANIFEST_PATH).arg(manifest_path);
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let output: Output = cmd.output().await?;
    let stdout: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr: String = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !stdout.is_empty() {
        for line in stdout.lines() {
            log::info!("{line}");
        }
    }
    if !stderr.is_empty() {
        if output.status.success() {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::info!("{line}");
            }
        } else {
            for line in stderr.lines() {
                if line.is_empty() {
                    continue;
                }
                log::error!("{line}");
            }
        }
    }
    if !output.status.success() {
        return Err(io::Error::other(ERROR_CLIPPY_FIX_FAILED));
    }
    Ok(())
}

/// Execute fmt command across all workspace text files and Rust sources.
///
/// # Arguments
///
/// - `&Args` - The parsed arguments
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
pub async fn execute_fmt(args: &Args) -> Result<(), io::Error> {
    let manifest_path: String = args
        .manifest_path
        .clone()
        .unwrap_or_else(|| CARGO_TOML.to_string());
    let manifest: &Path = Path::new(&manifest_path);
    let workspace_root: &Path = manifest
        .parent()
        .filter(|p: &&Path| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let report: UnifiedFmtReport = format_workspace_text_files(workspace_root, args.check)?;
    if report.get_files_formatted() > 0 {
        log::info!(
            "Formatted {} text files (scanned {})",
            report.get_files_formatted(),
            report.get_files_scanned()
        );
    }
    if !args.check {
        format_derive_attributes(&manifest_path).await?;
    }
    run_cargo_fmt(args.manifest_path.as_deref(), args.check).await?;
    if !args.check {
        execute_clippy_fix(args).await?;
    }
    Ok(())
}

/// Format code or supported text files at a specific path.
///
/// # Arguments
///
/// - `&Path` - Path to file or directory to format
///
/// # Returns
///
/// - `Result<(), io::Error>` - Success or error
pub async fn format_path(path: &Path) -> Result<(), io::Error> {
    let kind: FileFormatKind = detect_format_kind(path);
    if path.is_file() && !matches!(kind, FileFormatKind::Rust | FileFormatKind::Unsupported) {
        let _: UnifiedFmtReport = format_workspace_text_files(path, false)?;
        return Ok(());
    }
    if path.is_dir() {
        let _: UnifiedFmtReport = format_workspace_text_files(path, false)?;
    }
    let mut cmd: Command = Command::new(CARGO);
    cmd.arg(CARGO_FMT).arg("--").arg(path);
    cmd.stdout(Stdio::null()).stderr(Stdio::null());
    let _: ExitStatus = cmd.status().await?;
    Ok(())
}

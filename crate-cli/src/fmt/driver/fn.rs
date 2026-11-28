use super::*;

/// Resolves the unified formatting strategy for `path` based on filename and extension matching VS Code `settings.json`.
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
        if FMT_NAME_JSON_STRINGIFY.contains(&name) {
            return FileFormatKind::JsonStringify;
        }
        if FMT_NAME_YAML.contains(&name) {
            return FileFormatKind::Yaml;
        }
        if FMT_NAME_PRESERVE_TABS.contains(&name) {
            return FileFormatKind::PlainTextPreserveTabs;
        }
        if FMT_NAME_PLAIN_TEXT.contains(&name)
            || name.starts_with(FMT_PREFIX_DOCKERFILE)
            || name.starts_with(FMT_PREFIX_DOTENV)
        {
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
    if FMT_EXT_JSONC.contains(&ext) {
        return FileFormatKind::Jsonc;
    }
    if FMT_EXT_JSON.contains(&ext) {
        return FileFormatKind::Json;
    }
    if FMT_EXT_TOML.contains(&ext) {
        return FileFormatKind::Toml;
    }
    if FMT_EXT_JAVASCRIPT.contains(&ext) {
        return FileFormatKind::JavaScript;
    }
    if FMT_EXT_TYPESCRIPT.contains(&ext) {
        return FileFormatKind::TypeScript;
    }
    if FMT_EXT_TSX.contains(&ext) {
        return FileFormatKind::TypeScriptReact;
    }
    if FMT_EXT_CSS.contains(&ext) {
        return FileFormatKind::Css;
    }
    if FMT_EXT_SCSS.contains(&ext) {
        return FileFormatKind::Scss;
    }
    if FMT_EXT_LESS.contains(&ext) {
        return FileFormatKind::Less;
    }
    if FMT_EXT_HTML.contains(&ext) {
        return FileFormatKind::Html;
    }
    if FMT_EXT_VUE.contains(&ext) {
        return FileFormatKind::Vue;
    }
    if FMT_EXT_PLAIN_TEXT.contains(&ext) {
        return FileFormatKind::PlainText;
    }
    FileFormatKind::Unsupported
}

/// Formats text content in memory according to the file type inferred from `path` matching VS Code save behavior.
///
/// # Arguments
///
/// - `&Path` - File path used to determine the target formatter.
/// - `&str` - Raw UTF-8 file content.
///
/// # Returns
///
/// - `String` - Formatted text content (or unchanged input when no-op, unsupported, or syntactically invalid).
pub fn format_text_by_path(path: &Path, input: &str) -> String {
    match detect_format_kind(path) {
        FileFormatKind::Markdown => format_markdown(input),
        FileFormatKind::Yaml => format_yaml_with_path(input, path),
        FileFormatKind::Json => format_json(input).unwrap_or_else(|_: io::Error| input.to_string()),
        FileFormatKind::JsonStringify => {
            format_json_stringify(input).unwrap_or_else(|_: io::Error| input.to_string())
        }
        FileFormatKind::Jsonc => format_jsonc(input),
        FileFormatKind::Toml => format_toml(input),
        FileFormatKind::JavaScript => format_javascript_with_path(input, path),
        FileFormatKind::TypeScript => format_typescript_with_path(input, path),
        FileFormatKind::TypeScriptReact => format_tsx_with_path(input, path),
        FileFormatKind::Css => format_css(input),
        FileFormatKind::Scss => format_scss(input),
        FileFormatKind::Less => format_less(input),
        FileFormatKind::Html => format_html_with_path(input, path),
        FileFormatKind::Vue => format_vue_with_path(input, path),
        FileFormatKind::PlainText
        | FileFormatKind::PlainTextPreserveTabs
        | FileFormatKind::Rust
        | FileFormatKind::Unsupported => input.to_string(),
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
    let ignore_rules: Vec<PrettierIgnoreRule> = if root.is_dir() {
        load_prettier_ignore(root)
    } else {
        Vec::new()
    };
    let mut files: Vec<PathBuf> = Vec::new();
    collect_text_files(root, root, true, &ignore_rules, &mut files)?;
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
            .filter(|p: &&Path| !p.as_os_str().is_empty())
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

/// Returns `true` when `kind` has an active non-Rust formatter in VS Code and should be scanned and formatted.
///
/// # Arguments
///
/// - `FileFormatKind` - Classified file format kind.
///
/// # Returns
///
/// - `bool` - Whether the file is actively formatted by `format_workspace_text_files`.
fn is_active_text_format(kind: FileFormatKind) -> bool {
    !matches!(
        kind,
        FileFormatKind::Rust
            | FileFormatKind::PlainText
            | FileFormatKind::PlainTextPreserveTabs
            | FileFormatKind::Unsupported
    )
}

/// Recursively collects all actively formatted non-Rust text files beneath `path` without following symlinks.
///
/// # Arguments
///
/// - `&Path` - Workspace root used to compute relative paths for `.prettierignore`.
/// - `&Path` - Current directory or single file path.
/// - `bool` - Whether `path` is the top-level target (never skipped by directory name filters).
/// - `&[PrettierIgnoreRule]` - Parsed `.prettierignore` rules from the workspace root.
/// - `&mut Vec<PathBuf>` - Output accumulator.
///
/// # Returns
///
/// - `Result<(), io::Error>` - Traversal status.
fn collect_text_files(
    root: &Path,
    path: &Path,
    is_root: bool,
    ignore_rules: &[PrettierIgnoreRule],
    out: &mut Vec<PathBuf>,
) -> Result<(), io::Error> {
    if path.is_file() {
        let kind: FileFormatKind = detect_format_kind(path);
        if is_active_text_format(kind) {
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
        let file_type: FileType = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        let child: PathBuf = entry.path();
        let rel_posix: String = to_relative_posix(root, &child);
        if file_type.is_dir() {
            if is_prettier_ignored(&rel_posix, true, ignore_rules) {
                continue;
            }
            collect_text_files(root, &child, false, ignore_rules, out)?;
        } else if file_type.is_file() {
            if is_prettier_ignored(&rel_posix, false, ignore_rules) {
                continue;
            }
            let kind: FileFormatKind = detect_format_kind(&child);
            if is_active_text_format(kind) {
                out.push(child);
            }
        }
    }
    Ok(())
}

/// Parses `.prettierignore` at `root` if present into gitignore-style matching rules.
///
/// # Arguments
///
/// - `&Path` - Workspace root directory.
///
/// # Returns
///
/// - `Vec<PrettierIgnoreRule>` - Ordered list of parsed ignore rules.
fn load_prettier_ignore(root: &Path) -> Vec<PrettierIgnoreRule> {
    let ignore_path: PathBuf = root.join(FMT_PRETTIER_IGNORE_FILE);
    let content: String = match fs::read_to_string(&ignore_path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    let mut rules: Vec<PrettierIgnoreRule> = Vec::new();
    for raw_line in normalize_line_endings(&content).split(TEXT_LF) {
        let trimmed: &str = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (negated, after_bang): (bool, &str) = match trimmed.strip_prefix('!') {
            Some(rest) => (true, rest),
            None => (false, trimmed),
        };
        let (dir_only, after_slash): (bool, &str) = match after_bang.strip_suffix('/') {
            Some(rest) => (true, rest),
            None => (false, after_bang),
        };
        let (leading_slash, body): (bool, &str) = match after_slash.strip_prefix('/') {
            Some(rest) => (true, rest),
            None => (false, after_slash),
        };
        if body.is_empty() {
            continue;
        }
        let anchored: bool = leading_slash || body.contains('/');
        rules.push(PrettierIgnoreRule::new(
            body.to_string(),
            negated,
            dir_only,
            anchored,
        ));
    }
    rules
}

/// Converts `child` into a forward-slash relative path string rooted at `root`.
///
/// # Arguments
///
/// - `&Path` - Base directory path.
/// - `&Path` - Descendant path.
///
/// # Returns
///
/// - `String` - Forward-slash relative path.
fn to_relative_posix(root: &Path, child: &Path) -> String {
    let rel: &Path = child.strip_prefix(root).unwrap_or(child);
    rel.to_string_lossy().replace('\\', "/")
}

/// Evaluates `.prettierignore` rules in order for a relative POSIX path.
///
/// # Arguments
///
/// - `&str` - Forward-slash relative path from workspace root.
/// - `bool` - Whether the path refers to a directory.
/// - `&[PrettierIgnoreRule]` - Ordered ignore rules.
///
/// # Returns
///
/// - `bool` - `true` when the path should be skipped.
fn is_prettier_ignored(rel_path: &str, is_dir: bool, rules: &[PrettierIgnoreRule]) -> bool {
    if rules.is_empty() || rel_path.is_empty() {
        return false;
    }
    let mut ignored: bool = false;
    for rule in rules {
        if rule_matches_path(rule, rel_path, is_dir) {
            ignored = !rule.get_negated();
        }
    }
    ignored
}

/// Tests whether a single `.prettierignore` rule matches `rel_path` (checking ancestor directories only for non-negated rules).
///
/// # Arguments
///
/// - `&PrettierIgnoreRule` - Ignore rule to test.
/// - `&str` - Forward-slash relative path.
/// - `bool` - Whether `rel_path` itself is a directory.
///
/// # Returns
///
/// - `bool` - Whether the rule matches.
fn rule_matches_path(rule: &PrettierIgnoreRule, rel_path: &str, is_dir: bool) -> bool {
    let pattern: &str = rule.get_pattern();
    let anchored: bool = rule.get_anchored();
    if rule.get_negated() {
        if rule.get_dir_only() && !is_dir {
            return false;
        }
        return match_pattern_against(pattern, anchored, rel_path);
    }
    if (!rule.get_dir_only() || is_dir) && match_pattern_against(pattern, anchored, rel_path) {
        return true;
    }
    let mut cursor: usize = 0;
    while let Some(pos) = rel_path[cursor..].find('/') {
        let dir_prefix: &str = &rel_path[..(cursor + pos)];
        if match_pattern_against(pattern, anchored, dir_prefix) {
            return true;
        }
        cursor += pos + 1;
    }
    false
}

/// Matches a normalized glob pattern against either the full path (if anchored) or the final path segment.
///
/// # Arguments
///
/// - `&str` - Normalized glob pattern.
/// - `bool` - Whether the pattern is anchored to the workspace root.
/// - `&str` - Relative POSIX path candidate.
///
/// # Returns
///
/// - `bool` - Whether the glob matches.
fn match_pattern_against(pattern: &str, anchored: bool, rel_path: &str) -> bool {
    if anchored {
        return glob_match_path(pattern.as_bytes(), rel_path.as_bytes());
    }
    let base: &str = rel_path.rsplit('/').next().unwrap_or(rel_path);
    glob_match_path(pattern.as_bytes(), base.as_bytes())
}

/// Iterative glob matcher supporting `*` (non-slash), segment-bounded `**/`, `**` (cross-slash), and `?` (single non-slash char).
///
/// # Arguments
///
/// - `&[u8]` - Pattern bytes.
/// - `&[u8]` - Candidate path bytes.
///
/// # Returns
///
/// - `bool` - `true` when the entire candidate matches the pattern.
fn glob_match_path(pat: &[u8], text: &[u8]) -> bool {
    let plen: usize = pat.len();
    let tlen: usize = text.len();
    let mut prev: Vec<bool> = vec![false; tlen + 1];
    let mut curr: Vec<bool> = vec![false; tlen + 1];
    prev[0] = true;
    let mut pi: usize = 0;
    while pi < plen {
        curr.fill(false);
        if (pi == 0 || pat[pi - 1] == b'/')
            && pi + 2 < plen
            && pat[pi] == b'*'
            && pat[pi + 1] == b'*'
            && pat[pi + 2] == b'/'
        {
            let mut seen: bool = false;
            for ti in 0..=tlen {
                if prev[ti] {
                    curr[ti] = true;
                    seen = true;
                }
                if seen && ti < tlen && text[ti] == b'/' {
                    curr[ti + 1] = true;
                }
            }
            pi += 3;
        } else if pat[pi] == b'*' && pi + 1 < plen && pat[pi + 1] == b'*' {
            let mut seen: bool = false;
            for ti in 0..=tlen {
                if prev[ti] {
                    seen = true;
                }
                curr[ti] = seen;
            }
            pi += 2;
        } else if pat[pi] == b'*' {
            for ti in 0..=tlen {
                if prev[ti] || (ti > 0 && curr[ti - 1] && text[ti - 1] != b'/') {
                    curr[ti] = true;
                }
            }
            pi += 1;
        } else if pat[pi] == b'?' {
            for ti in 1..=tlen {
                if prev[ti - 1] && text[ti - 1] != b'/' {
                    curr[ti] = true;
                }
            }
            pi += 1;
        } else {
            let expected: u8 = pat[pi];
            for ti in 1..=tlen {
                if prev[ti - 1] && text[ti - 1] == expected {
                    curr[ti] = true;
                }
            }
            pi += 1;
        }
        prev.clone_from(&curr);
    }
    prev[tlen]
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
    if path.is_file() && kind != FileFormatKind::Rust {
        if is_active_text_format(kind) {
            let _: UnifiedFmtReport = format_workspace_text_files(path, false)?;
        }
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

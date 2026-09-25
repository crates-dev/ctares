use super::*;

#[test]
fn test_bump_version_type_enum() {
    assert_eq!(BumpVersionType::Patch, BumpVersionType::Patch);
    assert_eq!(BumpVersionType::Minor, BumpVersionType::Minor);
    assert_eq!(BumpVersionType::Major, BumpVersionType::Major);
    assert_eq!(BumpVersionType::Release, BumpVersionType::Release);
    assert_eq!(BumpVersionType::Alpha, BumpVersionType::Alpha);
    assert_eq!(BumpVersionType::Beta, BumpVersionType::Beta);
    assert_eq!(BumpVersionType::Rc, BumpVersionType::Rc);
}

#[test]
fn test_version_struct_creation() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("alpha.1".to_string()),
    };
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.prerelease, Some("alpha.1".to_string()));
}

#[test]
fn test_version_clone() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("beta".to_string()),
    };
    let cloned: Version = version.clone();
    assert_eq!(cloned.major, version.major);
    assert_eq!(cloned.minor, version.minor);
    assert_eq!(cloned.patch, version.patch);
    assert_eq!(cloned.prerelease, version.prerelease);
}

#[tokio::test]
async fn test_execute_bump_integration() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.1");
    let updated_content: String = read_to_string(&manifest_path).await.unwrap();
    assert!(updated_content.contains("version = \"0.1.1\""));
}

#[tokio::test]
async fn test_execute_bump_minor() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_minor");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Minor).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.2.0");
}

#[tokio::test]
async fn test_execute_bump_major() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_major");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Major).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.0.0");
}

#[tokio::test]
async fn test_execute_bump_alpha() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_alpha");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Alpha).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-alpha");
}

#[tokio::test]
async fn test_execute_bump_beta() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_beta");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha.2"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Beta).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-beta.1");
}

#[tokio::test]
async fn test_execute_bump_rc() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_rc");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-beta.1"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Rc).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-rc.1");
}

#[tokio::test]
async fn test_execute_bump_release() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_release");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Release).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0");
}

#[tokio::test]
async fn test_execute_bump_no_version_field() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_no_version");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_execute_bump_preserves_manifest_formatting() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_preserve_format");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "root"
version.workspace = true

[workspace.package]
version = "21.5.2"
edition = "2024"

# comment above workspace.dependencies must survive
[workspace.dependencies]
hyperlane-core = { path = "core", version = "21.5.2" }
serde = { version = "1.0.229", features = ["derive"] }

[profile.dev]
opt-level = 3
"#;
    let expected: &str = r#"[package]
name = "root"
version.workspace = true

[workspace.package]
version = "21.5.3"
edition = "2024"

# comment above workspace.dependencies must survive
[workspace.dependencies]
hyperlane-core = { path = "core", version = "21.5.2" }
serde = { version = "1.0.229", features = ["derive"] }

[profile.dev]
opt-level = 3
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "21.5.3");
    let updated: String = read_to_string(&manifest_path).await.unwrap();
    assert_eq!(updated, expected);
}

#[tokio::test]
async fn test_execute_bump_virtual_workspace_members() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_members");
    create_dir_all(tmp_dir.join("alpha")).await.unwrap();
    create_dir_all(tmp_dir.join("beta")).await.unwrap();
    let root_content: &str = r#"[workspace]
members = ["alpha", "beta"]

[workspace.dependencies]
beta = { path = "beta", version = "2.3.4" }

serde = { version = "1.0.229", features = ["derive"] }
"#;
    let alpha_content: &str = r#"[package]
name = "alpha"
version = "0.1.0"
edition = "2024"

[dependencies]
beta = { path = "../beta", version = "2.3.4" }

[dev-dependencies]
beta = { path = "../beta" }
"#;
    let beta_content: &str = r#"[package]
name = "beta"
version = "2.3.4"
edition = "2024"
"#;
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    write(&manifest_path, root_content).await.unwrap();
    write(tmp_dir.join("alpha/Cargo.toml"), alpha_content)
        .await
        .unwrap();
    write(tmp_dir.join("beta/Cargo.toml"), beta_content)
        .await
        .unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Minor).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "2 workspace members");
    let root_updated: String = read_to_string(&manifest_path).await.unwrap();
    assert_eq!(
        root_updated,
        r#"[workspace]
members = ["alpha", "beta"]

[workspace.dependencies]
beta = { path = "beta", version = "2.4.0" }

serde = { version = "1.0.229", features = ["derive"] }
"#
    );
    let alpha_updated: String = read_to_string(tmp_dir.join("alpha/Cargo.toml"))
        .await
        .unwrap();
    assert_eq!(
        alpha_updated,
        r#"[package]
name = "alpha"
version = "0.2.0"
edition = "2024"

[dependencies]
beta = { path = "../beta", version = "2.4.0" }

[dev-dependencies]
beta = { path = "../beta" }
"#
    );
    let beta_updated: String = read_to_string(tmp_dir.join("beta/Cargo.toml"))
        .await
        .unwrap();
    assert_eq!(
        beta_updated,
        r#"[package]
name = "beta"
version = "2.4.0"
edition = "2024"
"#
    );
}

#[tokio::test]
async fn test_execute_bump_virtual_workspace_glob_members() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_glob");
    create_dir_all(tmp_dir.join("crates/one")).await.unwrap();
    create_dir_all(tmp_dir.join("crates/two")).await.unwrap();
    create_dir_all(tmp_dir.join("crates/notacrate"))
        .await
        .unwrap();
    let root_content: &str = r#"[workspace]
members = ["crates/*"]
"#;
    write(tmp_dir.join("Cargo.toml"), root_content)
        .await
        .unwrap();
    for (member, version) in [("one", "0.1.0"), ("two", "1.2.3")] {
        let content: String = format!(
            "[package]\nname = \"{member}\"\nversion = \"{version}\"\nedition = \"2024\"\n"
        );
        write(tmp_dir.join(format!("crates/{member}/Cargo.toml")), content)
            .await
            .unwrap();
    }
    let result: Result<String, Box<dyn std::error::Error>> = execute_bump(
        tmp_dir.join("Cargo.toml").to_str().unwrap(),
        &BumpVersionType::Patch,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "2 workspace members");
    let one: String = read_to_string(tmp_dir.join("crates/one/Cargo.toml"))
        .await
        .unwrap();
    assert!(one.contains("version = \"0.1.1\""));
    let two: String = read_to_string(tmp_dir.join("crates/two/Cargo.toml"))
        .await
        .unwrap();
    assert!(two.contains("version = \"1.2.4\""));
}

#[tokio::test]
async fn test_execute_bump_shared_version_monorepo_leaves_members_untouched() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_shared");
    create_dir_all(tmp_dir.join("core")).await.unwrap();
    let root_content: &str = r#"[workspace]
members = ["core"]

[workspace.package]
version = "21.7.0"
edition = "2024"
"#;
    let core_content: &str = r#"[package]
name = "core"
version.workspace = true
edition.workspace = true
"#;
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    write(&manifest_path, root_content).await.unwrap();
    write(tmp_dir.join("core/Cargo.toml"), core_content)
        .await
        .unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "21.7.1");
    let core_after: String = read_to_string(tmp_dir.join("core/Cargo.toml"))
        .await
        .unwrap();
    assert_eq!(core_after, core_content);
}

#[tokio::test]
async fn test_execute_bump_virtual_workspace_skips_inherited_member() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_skip_inherited");
    create_dir_all(tmp_dir.join("a")).await.unwrap();
    create_dir_all(tmp_dir.join("b")).await.unwrap();
    let root_content: &str = r#"[workspace]
members = ["a", "b"]

[workspace.package]
edition = "2024"
"#;
    write(tmp_dir.join("Cargo.toml"), root_content)
        .await
        .unwrap();
    write(
        tmp_dir.join("a/Cargo.toml"),
        "[package]\nname = \"a\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
    )
    .await
    .unwrap();
    let b_content: &str = "[package]\nname = \"b\"\nversion.workspace = true\nedition = \"2024\"\n";
    write(tmp_dir.join("b/Cargo.toml"), b_content)
        .await
        .unwrap();
    let result: Result<String, Box<dyn std::error::Error>> = execute_bump(
        tmp_dir.join("Cargo.toml").to_str().unwrap(),
        &BumpVersionType::Patch,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1 workspace members");
    let a: String = read_to_string(tmp_dir.join("a/Cargo.toml")).await.unwrap();
    assert!(a.contains("version = \"1.0.1\""));
    let b_after: String = read_to_string(tmp_dir.join("b/Cargo.toml")).await.unwrap();
    assert_eq!(b_after, b_content);
}

#[tokio::test]
async fn test_execute_bump_empty_workspace_without_package_errors() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_empty");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    write(&manifest_path, "[workspace]\nmembers = []\n")
        .await
        .unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_execute_bump_virtual_workspace_prerelease_members() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_ws_prerelease");
    create_dir_all(tmp_dir.join("rc")).await.unwrap();
    write(
        tmp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"rc\"]\n",
    )
    .await
    .unwrap();
    write(
        tmp_dir.join("rc/Cargo.toml"),
        "[package]\nname = \"rc\"\nversion = \"0.1.0-alpha.1\"\nedition = \"2024\"\n",
    )
    .await
    .unwrap();
    let result: Result<String, Box<dyn std::error::Error>> = execute_bump(
        tmp_dir.join("Cargo.toml").to_str().unwrap(),
        &BumpVersionType::Alpha,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1 workspace members");
    let rc: String = read_to_string(tmp_dir.join("rc/Cargo.toml")).await.unwrap();
    assert!(rc.contains("version = \"0.1.0-alpha.2\""));
}

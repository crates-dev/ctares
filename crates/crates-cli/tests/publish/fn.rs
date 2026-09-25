use super::*;

#[test]
fn test_package_creation() {
    let package: Package = Package {
        name: "test-package".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
        publish: true,
    };
    assert_eq!(package.name, "test-package");
    assert_eq!(package.version, "0.1.0");
    assert!(package.local_dependencies.is_empty());
}

#[test]
fn test_package_clone() {
    let package: Package = Package {
        name: "test-package".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec!["dep1".to_string()],
        publish: true,
    };
    let cloned: Package = package.clone();
    assert_eq!(cloned.name, package.name);
    assert_eq!(cloned.version, package.version);
    assert_eq!(cloned.local_dependencies.len(), 1);
}

#[test]
fn test_package_equality() {
    let package1: Package = Package {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
        publish: true,
    };
    let package2: Package = Package {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
        publish: true,
    };
    assert_eq!(package1, package2);
}

#[test]
fn test_publish_result_success() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: true,
        error: None,
        retries: 0,
    };
    assert_eq!(result.package_name, "test");
    assert!(result.success);
    assert!(result.error.is_none());
    assert_eq!(result.retries, 0);
}

#[test]
fn test_publish_result_failure() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: false,
        error: Some("network error".to_string()),
        retries: 3,
    };
    assert!(!result.success);
    assert_eq!(result.error, Some("network error".to_string()));
    assert_eq!(result.retries, 3);
}

#[test]
fn test_publish_result_clone() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: true,
        error: None,
        retries: 0,
    };
    let cloned: PublishResult = result.clone();
    assert_eq!(cloned.package_name, result.package_name);
    assert_eq!(cloned.success, result.success);
    assert_eq!(cloned.error, result.error);
    assert_eq!(cloned.retries, result.retries);
}

#[test]
fn test_publish_error_display() {
    let error1: PublishError = PublishError::ManifestParseError;
    assert!(error1.to_string().contains("Failed to parse"));
    let error2: PublishError = PublishError::CircularDependency;
    assert!(error2.to_string().contains("Circular dependency"));
}

#[test]
fn test_publish_error_from_io() {
    let io_error: io::Error = io::Error::new(io::ErrorKind::NotFound, "test");
    let publish_error: PublishError = PublishError::from(io_error);
    assert!(publish_error.to_string().contains("IO error"));
}

async fn create_test_package(base: &Path, name: &str, dependencies: &str) {
    let package_dir: PathBuf = base.join(name);
    let manifest: String = format!(
        "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n{dependencies}"
    );
    fs::create_dir_all(&package_dir).await.unwrap();
    fs::write(package_dir.join("Cargo.toml"), manifest)
        .await
        .unwrap();
}

fn package_names(packages: &[Package]) -> Vec<&str> {
    packages
        .iter()
        .map(|package: &Package| package.name.as_str())
        .collect()
}

#[tokio::test]
async fn test_resolve_publish_order_follows_members_order() {
    let temp_dir: PathBuf = temp_dir().join("hyperlane_cli_test_order_members");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"zed\", \"alpha\", \"mid\"]\n",
    )
    .await
    .unwrap();
    for name in ["zed", "alpha", "mid"] {
        create_test_package(&temp_dir, name, "").await;
    }
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(package_names(&packages), vec!["zed", "alpha", "mid"]);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_appends_root_package_last() {
    let temp_dir: PathBuf = temp_dir().join("hyperlane_cli_test_order_root_last");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"rootpkg\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\nmembers = [\"sub\"]\n",
    )
    .await
    .unwrap();
    create_test_package(&temp_dir, "sub", "").await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(package_names(&packages), vec!["sub", "rootpkg"]);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_without_root_package() {
    let temp_dir: PathBuf = temp_dir().join("hyperlane_cli_test_order_virtual");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"one\", \"two\"]\n",
    )
    .await
    .unwrap();
    create_test_package(&temp_dir, "one", "").await;
    create_test_package(&temp_dir, "two", "").await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(package_names(&packages), vec!["one", "two"]);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_rejects_dependent_before_dependency() {
    let temp_dir: PathBuf = temp_dir().join("hyperlane_cli_test_order_invalid");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"a\", \"b\"]\n",
    )
    .await
    .unwrap();
    create_test_package(
        &temp_dir,
        "a",
        "\n[dependencies]\nb = { path = \"../b\" }\n",
    )
    .await;
    create_test_package(&temp_dir, "b", "").await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let result: Result<Vec<Package>, PublishError> = resolve_publish_order(&manifest_path).await;
    match result {
        Err(PublishError::InvalidPublishOrder(message)) => {
            assert!(message.contains("a depends on b"));
            assert!(message.contains("workspace.members"));
        }
        _ => panic!("expected InvalidPublishOrder"),
    }
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_accepts_dependency_before_dependent() {
    let temp_dir: PathBuf = temp_dir().join("hyperlane_cli_test_order_valid");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"b\", \"a\"]\n",
    )
    .await
    .unwrap();
    create_test_package(
        &temp_dir,
        "a",
        "\n[dependencies]\nb = { path = \"../b\" }\n",
    )
    .await;
    create_test_package(&temp_dir, "b", "").await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(package_names(&packages), vec!["b", "a"]);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[test]
fn test_is_already_published() {
    assert!(is_already_published(
        "error: crate version `21.6.3` is already published"
    ));
    assert!(is_already_published(
        "error: the remote server responded with an error (status 403 Forbidden): this crate version has already been uploaded"
    ));
    assert!(!is_already_published(
        "error: failed to verify project tarball"
    ));
    assert!(!is_already_published(""));
}

#[tokio::test]
async fn test_resolve_publish_order_positions_root_before_dependents() {
    let temp_dir: PathBuf = temp_dir().join("crates_cli_test_order_root_middle");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"facade\"\nversion = \"1.0.0\"\nedition = \"2021\"\n\n[dependencies]\ncore = { path = \"core\" }\n\n[workspace]\nmembers = [\"core\", \"ui\"]\n",
    )
    .await
    .unwrap();
    create_test_package(&temp_dir, "core", "").await;
    create_test_package(
        &temp_dir,
        "ui",
        "\n[dependencies]\nfacade = { path = \"..\" }\n",
    )
    .await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(package_names(&packages), vec!["core", "facade", "ui"]);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_root_position_conflict_errors() {
    let temp_dir: PathBuf = temp_dir().join("crates_cli_test_order_root_conflict");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[package]\nname = \"facade\"\nversion = \"1.0.0\"\nedition = \"2021\"\n\n[dependencies]\ncore = { path = \"core\" }\n\n[workspace]\nmembers = [\"ui\", \"core\"]\n",
    )
    .await
    .unwrap();
    create_test_package(&temp_dir, "core", "").await;
    create_test_package(
        &temp_dir,
        "ui",
        "\n[dependencies]\nfacade = { path = \"..\" }\n",
    )
    .await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let result: Result<Vec<Package>, PublishError> = resolve_publish_order(&manifest_path).await;
    match result {
        Err(PublishError::InvalidPublishOrder(message)) => {
            assert!(message.contains("facade"));
            assert!(message.contains("workspace.members"));
        }
        _ => panic!("expected InvalidPublishOrder"),
    }
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_resolves_workspace_inherited_version() {
    let temp_dir: PathBuf = temp_dir().join("crates_cli_test_order_inherited_version");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"inherited\"]\n\n[workspace.package]\nversion = \"9.9.9\"\nedition = \"2021\"\n",
    )
    .await
    .unwrap();
    let package_dir: PathBuf = temp_dir.join("inherited");
    fs::create_dir_all(&package_dir).await.unwrap();
    fs::write(
        package_dir.join("Cargo.toml"),
        "[package]\nname = \"inherited\"\nversion.workspace = true\nedition.workspace = true\n",
    )
    .await
    .unwrap();
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(packages.len(), 1);
    assert_eq!(packages[0].version, "9.9.9");
    let _ = fs::remove_dir_all(&temp_dir).await;
}

#[tokio::test]
async fn test_resolve_publish_order_reads_publish_flag() {
    let temp_dir: PathBuf = temp_dir().join("crates_cli_test_order_publish_flag");
    let _cleanup = fs::remove_dir_all(&temp_dir).await;
    fs::create_dir_all(&temp_dir).await.unwrap();
    fs::write(
        temp_dir.join("Cargo.toml"),
        "[workspace]\nmembers = [\"skipme\", \"shipme\"]\n",
    )
    .await
    .unwrap();
    let skip_dir: PathBuf = temp_dir.join("skipme");
    fs::create_dir_all(&skip_dir).await.unwrap();
    fs::write(
        skip_dir.join("Cargo.toml"),
        "[package]\nname = \"skipme\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n",
    )
    .await
    .unwrap();
    create_test_package(&temp_dir, "shipme", "").await;
    let manifest_path: String = temp_dir.join("Cargo.toml").to_string_lossy().to_string();
    let packages: Vec<Package> = resolve_publish_order(&manifest_path).await.unwrap();
    assert_eq!(packages[0].name, "skipme");
    assert!(!packages[0].publish);
    assert_eq!(packages[1].name, "shipme");
    assert!(packages[1].publish);
    let _ = fs::remove_dir_all(&temp_dir).await;
}

use super::*;

#[tokio::test]
async fn test_format_path_integration() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_fmt");
    let _: Result<(), Error> = create_dir_all(&tmp_dir).await;
    let test_file: PathBuf = tmp_dir.join("test.rs");
    write(&test_file, "fn main() {\n    println!(\"hello\");\n}\n")
        .await
        .unwrap();
    let result: Result<(), io::Error> = format_path(&tmp_dir).await;
    assert!(result.is_ok());
}

#[test]
fn test_markdown_table_headings_lists_match_prettier() {
    let input: &str = "#  Title  \r\n\r\n\r\n## Sub\n* item 1\n* item 2\n  * nested\n1. first\n2. second\n\n| Name | Description | Align |\n|---|---:|---|\n| a | 中文测试 | x |\n| longer | ok | y |\n";
    let expected: &str = "# Title\n\n## Sub\n\n- item 1\n- item 2\n  - nested\n\n1. first\n2. second\n\n| Name   | Description | Align |\n| ------ | ----------: | ----- |\n| a      |    中文测试 | x     |\n| longer |          ok | y     |\n";
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
}

#[test]
fn test_markdown_table_all_alignments_and_escaped_pipes() {
    let input: &str = "| Left | Center | Right | None |\n|:---|:---:|---:|---|\n| 1 | 2 | 3 | 4 |\n| long text | `a \\| b` | 100 | 中文 |\n";
    let expected: &str = "| Left      |  Center  | Right | None |\n| :-------- | :------: | ----: | ---- |\n| 1         |    2     |     3 | 4    |\n| long text | `a \\| b` |   100 | 中文 |\n";
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
}

#[test]
fn test_markdown_frontmatter_and_embedded_code_blocks() {
    let input: &str = "---\ntitle: \"Hello\"\ntags: [\"a\", \"b\"]\n---\n\n# Demo\n\n```json\n{\"a\":1,\"b\":[1,2]}\n```\n\n```yaml\nname: \"test\"\nsteps:\n- run: echo hi\n```\n\n```rust\nfn main() {   println!(\"hi\"); }\n```\n";
    let expected: &str = "---\ntitle: 'Hello'\ntags: ['a', 'b']\n---\n\n# Demo\n\n```json\n{ \"a\": 1, \"b\": [1, 2] }\n```\n\n```yaml\nname: 'test'\nsteps:\n  - run: echo hi\n```\n\n```rust\nfn main() {   println!(\"hi\"); }\n```\n";
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
}

#[test]
fn test_markdown_lists_alerts_breaks_html_and_nested_code() {
    let input: &str = "* a\n  * b\n    + c\n1) one\n2) two\n\n- [ ] todo\n- [x] done\n\n> [!WARNING]\n> First line.\n>\n> Second line.\n\n***\n\n<center>\n\n## euv\n\n</center>\n";
    let expected: &str = "- a\n  - b\n    - c\n\n1. one\n2. two\n\n- [ ] todo\n- [x] done\n\n> [!WARNING]\n> First line.\n>\n> Second line.\n\n---\n\n<center>\n\n## euv\n\n</center>\n";
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
    let nested_in: &str = "- item\n\n  ```json\n  {\"k\":1}\n  ```\n";
    let nested_expected: &str = "- item\n\n  ```json\n  { \"k\": 1 }\n  ```\n";
    assert_eq!(format_markdown(nested_in), nested_expected);
    let emphasis_in: &str = "*Italic tagline* with **bold** and `*code*` and a*b*c.\n";
    let emphasis_expected: &str = "_Italic tagline_ with **bold** and `*code*` and a*b*c.\n";
    assert_eq!(format_markdown(emphasis_in), emphasis_expected);
}

#[test]
fn test_yaml_workflow_and_compose_match_prettier() {
    let workflow_in: &str = "name: Rust\n\non:\n  push:\n    branches: [\"master\"]\n  pull_request:\n    branches:\n    - master\n\njobs:\n  build:\n    runs-on: \"ubuntu-latest\"\n    steps:\n    - name: Checkout\n      uses: actions/checkout@v4\n      with:\n        fetch-depth: 0\n    - name: Run\n      run: |\n        echo \"hello\"\n        echo 'world'\n";
    let workflow_expected: &str = "name: Rust\n\non:\n  push:\n    branches: ['master']\n  pull_request:\n    branches:\n      - master\n\njobs:\n  build:\n    runs-on: 'ubuntu-latest'\n    steps:\n      - name: Checkout\n        uses: actions/checkout@v4\n        with:\n          fetch-depth: 0\n      - name: Run\n        run: |\n          echo \"hello\"\n          echo 'world'\n";
    let workflow_out: String = format_yaml(workflow_in);
    assert_eq!(workflow_out, workflow_expected);
    assert_eq!(format_yaml(&workflow_out), workflow_out);
    let compose_in: &str = "services:\n  app:\n    image: \"postgres:16\"\n    ports:\n      - \"5432:5432\"\n    environment:\n      KEY: \"val\"\n      HAS_QUOTE: \"it's\"\n      ESCAPED: \"a\\nb\"\n";
    let compose_expected: &str = "services:\n  app:\n    image: 'postgres:16'\n    ports:\n      - '5432:5432'\n    environment:\n      KEY: 'val'\n      HAS_QUOTE: \"it's\"\n      ESCAPED: \"a\\nb\"\n";
    let compose_out: String = format_yaml(compose_in);
    assert_eq!(compose_out, compose_expected);
    assert_eq!(format_yaml(&compose_out), compose_out);
}

#[test]
fn test_yaml_quoted_keys_nested_seq_and_block_scalar() {
    let keys_in: &str = "\"quoted-key\": \"val\"\nplain: \"hello\"\nwith_apos: \"don't\"\nwith_esc: \"line1\\nline2\"\nsingle_existing: 'keep'\nflow_map: {a: \"1\", b: \"2\"}\nflow_arr: [\"a\", \"b\", \"c\"]\n";
    let keys_expected: &str = "'quoted-key': 'val'\nplain: 'hello'\nwith_apos: \"don't\"\nwith_esc: \"line1\\nline2\"\nsingle_existing: 'keep'\nflow_map: { a: '1', b: '2' }\nflow_arr: ['a', 'b', 'c']\n";
    assert_eq!(format_yaml(keys_in), keys_expected);
    let nested_in: &str = "matrix:\n  include:\n  - os: ubuntu-latest\n    name: \"linux\"\n    env:\n      FOO: \"bar\"\n  - os: macos-latest\n    name: \"mac\"\n";
    let nested_expected: &str = "matrix:\n  include:\n    - os: ubuntu-latest\n      name: 'linux'\n      env:\n        FOO: 'bar'\n    - os: macos-latest\n      name: 'mac'\n";
    assert_eq!(format_yaml(nested_in), nested_expected);
    let scalar_in: &str = "steps:\n- name: Script\n  run: |-\n      set -e\n      if true; then\n        echo \"hi\"\n      fi\n";
    let scalar_expected: &str = "steps:\n  - name: Script\n    run: |-\n      set -e\n      if true; then\n        echo \"hi\"\n      fi\n";
    assert_eq!(format_yaml(scalar_in), scalar_expected);
}

#[test]
fn test_yaml_preserves_github_actions_expressions() {
    let input: &str = "jobs:\n  test:\n    if: ${{ github.ref_name == 'master' && contains(github.event.head_commit.message, \"skip\") }}\n    steps:\n      - name: Echo\n        env:\n          TAG: ${{ needs.setup.outputs.tag }}\n        run: echo \"ok\"\n";
    let formatted: String = format_yaml(input);
    assert_eq!(formatted, input);
}

#[test]
fn test_json_and_jsonc_match_prettier() {
    let input: &str = "{\"a\":1,\"b\":[1,2,3],\"c\":{\"nested\":\"val\",\"arr\":[\"long-item-one\",\"long-item-two\",\"long-item-three\",\"long-item-four\"]}}\n";
    let expected: &str = "{\n  \"a\": 1,\n  \"b\": [1, 2, 3],\n  \"c\": {\n    \"nested\": \"val\",\n    \"arr\": [\n      \"long-item-one\",\n      \"long-item-two\",\n      \"long-item-three\",\n      \"long-item-four\"\n    ]\n  }\n}\n";
    let formatted: String = format_json(input).unwrap();
    assert_eq!(formatted, expected);
    assert_eq!(format_json(&formatted).unwrap(), formatted);
    let jsonc_in: &str = "{\n    // comment\n    \"a\": 1,\n    \"b\": [1, 2],\n}\n";
    let jsonc_expected: &str = "{\n  // comment\n  \"a\": 1,\n  \"b\": [1, 2],\n}\n";
    assert_eq!(format_jsonc(jsonc_in), jsonc_expected);
}

#[test]
fn test_toml_and_plain_text_formatting() {
    let toml_in: &str = "[package]\r\nname=\"demo\"   \r\ndescription=\"UTF-8 ’ curly quote and 中文\"\r\n\r\n\r\nexclude=[\r\n  \"target\",\r\n  \"dist\",\r\n]\r\n";
    let toml_expected: &str = "[package]\nname = \"demo\"\ndescription = \"UTF-8 ’ curly quote and 中文\"\n\nexclude = [\n    \"target\",\n    \"dist\",\n]\n";
    assert_eq!(format_toml(toml_in), toml_expected);
    assert_eq!(format_toml(toml_expected), toml_expected);
    let makefile_in: &str = "build:  \r\n\tcargo build   \r\n\r\n";
    assert_eq!(
        format_plain_text(makefile_in, true),
        "build:\n\tcargo build\n"
    );
    let sh_in: &str = "#!/usr/bin/env bash\r\n\techo hi   \r\n\r\n";
    assert_eq!(
        format_plain_text(sh_in, false),
        "#!/usr/bin/env bash\n  echo hi\n"
    );
}

#[tokio::test]
async fn test_detect_format_kind_and_workspace_text_fmt_check_mode() {
    assert_eq!(
        detect_format_kind(Path::new("README.md")),
        FileFormatKind::Markdown
    );
    assert_eq!(
        detect_format_kind(Path::new(".github/workflows/rust.yml")),
        FileFormatKind::Yaml
    );
    assert_eq!(
        detect_format_kind(Path::new("compose.yaml")),
        FileFormatKind::Yaml
    );
    assert_eq!(
        detect_format_kind(Path::new("config.json")),
        FileFormatKind::Json
    );
    assert_eq!(
        detect_format_kind(Path::new("settings.jsonc")),
        FileFormatKind::Jsonc
    );
    assert_eq!(
        detect_format_kind(Path::new("Cargo.toml")),
        FileFormatKind::Toml
    );
    assert_eq!(
        detect_format_kind(Path::new("Makefile")),
        FileFormatKind::PlainTextPreserveTabs
    );
    assert_eq!(
        detect_format_kind(Path::new("scripts/run.sh")),
        FileFormatKind::PlainText
    );
    assert_eq!(
        detect_format_kind(Path::new("src/lib.rs")),
        FileFormatKind::Rust
    );
    assert_eq!(
        detect_format_kind(Path::new("logo.png")),
        FileFormatKind::Unsupported
    );
    let root: PathBuf = temp_dir().join("crate_cli_unified_fmt_test");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    create_dir_all(root.join("target")).await.unwrap();
    let md_path: PathBuf = root.join("README.md");
    let yml_path: PathBuf = root.join("ci.yml");
    let ignored_path: PathBuf = root.join("target/ignored.md");
    write(&md_path, "#  Hello\n* a\n").await.unwrap();
    write(&yml_path, "name: \"ci\"\nsteps:\n- run: ok\n")
        .await
        .unwrap();
    write(&ignored_path, "#   Untouched\n").await.unwrap();
    let check_err: Result<UnifiedFmtReport, io::Error> = format_workspace_text_files(&root, true);
    assert!(check_err.is_err());
    let write_report: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(write_report.get_files_scanned(), 2);
    assert_eq!(write_report.get_files_formatted(), 2);
    assert_eq!(read_to_string(&md_path).await.unwrap(), "# Hello\n\n- a\n");
    assert_eq!(
        read_to_string(&yml_path).await.unwrap(),
        "name: 'ci'\nsteps:\n  - run: ok\n"
    );
    assert_eq!(
        read_to_string(&ignored_path).await.unwrap(),
        "#   Untouched\n"
    );
    let check_ok: UnifiedFmtReport = format_workspace_text_files(&root, true).unwrap();
    assert_eq!(check_ok.get_files_formatted(), 0);
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
}

use super::*;

#[tokio::test]
async fn test_format_path_integration() {
    let tmp_dir: PathBuf = temp_dir().join("crate_cli_format_path_integration");
    let _: Result<(), Error> = remove_dir_all(&tmp_dir).await;
    create_dir_all(&tmp_dir).await.unwrap();
    let test_file: PathBuf = tmp_dir.join("test.rs");
    write(&test_file, "fn main() {\n    println!(\"hello\");\n}\n")
        .await
        .unwrap();
    let result: Result<(), io::Error> = format_path(&tmp_dir).await;
    assert!(result.is_ok());
    let _: Result<(), Error> = remove_dir_all(&tmp_dir).await;
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
fn test_markdown_ordered_list_nesting_loose_lists_and_governance_paths() {
    let nested_in: &str = concat!(
        "# AGENTS.md\n\n",
        "1. **First item**:\n",
        "   - Nested bullet one\n",
        "   - Nested bullet two\n\n",
        "   ```sh\n",
        "   cargo fmt --all -- --check\n",
        "   ```\n\n",
        "2. **Second item**:\n",
        "   1. Sub-one\n",
        "   2. Sub-two\n\n",
        "   ```js\n",
        "   const x = \"double\"\n",
        "   ```\n\n",
        "## Ten\n\n",
        "10. **Tenth item**:\n",
        "    - Four-space child\n",
    );
    let nested_expected: &str = concat!(
        "# AGENTS.md\n\n",
        "1. **First item**:\n",
        "   - Nested bullet one\n",
        "   - Nested bullet two\n\n",
        "   ```sh\n",
        "   cargo fmt --all -- --check\n",
        "   ```\n\n",
        "2. **Second item**:\n",
        "   1. Sub-one\n",
        "   2. Sub-two\n\n",
        "   ```js\n",
        "   const x = 'double';\n",
        "   ```\n\n",
        "## Ten\n\n",
        "10. **Tenth item**:\n",
        "    - Four-space child\n",
    );
    let governance_paths: [&str; 4] = ["AGENTS.md", "Agent.md", "SKILL.md", "README.md"];
    for rel in governance_paths {
        let formatted: String = format_text_by_path(Path::new(rel), nested_in);
        assert_eq!(formatted, nested_expected);
        assert_eq!(format_text_by_path(Path::new(rel), &formatted), formatted);
    }
    let two_space_in: &str = concat!(
        "1. Outer item:\n",
        "  - Un-nested sibling\n\n",
        "  ```sh\n",
        "  echo hi\n",
        "  ```\n",
    );
    let two_space_expected: &str = concat!(
        "1. Outer item:\n\n",
        "- Un-nested sibling\n\n",
        "```sh\n",
        "echo hi\n",
        "```\n",
    );
    let two_space_out: String = format_markdown(two_space_in);
    assert_eq!(two_space_out, two_space_expected);
    assert_eq!(format_markdown(&two_space_out), two_space_out);
    let loose_in: &str = "- first paragraph\n\n- second paragraph\n\n- third paragraph\n";
    let loose_out: String = format_markdown(loose_in);
    assert_eq!(loose_out, loose_in);
    assert_eq!(format_markdown(&loose_out), loose_out);
}

#[test]
fn test_markdown_setext_emoji_table_breaks_comments_emphasis_titles_and_empty_fences() {
    let setext_and_emphasis_in: &str = concat!(
        "Title\n",
        "=====\n\n",
        "Subtitle\n",
        "--------\n\n",
        "Text *em*, **strong**, ***triple-star***, ___triple-under___, **_mixed1_**, _**mixed2**_, ~~strike~~.\n\n",
        "Line with hard break  \n",
        "next line.\n\n",
        "<!--\n",
        "multiline\n",
        "comment\n",
        "-->\n",
    );
    let setext_and_emphasis_expected: &str = concat!(
        "# Title\n\n",
        "## Subtitle\n\n",
        "Text _em_, **strong**, **_triple-star_**, **_triple-under_**, **_mixed1_**, _**mixed2**_, ~~strike~~.\n\n",
        "Line with hard break  \n",
        "next line.\n\n",
        "<!--\n",
        "multiline\n",
        "comment\n",
        "-->\n",
    );
    let out: String = format_markdown(setext_and_emphasis_in);
    assert_eq!(out, setext_and_emphasis_expected);
    assert_eq!(format_markdown(&out), out);
    let emoji_table_in: &str = concat!(
        "| Name | Status | Star |\n",
        "|:---|:---:|---:|\n",
        "| 中文 | ✅ | ⭐ |\n",
        "| alpha | ⚠️ | ok |\n",
    );
    let emoji_table_expected: &str = concat!(
        "| Name  | Status | Star |\n",
        "| :---- | :----: | ---: |\n",
        "| 中文  |   ✅   |   ⭐ |\n",
        "| alpha |   ⚠️   |   ok |\n",
    );
    let emoji_out: String = format_markdown(emoji_table_in);
    assert_eq!(emoji_out, emoji_table_expected);
    assert_eq!(format_markdown(&emoji_out), emoji_out);
    let titles_in: &str = concat!(
        "Link [a](https://example.com \"Double\"), ",
        "[b](https://example.com (Paren)), ",
        "[c](<https://example.com/a b> \"Angle\"), ",
        "[d](https://example.com \"it's ok\"), ",
        "[e](https://example.com \"it's \\\"both\\\"\"), ",
        "[f](https://example.com \"say \\\"hi\\\"\").\n\n",
        "![img](https://example.com/a.png \"Img\")\n\n",
        "[ref]: https://example.com \"Ref\"\n",
    );
    let titles_expected: &str = concat!(
        "Link [a](https://example.com 'Double'), ",
        "[b](https://example.com 'Paren'), ",
        "[c](<https://example.com/a b> 'Angle'), ",
        "[d](https://example.com \"it's ok\"), ",
        "[e](https://example.com (it's \"both\")), ",
        "[f](https://example.com 'say \"hi\"').\n\n",
        "![img](https://example.com/a.png 'Img')\n\n",
        "[ref]: https://example.com 'Ref'\n",
    );
    let titles_out: String = format_markdown(titles_in);
    assert_eq!(titles_out, titles_expected);
    assert_eq!(format_markdown(&titles_out), titles_out);
    let empty_fence_in: &str = "```\n```\n\n> ```\n> ```\n";
    let empty_fence_expected: &str = "```\n\n```\n\n> ```\n>\n> ```\n";
    let empty_fence_out: String = format_markdown(empty_fence_in);
    assert_eq!(empty_fence_out, empty_fence_expected);
    assert_eq!(format_markdown(&empty_fence_out), empty_fence_out);
}

#[test]
fn test_markdown_frontmatter_wrap_and_embedded_fence_allowlist() {
    let input: &str = concat!(
        "---\n",
        "name: demo\n",
        "description: \"Double\"\n",
        "metadata:\n",
        "  hermes:\n",
        "    related_skills: [alpha-skill, beta-skill, gamma-skill, delta-skill, epsilon-skill]\n",
        "---\n\n",
        "```js\n",
        "const f = x => \"hi\"\n",
        "```\n\n",
        "```ts\n",
        "const g = (n: number): string => \"v\" + n\n",
        "```\n\n",
        "```json\n",
        "{\"a\":1,\"b\":[2,3]}\n",
        "```\n\n",
        "```yaml\n",
        "key: \"value\"\n",
        "```\n\n",
        "```css\n",
        "body{font-family:\"Inter\",sans-serif}\n",
        "```\n\n",
        "```html\n",
        "<!DOCTYPE html><div><span>hi</span></div>\n",
        "```\n\n",
        "```toml\n",
        "a=\"raw\"\n",
        "```\n\n",
        "```sh\n",
        "echo   \"raw\"\n",
        "```\n\n",
        "```bash\n",
        "echo   \"raw\"\n",
        "```\n\n",
        "```rust\n",
        "fn main(){println!(\"raw\");}\n",
        "```\n",
    );
    let expected: &str = concat!(
        "---\n",
        "name: demo\n",
        "description: 'Double'\n",
        "metadata:\n",
        "  hermes:\n",
        "    related_skills:\n",
        "      [alpha-skill, beta-skill, gamma-skill, delta-skill, epsilon-skill]\n",
        "---\n\n",
        "```js\n",
        "const f = (x) => 'hi';\n",
        "```\n\n",
        "```ts\n",
        "const g = (n: number): string => 'v' + n;\n",
        "```\n\n",
        "```json\n",
        "{ \"a\": 1, \"b\": [2, 3] }\n",
        "```\n\n",
        "```yaml\n",
        "key: 'value'\n",
        "```\n\n",
        "```css\n",
        "body {\n",
        "  font-family: 'Inter', sans-serif;\n",
        "}\n",
        "```\n\n",
        "```html\n",
        "<!DOCTYPE html>\n",
        "<div><span>hi</span></div>\n",
        "```\n\n",
        "```toml\n",
        "a=\"raw\"\n",
        "```\n\n",
        "```sh\n",
        "echo   \"raw\"\n",
        "```\n\n",
        "```bash\n",
        "echo   \"raw\"\n",
        "```\n\n",
        "```rust\n",
        "fn main(){println!(\"raw\");}\n",
        "```\n",
    );
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
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
    let compose_out: String = format_text_by_path(Path::new("docker-compose.yml"), compose_in);
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
fn test_yaml_single_quote_wrapping_prettierrc_and_invalid_fallback() {
    let wrap_in: &str = concat!(
        "escaped: \"say \\\"hi\\\"\"\n",
        "flow_wrap: [alpha-item, beta-item, gamma-item, delta-item, epsilon-item, zeta-item]\n",
        "flow_collapse: [\n",
        "  \"x\",\n",
        "  \"y\"\n",
        "]\n",
    );
    let wrap_expected: &str = concat!(
        "escaped: 'say \"hi\"'\n",
        "flow_wrap:\n",
        "  [alpha-item, beta-item, gamma-item, delta-item, epsilon-item, zeta-item]\n",
        "flow_collapse: ['x', 'y']\n",
    );
    let wrap_out: String = format_yaml(wrap_in);
    assert_eq!(wrap_out, wrap_expected);
    assert_eq!(format_yaml(&wrap_out), wrap_out);
    let prettierrc_json_in: &str = "{\"semi\":true,\"singleQuote\":true}\n";
    let prettierrc_json_out: String =
        format_text_by_path(Path::new(".prettierrc"), prettierrc_json_in);
    assert_eq!(
        prettierrc_json_out,
        "{ \"semi\": true, \"singleQuote\": true }\n"
    );
    assert_eq!(
        format_text_by_path(Path::new(".prettierrc"), &prettierrc_json_out),
        prettierrc_json_out
    );
    let prettierrc_yaml_in: &str = "semi: true\nsingleQuote: true\ntabWidth: 2\n";
    let prettierrc_yaml_out: String =
        format_text_by_path(Path::new(".prettierrc"), prettierrc_yaml_in);
    assert_eq!(prettierrc_yaml_out, prettierrc_yaml_in);
    let invalid_yaml: &str = "a: [1, 2\nb: : :\n";
    assert_eq!(format_yaml(invalid_yaml), invalid_yaml);
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
fn test_json_object_wrap_stringify_tsconfig_jsonc_and_invalid_fallback() {
    let stripe_in: &str = "{\n  \"single\": { \"a\": 1 },\n  \"multi\": {\n    \"enabled\": true\n  },\n  \"arr\": [1, 2, 3]\n}\n";
    let stripe_out: String = format_text_by_path(Path::new("stripe_event.json"), stripe_in);
    assert_eq!(stripe_out, stripe_in);
    assert_eq!(
        format_text_by_path(Path::new("stripe_event.json"), &stripe_out),
        stripe_out
    );
    let pkg_in: &str =
        "{\"name\":\"demo\",\"version\":\"1.0.0\",\"dependencies\":{\"foo\":\"^1.0.0\"}}\n";
    let pkg_expected: &str = "{\n  \"name\": \"demo\",\n  \"version\": \"1.0.0\",\n  \"dependencies\": {\n    \"foo\": \"^1.0.0\"\n  }\n}\n";
    let manifest_names: [&str; 3] = ["package.json", "package-lock.json", "composer.json"];
    for name in manifest_names {
        let pkg_out: String = format_text_by_path(Path::new(name), pkg_in);
        assert_eq!(pkg_out, pkg_expected);
        assert_eq!(format_text_by_path(Path::new(name), &pkg_out), pkg_out);
    }
    assert_eq!(format_json_stringify(pkg_in).unwrap(), pkg_expected);
    let tsconfig_in: &str = "{\n  // comment in tsconfig\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"strict\": true,\n  },\n}\n";
    let tsconfig_expected: &str = "{\n  // comment in tsconfig\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"strict\": true\n  }\n}\n";
    let tsconfig_out: String = format_text_by_path(Path::new("tsconfig.json"), tsconfig_in);
    assert_eq!(tsconfig_out, tsconfig_expected);
    assert_eq!(
        format_text_by_path(Path::new("tsconfig.json"), &tsconfig_out),
        tsconfig_out
    );
    let vscode_in: &str = "{\n  // vscode setting\n  \"editor.tabSize\": 2,\n}\n";
    let vscode_expected: &str = "{\n  // vscode setting\n  \"editor.tabSize\": 2\n}\n";
    let vscode_out: String = format_text_by_path(Path::new(".vscode/settings.json"), vscode_in);
    assert_eq!(vscode_out, vscode_expected);
    let jsonc_in: &str = "{\n  // keep trailing comma\n  \"a\": 1,\n  \"b\": [1, 2,],\n}\n";
    let jsonc_expected: &str = "{\n  // keep trailing comma\n  \"a\": 1,\n  \"b\": [1, 2],\n}\n";
    assert_eq!(
        format_text_by_path(Path::new("config.jsonc"), jsonc_in),
        jsonc_expected
    );
    let cjson_in: &str = "{\n  // cjson\n  \"flag\": true,\n}\n";
    assert_eq!(
        format_text_by_path(Path::new("custom.cjson"), cjson_in),
        cjson_in
    );
    let invalid_json: &str = "{ invalid json }\n";
    assert!(format_json(invalid_json).is_err());
    assert!(format_json_stringify(invalid_json).is_err());
    assert_eq!(
        format_text_by_path(Path::new("broken.json"), invalid_json),
        invalid_json
    );
    assert_eq!(
        format_text_by_path(Path::new("package.json"), invalid_json),
        invalid_json
    );
    assert_eq!(format_jsonc(invalid_json), invalid_json);
}

#[test]
fn test_toml_taplo_formatting_and_invalid_fallback() {
    let toml_in: &str = concat!(
        "# Top comment\r\n",
        "title=\"demo\"\r\n\r\n\r\n\r\n",
        "[package]\r\n",
        "name =   \"demo\"\r\n",
        "description = \"UTF-8 ’ curly quote and 中文\"\r\n",
        "version = \"0.1.0\"\r\n",
        "edition = \"2024\"\r\n",
        "keywords = [ \"b\", \"a\" ]\r\n",
        "long_array = [\"alpha-item\", \"beta-item\", \"gamma-item\", \"delta-item\", \"epsilon-item\", \"zeta-item\"]\r\n",
        "multiline_short = [\r\n",
        "  \"x\",\r\n",
        "  \"y\"\r\n",
        "]\r\n",
        "nested_array = [\r\n",
        "  [\"a\", \"b\"],\r\n",
        "  [\"c\", \"d\"]\r\n",
        "]\r\n\r\n",
        "[dependencies]\r\n",
        "z_crate = { version = \"1.0\", features = [\"alpha-feature\", \"beta-feature\", \"gamma-feature\"], default-features = false }\r\n",
        "a_crate = \"0.2\" # trailing comment\r\n",
        "longer_crate = {path=\"../x\",optional=true} # another comment\r\n\r\n",
        "[[bin]]\r\n",
        "name=\"demo\"\r\n",
        "path = \"src/main.rs\"\r\n",
    );
    let toml_expected: &str = concat!(
        "# Top comment\n",
        "title = \"demo\"\n\n\n",
        "[package]\n",
        "name = \"demo\"\n",
        "description = \"UTF-8 ’ curly quote and 中文\"\n",
        "version = \"0.1.0\"\n",
        "edition = \"2024\"\n",
        "keywords = [\"b\", \"a\"]\n",
        "long_array = [\n",
        "    \"alpha-item\",\n",
        "    \"beta-item\",\n",
        "    \"gamma-item\",\n",
        "    \"delta-item\",\n",
        "    \"epsilon-item\",\n",
        "    \"zeta-item\",\n",
        "]\n",
        "multiline_short = [\"x\", \"y\"]\n",
        "nested_array = [[\"a\", \"b\"], [\"c\", \"d\"]]\n\n",
        "[dependencies]\n",
        "z_crate = { version = \"1.0\", features = [\n",
        "    \"alpha-feature\",\n",
        "    \"beta-feature\",\n",
        "    \"gamma-feature\",\n",
        "], default-features = false }\n",
        "a_crate = \"0.2\" # trailing comment\n",
        "longer_crate = { path = \"../x\", optional = true } # another comment\n\n",
        "[[bin]]\n",
        "name = \"demo\"\n",
        "path = \"src/main.rs\"\n",
    );
    let toml_out: String = format_text_by_path(Path::new("Cargo.toml"), toml_in);
    assert_eq!(toml_out, toml_expected);
    assert_eq!(format_toml(&toml_out), toml_out);
    let invalid_toml: &str = "[broken\nkey = ===\n";
    assert_eq!(format_toml(invalid_toml), invalid_toml);
}

#[test]
fn test_web_js_ts_tsx_css_scss_less_html_vue_match_prettier() {
    let js_in: &str = concat!(
        "/**\n",
        " * Keep JSDoc\n",
        " * @param {string} name\n",
        " */\n",
        "export const greet = name => {\n",
        "  return [\n",
        "    \"hello\",\n",
        "    name\n",
        "  ]\n",
        "}\n",
    );
    let js_expected: &str = concat!(
        "/**\n",
        " * Keep JSDoc\n",
        " * @param {string} name\n",
        " */\n",
        "export const greet = (name) => {\n",
        "  return ['hello', name];\n",
        "};\n",
    );
    let js_out: String = format_javascript(js_in);
    assert_eq!(js_out, js_expected);
    assert_eq!(format_javascript(&js_out), js_out);
    let mjs_in: &str =
        "import { readFileSync } from \"node:fs\"\nexport const data = { a: 1, b: \"two\" }\n";
    let mjs_expected: &str =
        "import { readFileSync } from 'node:fs';\nexport const data = { a: 1, b: 'two' };\n";
    assert_eq!(
        format_text_by_path(Path::new("sample.mjs"), mjs_in),
        mjs_expected
    );
    let cjs_in: &str = "const x = \"commonjs\"\nmodule.exports = { x, items: [1, 2, 3] }\n";
    let cjs_expected: &str = "const x = 'commonjs';\nmodule.exports = { x, items: [1, 2, 3] };\n";
    assert_eq!(
        format_text_by_path(Path::new("sample.cjs"), cjs_in),
        cjs_expected
    );
    let wxs_in: &str = "var name = \"demo\"\nmodule.exports = { name: name }\n";
    let wxs_expected: &str = "var name = 'demo';\nmodule.exports = { name: name };\n";
    assert_eq!(
        format_text_by_path(Path::new("sample.wxs"), wxs_in),
        wxs_expected
    );
    let ts_in: &str = "export const greet = (name: string,): string => \"Hello, \" + name\n";
    let ts_expected: &str = "export const greet = (name: string): string => 'Hello, ' + name;\n";
    let ts_out: String = format_typescript(ts_in);
    assert_eq!(ts_out, ts_expected);
    assert_eq!(format_typescript(&ts_out), ts_out);
    let mts_in: &str =
        "export type Mode = \"fast\" | \"safe\"\nexport const mode: Mode = \"fast\"\n";
    let mts_expected: &str =
        "export type Mode = 'fast' | 'safe';\nexport const mode: Mode = 'fast';\n";
    assert_eq!(
        format_text_by_path(Path::new("sample.mts"), mts_in),
        mts_expected
    );
    let cts_in: &str = "const port: number = 8080\nexport = { port }\n";
    let cts_expected: &str = "const port: number = 8080;\nexport = { port };\n";
    assert_eq!(
        format_text_by_path(Path::new("sample.cts"), cts_in),
        cts_expected
    );
    let tsx_in: &str = "export function Card({ title }: { title: string }) {\n  const label = \"card\"\n  return <div className='card' data-label={label}>{title}</div>\n}\n";
    let tsx_expected: &str = "export function Card({ title }: { title: string }) {\n  const label = 'card';\n  return (\n    <div className=\"card\" data-label={label}>\n      {title}\n    </div>\n  );\n}\n";
    let tsx_out: String = format_tsx(tsx_in);
    assert_eq!(tsx_out, tsx_expected);
    assert_eq!(format_tsx(&tsx_out), tsx_out);
    let invalid_js: &str = "const x = (((\n";
    assert_eq!(format_javascript(invalid_js), invalid_js);
    let css_in: &str = "body{margin:0;font-family:\"Inter\",sans-serif;color:#333}\n";
    let css_expected: &str =
        "body {\n  margin: 0;\n  font-family: 'Inter', sans-serif;\n  color: #333;\n}\n";
    let css_out: String = format_css(css_in);
    assert_eq!(css_out, css_expected);
    assert_eq!(format_css(&css_out), css_out);
    assert_eq!(
        format_text_by_path(Path::new("style.wxss"), css_in),
        css_expected
    );
    let scss_in: &str = "$primary: #0070f3;\n.btn { color: $primary; &:hover { opacity: 0.8; } }\n";
    let scss_expected: &str =
        "$primary: #0070f3;\n.btn {\n  color: $primary;\n  &:hover {\n    opacity: 0.8;\n  }\n}\n";
    let scss_out: String = format_scss(scss_in);
    assert_eq!(scss_out, scss_expected);
    assert_eq!(format_scss(&scss_out), scss_out);
    let less_in: &str = "@base: #f938ab;\n.box { border-color: @base; }\n";
    let less_expected: &str = "@base: #f938ab;\n.box {\n  border-color: @base;\n}\n";
    let less_out: String = format_less(less_in);
    assert_eq!(less_out, less_expected);
    assert_eq!(format_less(&less_out), less_out);
    let html_in: &str = "<!DOCTYPE html><html><head><style>body{font-family:\"Inter\";color:#333}</style></head><body><h1>Hi</h1><script>const msg = \"hello\"</script></body></html>\n";
    let html_expected: &str = concat!(
        "<!doctype html>\n",
        "<html>\n",
        "  <head>\n",
        "    <style>\n",
        "      body {\n",
        "        font-family: 'Inter';\n",
        "        color: #333;\n",
        "      }\n",
        "    </style>\n",
        "  </head>\n",
        "  <body>\n",
        "    <h1>Hi</h1>\n",
        "    <script>\n",
        "      const msg = 'hello';\n",
        "    </script>\n",
        "  </body>\n",
        "</html>\n",
    );
    let html_out: String = format_html(html_in);
    assert_eq!(html_out, html_expected);
    assert_eq!(format_html(&html_out), html_out);
    assert_eq!(
        format_text_by_path(Path::new("index.htm"), html_in),
        html_expected
    );
    let invalid_html: &str = "<img src=\"a.png\"></img>\n";
    assert_eq!(format_html(invalid_html), invalid_html);
    let vue_in: &str = "<template><div class=\"app\"><span>{{ msg }}</span></div></template>\n<script setup lang=\"ts\">\nconst msg: string = \"hello\"\n</script>\n<style scoped>\n.app{font-family:\"Inter\";color:#111}\n</style>\n";
    let vue_expected: &str = concat!(
        "<template>\n",
        "  <div class=\"app\">\n",
        "    <span>{{ msg }}</span>\n",
        "  </div>\n",
        "</template>\n",
        "<script setup lang=\"ts\">\n",
        "const msg: string = 'hello';\n",
        "</script>\n",
        "<style scoped>\n",
        ".app {\n",
        "  font-family: 'Inter';\n",
        "  color: #111;\n",
        "}\n",
        "</style>\n",
    );
    let vue_out: String = format_vue(vue_in);
    assert_eq!(vue_out, vue_expected);
    assert_eq!(format_vue(&vue_out), vue_out);
}

#[tokio::test]
async fn test_vscode_noop_text_files_preserved_verbatim() {
    let cases: [(&str, &str); 21] = [
        ("LICENSE", "MIT License\n\n\nCopyright (c) 2026   \n"),
        (".gitattributes", "*\ttext=auto\teol=lf\n"),
        (".gitignore", "/target\n\n\n*.log   \n"),
        (".dockerignore", "target/\n\n\n*.tmp\n"),
        (".editorconfig", "root = true\n\n[*]\nindent_style = tab\n"),
        (
            ".githooks/pre-commit",
            "#!/usr/bin/env bash\n\n\npython3 - <<'PY'\ndef first():\n    return 1\n\n\ndef second():\n    return 2\nPY\n",
        ),
        (
            "scripts/run.sh",
            "#!/usr/bin/env sh\n\techo \"keep tabs\"   \n\n\necho \"keep blanks\"\n",
        ),
        ("scripts/run.bash", "#!/usr/bin/env bash\n\techo \"tabs\"\n"),
        ("scripts/run.zsh", "#!/usr/bin/env zsh\n\techo \"zsh\"\n"),
        ("Makefile", "all:   \n\t@echo \"tab required\"\n\n"),
        ("Dockerfile", "FROM alpine:3.20\nRUN   echo \"raw\"\n"),
        (".env", "APP_ENV=dev\nSECRET=\"keep\"   \n"),
        ("notes.txt", "plain text   \r\n\tindented\r\n\r\n"),
        ("app.conf", "worker_processes  auto;\n\n\n"),
        ("config.ini", "[core]\n\tmode = raw   \n"),
        ("pom.xml", "<project>  <name>demo</name></project>\n"),
        (
            "icon.svg",
            "<svg><circle   cx=\"5\" cy=\"5\" r=\"4\"/></svg>\n",
        ),
        (
            "script.py",
            "def f():\n    x=1+2\n\n\ndef g():\n    return 0\n",
        ),
        ("query.sql", "select   * from users where id=1;\n"),
        (
            "App.jsx",
            "export const App = () => <div className=\"a\">hi</div>\n",
        ),
        (
            "Cargo.lock",
            "version = 4\n\n[[package]]\nname = \"demo\"\nversion = \"0.1.0\"\n",
        ),
    ];
    let root: PathBuf = temp_dir().join("crate_cli_vscode_noop_preservation_test");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    create_dir_all(&root).await.unwrap();
    for (rel, content) in cases {
        assert_eq!(format_text_by_path(Path::new(rel), content), content);
        let full: PathBuf = root.join(rel);
        if let Some(parent) = full.parent() {
            create_dir_all(parent).await.unwrap();
        }
        write(&full, content).await.unwrap();
    }
    let write_report: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(write_report.get_files_scanned(), 0);
    assert_eq!(write_report.get_files_formatted(), 0);
    let check_report: UnifiedFmtReport = format_workspace_text_files(&root, true).unwrap();
    assert_eq!(check_report.get_files_scanned(), 0);
    assert_eq!(check_report.get_files_formatted(), 0);
    for (rel, content) in cases {
        let on_disk: String = read_to_string(root.join(rel)).await.unwrap();
        assert_eq!(on_disk, content);
    }
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
}

#[test]
fn test_backward_compat_plain_text_and_detect_format_kind() {
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
    assert_eq!(normalize_line_endings("a\r\nb\rc\n"), "a\nb\nc\n");
    let cases: [(&str, FileFormatKind); 53] = [
        ("src/lib.rs", FileFormatKind::Rust),
        ("README.md", FileFormatKind::Markdown),
        ("AGENTS.md", FileFormatKind::Markdown),
        ("Agent.md", FileFormatKind::Markdown),
        ("SKILL.md", FileFormatKind::Markdown),
        ("doc.markdown", FileFormatKind::Markdown),
        ("page.mdx", FileFormatKind::Markdown),
        (".github/workflows/rust.yml", FileFormatKind::Yaml),
        ("compose.yaml", FileFormatKind::Yaml),
        (".prettierrc", FileFormatKind::Yaml),
        ("config.json", FileFormatKind::Json),
        ("tsconfig.json", FileFormatKind::Json),
        (".vscode/settings.json", FileFormatKind::Json),
        ("package.json", FileFormatKind::JsonStringify),
        ("package-lock.json", FileFormatKind::JsonStringify),
        ("composer.json", FileFormatKind::JsonStringify),
        ("settings.jsonc", FileFormatKind::Jsonc),
        ("custom.cjson", FileFormatKind::Jsonc),
        ("Cargo.toml", FileFormatKind::Toml),
        ("app.js", FileFormatKind::JavaScript),
        ("app.mjs", FileFormatKind::JavaScript),
        ("app.cjs", FileFormatKind::JavaScript),
        ("helper.wxs", FileFormatKind::JavaScript),
        ("app.ts", FileFormatKind::TypeScript),
        ("app.mts", FileFormatKind::TypeScript),
        ("app.cts", FileFormatKind::TypeScript),
        ("Card.tsx", FileFormatKind::TypeScriptReact),
        ("style.css", FileFormatKind::Css),
        ("style.wxss", FileFormatKind::Css),
        ("style.scss", FileFormatKind::Scss),
        ("style.less", FileFormatKind::Less),
        ("index.html", FileFormatKind::Html),
        ("index.htm", FileFormatKind::Html),
        ("App.vue", FileFormatKind::Vue),
        ("Makefile", FileFormatKind::PlainTextPreserveTabs),
        (".gitattributes", FileFormatKind::PlainTextPreserveTabs),
        ("LICENSE", FileFormatKind::PlainText),
        (".gitignore", FileFormatKind::PlainText),
        (".dockerignore", FileFormatKind::PlainText),
        (".editorconfig", FileFormatKind::PlainText),
        (".githooks/pre-commit", FileFormatKind::PlainText),
        ("scripts/run.sh", FileFormatKind::PlainText),
        ("scripts/run.bash", FileFormatKind::PlainText),
        ("scripts/run.zsh", FileFormatKind::PlainText),
        ("Dockerfile", FileFormatKind::PlainText),
        ("Dockerfile.dev", FileFormatKind::PlainText),
        (".env", FileFormatKind::PlainText),
        (".env.local", FileFormatKind::PlainText),
        ("notes.txt", FileFormatKind::PlainText),
        ("script.py", FileFormatKind::Unsupported),
        ("query.sql", FileFormatKind::Unsupported),
        ("App.jsx", FileFormatKind::Unsupported),
        ("Cargo.lock", FileFormatKind::Unsupported),
    ];
    for (path, expected_kind) in cases {
        assert_eq!(detect_format_kind(Path::new(path)), expected_kind);
    }
    assert_eq!(
        detect_format_kind(Path::new("logo.png")),
        FileFormatKind::Unsupported
    );
}

#[tokio::test]
async fn test_workspace_fmt_check_prettierignore_and_double_pass_idempotency() {
    let root: PathBuf = temp_dir().join("crate_cli_unified_fmt_test");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    create_dir_all(root.join("target")).await.unwrap();
    create_dir_all(root.join("ignored_dir")).await.unwrap();
    write(
        root.join(".prettierignore"),
        "ignored_dir/\nignored.js\n*.min.js\n!keep.min.js\n",
    )
    .await
    .unwrap();
    let md_path: PathBuf = root.join("README.md");
    let yml_path: PathBuf = root.join("ci.yml");
    let js_keep_path: PathBuf = root.join("keep.min.js");
    let js_ignored_path: PathBuf = root.join("ignored.js");
    let js_glob_ignored_path: PathBuf = root.join("vendor.min.js");
    let dir_ignored_path: PathBuf = root.join("ignored_dir/sub.md");
    let target_ignored_path: PathBuf = root.join("target/ignored.md");
    write(&md_path, "#  Hello\n* a\n").await.unwrap();
    write(&yml_path, "name: \"ci\"\nsteps:\n- run: ok\n")
        .await
        .unwrap();
    write(&js_keep_path, "const k = \"keep\"\n").await.unwrap();
    write(&js_ignored_path, "const x = \"ignored\"\n")
        .await
        .unwrap();
    write(&js_glob_ignored_path, "const v = \"vendor\"\n")
        .await
        .unwrap();
    write(&dir_ignored_path, "#   Skipped Dir\n").await.unwrap();
    write(&target_ignored_path, "#   Untouched\n")
        .await
        .unwrap();
    let check_err: Result<UnifiedFmtReport, io::Error> = format_workspace_text_files(&root, true);
    assert!(check_err.is_err());
    let pass1: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(pass1.get_files_scanned(), 3);
    assert_eq!(pass1.get_files_formatted(), 3);
    let pass2: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(pass2.get_files_scanned(), 3);
    assert_eq!(pass2.get_files_formatted(), 0);
    assert_eq!(read_to_string(&md_path).await.unwrap(), "# Hello\n\n- a\n");
    assert_eq!(
        read_to_string(&yml_path).await.unwrap(),
        "name: 'ci'\nsteps:\n  - run: ok\n"
    );
    assert_eq!(
        read_to_string(&js_keep_path).await.unwrap(),
        "const k = 'keep';\n"
    );
    assert_eq!(
        read_to_string(&js_ignored_path).await.unwrap(),
        "const x = \"ignored\"\n"
    );
    assert_eq!(
        read_to_string(&js_glob_ignored_path).await.unwrap(),
        "const v = \"vendor\"\n"
    );
    assert_eq!(
        read_to_string(&dir_ignored_path).await.unwrap(),
        "#   Skipped Dir\n"
    );
    assert_eq!(
        read_to_string(&target_ignored_path).await.unwrap(),
        "#   Untouched\n"
    );
    let check_ok: UnifiedFmtReport = format_workspace_text_files(&root, true).unwrap();
    assert_eq!(check_ok.get_files_formatted(), 0);
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
}

#[cfg(unix)]
#[tokio::test]
async fn test_workspace_walker_symlink_loop_safety() {
    let root: PathBuf = temp_dir().join("crate_cli_symlink_loop_test");
    let external: PathBuf = temp_dir().join("crate_cli_symlink_external_target.md");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    create_dir_all(root.join("nested")).await.unwrap();
    let real_md: PathBuf = root.join("real.md");
    write(&real_md, "#   Real\n").await.unwrap();
    write(&external, "#   External Untouched\n").await.unwrap();
    symlink(&root, root.join("nested/loop_back")).unwrap();
    symlink(&external, root.join("linked.md")).unwrap();
    let report: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(report.get_files_scanned(), 1);
    assert_eq!(report.get_files_formatted(), 1);
    assert_eq!(read_to_string(&real_md).await.unwrap(), "# Real\n");
    assert_eq!(
        read_to_string(&external).await.unwrap(),
        "#   External Untouched\n"
    );
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    let _: Result<(), io::Error> = fs::remove_file(&external).await;
}

#[test]
fn test_markdown_indented_code_blocks_do_not_corrupt_or_trigger_false_fence() {
    let input: &str = concat!(
        "Text\n\n",
        "    ***not emphasis***\n",
        "    ```\n",
        "    still code\n\n",
        "After ***bold italic*** and [a](https://x.com \"title\").\n",
    );
    let expected: &str = concat!(
        "Text\n\n",
        "    ***not emphasis***\n",
        "    ```\n",
        "    still code\n\n",
        "After **_bold italic_** and [a](https://x.com 'title').\n",
    );
    let formatted: String = format_markdown(input);
    assert_eq!(formatted, expected);
    assert_eq!(format_markdown(&formatted), formatted);
}

#[tokio::test]
async fn test_prettierignore_globstar_negation_pruning_and_single_file_format_path() {
    let root: PathBuf = temp_dir().join("crate_cli_prettierignore_edge_cases_test");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
    let dirs: [&str; 9] = [
        "sub",
        "a/c",
        "vendor",
        "x/vendor",
        "myvendor",
        "x/myvendor",
        "docs",
        "build",
        "out_custom",
    ];
    for rel_dir in dirs {
        create_dir_all(root.join(rel_dir)).await.unwrap();
    }
    write(
        root.join(".prettierignore"),
        concat!(
            "**/foo.js\n",
            "a/**/b.js\n",
            "**/vendor/\n",
            "*.md\n",
            "!docs/\n",
            "build/\n",
            "!build/keep.js\n",
            "out_custom/*\n",
            "!out_custom/keep.js\n",
        ),
    )
    .await
    .unwrap();
    let raw_js: &str = "const x = \"raw\"\n";
    let fmt_js: &str = "const x = 'raw';\n";
    let ignored_files: [&str; 8] = [
        "foo.js",
        "sub/foo.js",
        "a/b.js",
        "a/c/b.js",
        "vendor/a.js",
        "x/vendor/a.js",
        "build/keep.js",
        "out_custom/drop.js",
    ];
    let formatted_files: [&str; 7] = [
        "barfoo.js",
        "sub/barfoo.js",
        "a/xb.js",
        "a/c/xb.js",
        "myvendor/a.js",
        "x/myvendor/a.js",
        "out_custom/keep.js",
    ];
    for rel in ignored_files {
        write(root.join(rel), raw_js).await.unwrap();
    }
    for rel in formatted_files {
        write(root.join(rel), raw_js).await.unwrap();
    }
    let docs_md: PathBuf = root.join("docs/a.md");
    write(&docs_md, "#  Ignored Md\n").await.unwrap();
    let report: UnifiedFmtReport = format_workspace_text_files(&root, false).unwrap();
    assert_eq!(report.get_files_scanned(), formatted_files.len());
    assert_eq!(report.get_files_formatted(), formatted_files.len());
    for rel in ignored_files {
        assert_eq!(read_to_string(root.join(rel)).await.unwrap(), raw_js);
    }
    for rel in formatted_files {
        assert_eq!(read_to_string(root.join(rel)).await.unwrap(), fmt_js);
    }
    assert_eq!(read_to_string(&docs_md).await.unwrap(), "#  Ignored Md\n");
    let py_file: PathBuf = root.join("script.py");
    let py_raw: &str = "x=1+2\n";
    write(&py_file, py_raw).await.unwrap();
    assert!(format_path(&py_file).await.is_ok());
    assert_eq!(read_to_string(&py_file).await.unwrap(), py_raw);
    let single_md: PathBuf = root.join("single.md");
    write(&single_md, "#   Title\n").await.unwrap();
    assert!(format_path(&single_md).await.is_ok());
    assert_eq!(read_to_string(&single_md).await.unwrap(), "# Title\n");
    let _: Result<(), io::Error> = remove_dir_all(&root).await;
}

use super::*;

#[test]
fn resolves_a_path_under_the_root() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver {
        root: Path::new("/srv/public"),
        index: "index.html",
    };
    let file: Option<PathBuf> = resolver.resolve("assets/app.js");
    assert_eq!(file, Some(PathBuf::from("/srv/public/assets/app.js")));
}

#[test]
fn refuses_a_path_that_climbs_out_of_the_root() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver {
        root: Path::new("/srv/public"),
        index: "index.html",
    };
    let file: Option<PathBuf> = resolver.resolve("../../etc/passwd");
    assert!(file.is_none());
}

#[test]
fn refuses_a_segment_that_names_its_directory() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver::default();
    let file: Option<PathBuf> = resolver.resolve("assets/./app.js");
    assert!(file.is_none());
}

#[test]
fn refuses_a_segment_that_holds_a_backslash() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver::default();
    let file: Option<PathBuf> = resolver.resolve("assets\\app.js");
    assert!(file.is_none());
}

#[test]
fn refuses_an_empty_segment() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver::default();
    let file: Option<PathBuf> = resolver.resolve("assets//app.js");
    assert!(file.is_none());
}

#[test]
fn answers_a_directory_with_the_index_file() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver {
        root: Path::new("/srv/public"),
        index: "index.html",
    };
    let file: Option<PathBuf> = resolver.resolve("");
    assert_eq!(file, Some(PathBuf::from("/srv/public/index.html")));
}

#[test]
fn answers_a_directory_with_the_index_it_was_given() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver {
        root: Path::new("/srv/public"),
        index: "home.html",
    };
    let file: Option<PathBuf> = resolver.resolve("");
    assert_eq!(file, Some(PathBuf::from("/srv/public/home.html")));
}

#[test]
fn serves_the_current_directory_by_default() {
    let resolver: StaticFileResolver<'_> = StaticFileResolver::default();
    assert_eq!(resolver.get_root(), Path::new("."));
    assert_eq!(resolver.get_index(), "index.html");
}

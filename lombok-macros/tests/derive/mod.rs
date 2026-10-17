mod r#fn;
mod r#struct;

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use r#struct::*;

fn read_fn_rs() -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/generate/fn.rs");
    read_to_string(&path).expect("generate/fn.rs is readable")
}

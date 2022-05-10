use std::{env, path::Path};
use mdbook;

fn build_book() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    eprintln!("OUTDIR is {:?}", out_dir);
    let cargo_manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let book_toml_path=Path::new(&cargo_manifest_dir).join("book").join("book.toml");
    let book_path = Path::new(&cargo_manifest_dir).join("book");

    let mut config = mdbook::Config::from_disk(book_toml_path).unwrap();
    config.build.build_dir = Path::new(&out_dir).to_path_buf();
    // config.set("build.build_dir", Path::new(&out_dir)).unwrap();
    eprintln!("Building the book with config .... {:?}", &config);
    let book = mdbook::MDBook::load_with_config(book_path, config).unwrap();
    book.build().expect("Unable to build the book");
    println!("cargo:rerun-if-changed={}", Path::new(&cargo_manifest_dir).join("book").to_str().unwrap())
}

fn main() {
  build_book();
}
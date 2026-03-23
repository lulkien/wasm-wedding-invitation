use sass_rs::{compile_file, Options, OutputStyle};
use std::{fs, path::Path};

fn main() {
    // Re-run this build script whenever any SCSS file changes
    println!("cargo:rerun-if-changed=assets/scss/");

    let options = Options {
        output_style: OutputStyle::Expanded,
        precision: 10,
        ..Default::default()
    };

    let css = compile_file(Path::new("assets/scss/main.scss"), options)
        .expect("SCSS compilation failed — check assets/scss/ for errors");

    fs::write("assets/main.css", css)
        .expect("Failed to write assets/main.css");
}

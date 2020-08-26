use grass;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    //println!("cargo:rerun-if-changed=scss/*");

    let sass = grass::from_path("scss/main.scss", &grass::Options::default())
        .expect("Failed to build css");

    let mut output_buf = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("static/style/main.css")
            .expect("Could not access file"),
    );

    output_buf
        .write_all(sass.as_bytes())
        .expect("Could not write to file.");
}

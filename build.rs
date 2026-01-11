fn main() {
    // Tell Cargo that if the template files change, rerun this build script.
    println!("cargo:rerun-if-changed=src/templates");
    askama_axum::Config::new()
        .unwrap()
        .compile_template_dir("src/templates")
        .unwrap();
}


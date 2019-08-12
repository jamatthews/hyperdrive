extern crate bindgen;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

fn rbconfig(key: &str) -> String {
    let ruby = match env::var_os("RUBY") {
        Some(val) => val.to_os_string(),
        None => OsStr::new("ruby").to_os_string(),
    };
    let config = Command::new(ruby)
        .arg("-e")
        .arg(format!("print RbConfig::CONFIG['{}']", key))
        .output()
        .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    String::from_utf8(config.stdout).expect("RbConfig value not UTF-8!")
}

fn main() {
    // println!("cargo:rustc-link-search={}", rbconfig("libdir"));
    // println!("cargo:rustc-link-lib=dylib={}", rbconfig("RUBY_SO_NAME"));

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}", rbconfig("rubyhdrdir")))
        .clang_arg(format!("-I{}", rbconfig("rubyarchhdrdir")))
        .clang_arg(format!("-I{}", "../../ruby"))
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

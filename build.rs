extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=obs");

    let bindings_file = PathBuf::from(
        env::var("OUT_DIR").unwrap()
    ).join("libobs_extern.rs");

    let libobs_include_dir = env::var("LIBOBS_INCLUDE_DIR");
    
    match env::var("LIBOBS_LIB_DIR") {
        Ok(x) => println!("cargo:rustc-link-search={}", x),
        _ => {}
    }

    let stat = std::fs::metadata(&bindings_file);
    match stat.err() {
        Some(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                let mut builder = bindgen::Builder::default()
                    .emit_builtins()
                    .ctypes_prefix("libc")
                    .raw_line("extern crate libc;")
                    .header("src/c/libobs_wrapper.h")
                    .blacklist_type("max_align_t")
                    .blacklist_type("FP_NAN")
                    .blacklist_type("FP_INFINITE")
                    .blacklist_type("FP_ZERO")
                    .blacklist_type("FP_SUBNORMAL")
                    .blacklist_type("FP_NORMAL");

                match libobs_include_dir {
                    Ok(x) => {
                        builder = builder.clang_arg(format!("-I{}", x));
                        println!("cargo:include={}", x);
                    }
                    _ => {}
                }

                builder
                    .generate()
                    .expect("Unable to generate libobs bindings")
                    .write_to_file(bindings_file)
                    .expect("Couldn't write libobs bindings!");
            }
            _ => {}
        },
        None => {}
    }
}

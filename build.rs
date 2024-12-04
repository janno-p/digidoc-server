use std::path::PathBuf;

use cxx_build::CFG;

fn main() {
    let digidoc_headers = vec![PathBuf::from("/opt/digidoc/include/digidocpp")];
    CFG.exported_header_dirs.extend(digidoc_headers.iter().map(PathBuf::as_path));

    cxx_build::bridge("src/digidoc.rs")
        .file("src/digidoc.cpp")
        .compile("digidoc-server");

    println!("cargo:rustc-link-arg=-ldigidocpp");
    println!("cargo:rustc-link-search=/opt/digidoc/lib");

    println!("cargo:rerun-if-changed=include/digidoc.hpp");
    println!("cargo:rerun-if-changed=src/digidoc.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

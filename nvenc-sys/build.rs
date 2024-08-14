use std::env;
use std::path::PathBuf;

use bindgen::{builder, Bindings, Builder};

fn main() {
    let bindings = generate_bindings("include/nvenc.h")
        .allowlist_type(".*Nv.*")
        .allowlist_type(".*NV.*")
        .allowlist_var(".*NV.*")
        .allowlist_function(".*Nv.*")
        .blocklist_item(".*NV.*_GUID")
        .generate()
        .expect("failed to generate bindings");
    write_bindings(bindings, "nvenc.rs");
}

#[derive(Debug)]
pub struct FixBindgen {}

impl bindgen::callbacks::ParseCallbacks for FixBindgen {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.trim_start_matches("FIXBIND_").to_owned())
    }
}

fn generate_bindings(header: &str) -> Builder {
    println!("cargo:rerun-if-changed={}", header);

    let bindings = builder()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(FixBindgen {}))
        .opaque_type("_IMAGE_TLS_DIRECTORY64")
        .opaque_type("IMAGE_TLS_DIRECTORY64")
        .opaque_type("PIMAGE_TLS_DIRECTORY64")
        .opaque_type("IMAGE_TLS_DIRECTORY")
        .opaque_type("PIMAGE_TLS_DIRECTORY");
    return bindings;
}

fn write_bindings(bindings: Bindings, out: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(out))
        .expect("Couldn't write to file");
}

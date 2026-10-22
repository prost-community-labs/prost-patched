fn main() {
    prost_patched_build::Config::new()
        .prost_patched_path("::reexported_prost_patched::prost_patched")
        .prost_patched_types_path("::reexported_prost_patched::prost_patched_types")
        .compile_protos(&["protos/prost_path.proto"], &["protos"])
        .unwrap();
}

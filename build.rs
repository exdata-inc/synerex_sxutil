
fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .includes(&["../api/", "../nodeapi/", "../protobuf/src/"])
        // Inputs must reside in some of include paths.
        .input("../api/synerex.proto")
        .input("../nodeapi/nodeapi.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();
}

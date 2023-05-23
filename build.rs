use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

   tonic_build::configure()
           .protoc_arg("--experimental_allow_proto3_optional") // for older systems
           .build_client(true)
           .build_server(true)
           .file_descriptor_set_path(out_dir.join("store_descriptor.bin"))
           .out_dir("./src")
           .compile(&["../api/synerex.proto", "../nodeapi/nodeapi.proto"], &["../api/", "../nodeapi/", "../protobuf/src/"])?;

   Ok(())
}

// fn main() {
//     protobuf_codegen::Codegen::new()
//         .protoc()
//         .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
//         .includes(&["../api/", "../nodeapi/", "../protobuf/src/"])
//         // Inputs must reside in some of include paths.
//         .input("../api/synerex.proto")
//         .input("../nodeapi/nodeapi.proto")
//         // Specify output directory relative to Cargo output directory.
//         .cargo_out_dir("protos")
//         .run_from_script();
// }
use std::env;
use protoc_bin_vendored::protoc_bin_path;
use protobuf_codegen::Codegen;


fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let protoc_path = protoc_bin_path().unwrap();

    // protoc_rust::Codegen::new()
    // .out_dir(out_dir)
    // .inputs(&["protos/operation.proto"])
    // .include("protos")
    // .run()
    // .expect("Running protoc failed.");

    Codegen::new()
        .protoc()
        .protoc_path(&protoc_path)
        .includes(&["proto"])
        .input("proto/operation.proto")
        .cargo_out_dir("proto")
        .run_from_script();

    // Codegen::new()
    //     .out_dir(out_dir)
    //     .inputs(&["src/protos/operation.proto"])
    //     .include("src/protos")
    //     .run()
    //     .expect("Codegen failed.");


}

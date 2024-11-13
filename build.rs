use protobuf_codegen_pure::Codegen;

fn main() {
    println!("cargo:warning=Hi!");

    Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/operation.proto"])
        .include("protos")
        .run()
        .expect("Codegen failed.");
}

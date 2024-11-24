use std::env;
use std::path::PathBuf;

fn main() {
    let proto_root = "protos";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let protos = ["protos/krec.proto"];

    let includes = [proto_root];

    std::fs::create_dir_all(out_dir.join("protos")).expect("Failed to create protos directory");

    tonic_build::configure()
        .build_server(true)
        .out_dir(out_dir.join("protos"))
        .compile_protos(&protos, &includes)
        .expect("Failed to compile protos");

    for proto in protos {
        println!("cargo:rerun-if-changed={}/protos/{}", proto_root, proto);
    }
    println!("cargo:rerun-if-changed={}", proto_root);
}

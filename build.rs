use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("proto");
    fs::create_dir_all(&dest_path).unwrap();

    let proto_src_files = glob_simple("./proto/*.proto");

    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: dest_path.to_str().unwrap(),
        input: &proto_src_files
            .iter()
            .map(|proto_file| proto_file.as_ref())
            .collect::<Vec<&str>>(),
        includes: &["./proto"],
        rust_protobuf: true, // also generate protobuf messages, not just services
        ..Default::default()
    })
    .expect("protoc");

    let mod_file_content = proto_src_files
        .iter()
        .map(|proto_file| {
            let proto_path = Path::new(proto_file);
            let mut mods = vec![format!(
                "pub mod {};",
                proto_path.file_stem().unwrap().to_str().unwrap()
            )];

            if proto_file.ends_with("Service.proto") {
                mods.push(format!(
                    "pub mod {}_grpc;",
                    proto_path.file_stem().unwrap().to_str().unwrap()
                ))
            }

            mods
        })
        .flatten()
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(dest_path.join("mod.rs"), mod_file_content.as_bytes())
        .expect("failed to write mod.rs");
}

fn glob_simple(pattern: &str) -> Vec<String> {
    glob::glob(pattern)
        .expect("glob")
        .map(|g| {
            g.expect("item")
                .as_path()
                .to_str()
                .expect("utf-8")
                .to_owned()
        })
        .collect()
}

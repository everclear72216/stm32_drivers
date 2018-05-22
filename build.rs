
use std::fs;
use std::env;
use std::path::PathBuf;

fn main() {

    let link_files = vec![
        "link/link.x",
        "link/memory.x",
        "link/device.x",
    ];

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);

    for file_path in link_files {
        {
            let file_path = PathBuf::from(file_path);
            let file_name = file_path.file_name().unwrap();
            let out_path = PathBuf::from(out_dir.join(file_name));

            fs::copy(&file_path, &out_path).unwrap();
        }

        println!("cargo:rerun-if-changed={}", file_path);
    }

    println!("cargo:rerun-if-changed=build.rs");

    ()
}

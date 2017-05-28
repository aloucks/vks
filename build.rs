use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=vulkan-1");

        if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
            let mut vulkan_sdk_path = PathBuf::from(vulkan_sdk);

            if target.contains("x86_64") {
                vulkan_sdk_path.push("Lib");
            }
            else {
                vulkan_sdk_path.push("Lib32");
            }

            println!("cargo:rustc-link-search=native={}", vulkan_sdk_path.to_str().unwrap());
        }
    }
    else {
        println!("cargo:rustc-link-lib=dylib=vulkan");
    }
}

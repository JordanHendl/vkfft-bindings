use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn ensure_submodule(path: &Path) {
    // Heuristic: if the directory exists and contains vkFFT.h, assume initialized.
    if path.join("vkFFT").join("vkFFT.h").is_file() {
        return;
    }

    // Run: git submodule update --init --recursive vendor/VkFFT
    let status = Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--recursive",
            "vendor/VkFFT",
        ])
        .current_dir(env::var("CARGO_MANIFEST_DIR").unwrap())
        .status()
        .expect("failed to execute git to init/update submodule");

    if !status.success() {
        panic!("git submodule update --init --recursive vendor/VkFFT failed");
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/shim.cpp");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vkfft_dir = manifest_dir.join("vendor").join("VkFFT");
    let vkfft_include = vkfft_dir.join("vkFFT"); // contains vkFFT.h

    ensure_submodule(&vkfft_dir);

    println!("cargo:rerun-if-changed=wrapper.h");
    println!(
        "cargo:rerun-if-changed={}",
        vkfft_include.join("vkFFT.h").display()
    );
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");

    // Generate bindings.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let glslang_include = find_glslang_c_interface_dir().expect(
        "Could not find glslang_c_interface.h. Install glslang dev headers or set VULKAN_SDK.",
    );

    println!(
        "cargo:rerun-if-changed={}",
        vkfft_dir
            .join("vkFFT/vkFFT_AppManagement/vkFFT_InitializeApp.h")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        vkfft_dir
            .join("vkFFT/vkFFT_AppManagement/vkFFT_RunApp.h")
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        vkfft_dir
            .join("vkFFT/vkFFT_AppManagement/vkFFT_DeleteApp.h")
            .display()
    );

    cc::Build::new()
        .cpp(true)
        .file(manifest_dir.join("src/shim.cpp"))
        .include(&vkfft_include) // for vkFFT.h
        .include(&vkfft_dir) // for vkFFT/...
        .include(&glslang_include) // for glslang_c_interface.h
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-w") // GCC/Clang: suppress all warnings
        .flag_if_supported("-Wno-everything") // Clang
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-old-style-cast")
        .flag_if_supported("-Wno-shadow")
        .compile("vkfft_shim");

    println!("cargo:rustc-link-lib=static=vkfft_shim");
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("wrapper.h").to_string_lossy())
        .clang_arg(format!("-I{}", vkfft_include.display()))
        .clang_arg(format!("-I{}", vkfft_dir.display()))
        .clang_arg(format!("-I{}", vkfft_include.display()))
        .clang_arg(format!("-I{}", glslang_include.display()))
        .clang_arg(format!("-I{}", glslang_include.display()))
        .allowlist_function("initializeVkFFT")
        .allowlist_function("deleteVkFFT")
        .allowlist_function("VkFFTAppend")
        .allowlist_function("vkfft_.*")
        .allowlist_type("VkFFT(Application|Configuration|LaunchParams|Result)")
        .allowlist_type("Vk.*")
        .allowlist_var("VKFFT_.*")
        .allowlist_var("VK_.*")
        // If you want to keep it broader:
        .allowlist_function("VkFFT.*")
        .use_core()
        .clang_arg("-DVKFFT_BACKEND=0")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .generate_comments(false)
        .layout_tests(false)
        .generate()
        .expect("bindgen: failed to generate VkFFT bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("bindgen: failed to write bindings.rs");
}

fn find_glslang_c_interface_dir() -> Option<PathBuf> {
    // VkFFT includes: "glslang_c_interface.h" (no path),
    // so we must add an include dir that directly contains that file.
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(sdk) = env::var("VULKAN_SDK") {
        // Vulkan SDK layout:
        //   $VULKAN_SDK/Include/glslang/Include/glslang_c_interface.h
        candidates.push(
            PathBuf::from(&sdk)
                .join("Include")
                .join("glslang")
                .join("Include"),
        );
    }

    // Common system layouts (Debian/Ubuntu/etc):
    candidates.push(PathBuf::from("/usr/include/glslang/Include"));
    candidates.push(PathBuf::from("/usr/local/include/glslang/Include"));

    for dir in candidates {
        if dir.join("glslang_c_interface.h").is_file() {
            return Some(dir);
        }
    }
    None
}

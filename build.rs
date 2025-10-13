// Build script for transmutation
// Handles C++ compilation and linking for docling-parse FFI

use std::path::Path;

fn main() {
    // Only build C++ if docling-ffi feature is enabled
    #[cfg(feature = "docling-ffi")]
    {
        println!("cargo:rerun-if-changed=cpp/docling_ffi.cpp");
        println!("cargo:rerun-if-changed=cpp/docling_ffi.h");
        println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
        
        // Platform-specific library paths and names
        #[cfg(target_os = "windows")]
        {
            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") { "x86" } else { "ARM" };
            let build_dir = format!("build_windows_{}", arch);
            let lib_path = Path::new(&format!("{}/Release/docling_ffi.lib", build_dir));
            let libs_path = Path::new(&format!("libs/docling-ffi-windows_{}.lib", arch));
            
            // Check if library exists in either location
            if !lib_path.exists() && !libs_path.exists() {
                eprintln!("========================================");
                eprintln!("ERROR: docling_ffi.lib not found!");
                eprintln!("========================================");
                eprintln!("Please run: .\\build_cpp.ps1");
                eprintln!("This will build the C++ library for Windows.");
                eprintln!("Or use WSL for full functionality:");
                eprintln!("  wsl -d Ubuntu-24.04 -- bash -c 'cd /mnt/f/Node/hivellm/transmutation && ./build_cpp.sh'");
                eprintln!("========================================");
                panic!("C++ library not built");
            }
            
            println!("cargo:rustc-link-search=native=libs");
            println!("cargo:rustc-link-search=native={}/Release", build_dir);
            println!("cargo:rustc-link-search=native=docling-parse/build_windows_{}_docling/Release", arch);
            println!("cargo:rustc-link-lib=dylib=docling_ffi");
            
            // Copy DLL to target directory for runtime
            let dll_src = if Path::new(&format!("libs/docling-ffi-windows_{}.dll", arch)).exists() {
                Path::new(&format!("libs/docling-ffi-windows_{}.dll", arch))
            } else {
                Path::new(&format!("{}/Release/docling_ffi.dll", build_dir))
            };
            
            let target_dir = std::env::var("OUT_DIR").unwrap();
            let dll_dst = Path::new(&target_dir).join("../../../docling_ffi.dll");
            if dll_src.exists() {
                let _ = std::fs::copy(dll_src, dll_dst);
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") { "x86" } 
                      else if cfg!(target_arch = "aarch64") { "ARM" }
                      else { "x86" };
            let build_dir = format!("build_linux_{}", arch);
            
            // Try multiple locations for the library
            println!("cargo:rustc-link-search=native=libs");  // Pre-built library location
            println!("cargo:rustc-link-search=native={}", build_dir);
            println!("cargo:rustc-link-search=native=cpp/build");  // Legacy location
            println!("cargo:rustc-link-search=native=docling-parse/build_linux_{}_docling", arch);
            println!("cargo:rustc-link-lib=dylib=docling_ffi");
            
            // Add rpath for runtime library loading
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../libs");
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../{}", build_dir);
        }
        
        #[cfg(target_os = "macos")]
        {
            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") { "x86" } 
                      else if cfg!(target_arch = "aarch64") { "ARM" }
                      else { "ARM" };  // Default to ARM for Apple Silicon
            let build_dir = format!("build_macos_{}", arch);
            
            println!("cargo:rustc-link-search=native=libs");
            println!("cargo:rustc-link-search=native={}", build_dir);
            println!("cargo:rustc-link-search=native=cpp/build");  // Legacy location
            println!("cargo:rustc-link-search=native=docling-parse/build_macos_{}_docling", arch);
            println!("cargo:rustc-link-lib=dylib=docling_ffi");
            
            // Add rpath for runtime library loading on macOS
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../libs");
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../{}", build_dir);
        }
        
        // Link against standard C++ library
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}


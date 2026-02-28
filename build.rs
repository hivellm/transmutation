// Build script for transmutation
// Handles C++ compilation and linking for docling-parse FFI

use std::process::Command;

fn main() {
    // Embed icon and metadata into Windows executable (only for CLI binary builds)
    // When used as a library dependency, skip resource compilation to avoid
    // duplicate resource errors (see: https://github.com/hivellm/transmutation/issues/3)
    #[cfg(all(target_os = "windows", feature = "cli"))]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Transmutation");
        res.set(
            "FileDescription",
            "High-performance document conversion engine for AI/LLM embeddings",
        );
        res.set("CompanyName", "HiveLLM Team");
        res.set("LegalCopyright", "Copyright (c) 2025 HiveLLM Team");
        res.set("OriginalFilename", "transmutation.exe");
        if let Err(e) = res.compile() {
            eprintln!("Warning: Failed to compile Windows resources: {e}");
        }
    }

    // Check for optional external dependencies
    check_external_dependencies();

    // Only build C++ if docling-ffi feature is enabled
    #[cfg(feature = "docling-ffi")]
    {
        println!("cargo:rerun-if-changed=cpp/docling_ffi.cpp");
        println!("cargo:rerun-if-changed=cpp/docling_ffi.h");
        println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");

        // Platform-specific library paths and names
        #[cfg(target_os = "windows")]
        {
            use std::path::Path;

            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") {
                "x86"
            } else {
                "ARM"
            };
            let build_dir = format!("build_windows_{arch}");
            let lib_path_str = format!("{build_dir}/Release/docling_ffi.lib");
            let libs_path_str = format!("libs/docling-ffi-windows_{arch}.lib");
            let lib_path = Path::new(&lib_path_str);
            let libs_path = Path::new(&libs_path_str);

            // Check if library exists in either location
            if !lib_path.exists() && !libs_path.exists() {
                eprintln!("========================================");
                eprintln!("ERROR: docling_ffi.lib not found!");
                eprintln!("========================================");
                eprintln!("Please run: .\\build_cpp.ps1");
                eprintln!("This will build the C++ library for Windows.");
                eprintln!("Or use WSL for full functionality:");
                eprintln!(
                    "  wsl -d Ubuntu-24.04 -- bash -c 'cd /mnt/f/Node/hivellm/transmutation && ./build_cpp.sh'"
                );
                eprintln!("========================================");
                panic!("C++ library not built");
            }

            println!("cargo:rustc-link-search=native=libs");
            println!("cargo:rustc-link-search=native={build_dir}/Release");
            println!(
                "cargo:rustc-link-search=native=docling-parse/build_windows_{arch}_docling/Release"
            );
            println!("cargo:rustc-link-lib=dylib=docling_ffi");

            // Copy DLL to target directory for runtime
            let dll_libs_path = format!("libs/docling-ffi-windows_{arch}.dll");
            let dll_build_path = format!("{build_dir}/Release/docling_ffi.dll");
            let dll_src_str = if Path::new(&dll_libs_path).exists() {
                dll_libs_path
            } else {
                dll_build_path
            };
            let dll_src = Path::new(&dll_src_str);

            let target_dir = std::env::var("OUT_DIR").unwrap();
            let dll_dst = Path::new(&target_dir).join("../../../docling_ffi.dll");
            if dll_src.exists() {
                let _ = std::fs::copy(dll_src, dll_dst);
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::path::Path;

            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") {
                "x86"
            } else if cfg!(target_arch = "aarch64") {
                "ARM"
            } else {
                "x86"
            };
            let build_dir = format!("build_linux_{arch}");

            // Check if library exists in any known location
            let lib_build = format!("{build_dir}/libdocling_ffi.so");
            let lib_libs = format!("libs/libdocling_ffi.so");
            let lib_legacy = "cpp/build/libdocling_ffi.so";
            let lib_docling = format!("docling-parse/build_linux_{arch}_docling/libdocling_ffi.so");

            if !Path::new(&lib_build).exists()
                && !Path::new(&lib_libs).exists()
                && !Path::new(lib_legacy).exists()
                && !Path::new(&lib_docling).exists()
            {
                eprintln!("========================================");
                eprintln!("ERROR: libdocling_ffi.so not found!");
                eprintln!("========================================");
                eprintln!("The 'docling-ffi' feature requires a pre-built C++ library.");
                eprintln!("Please build it first: ./scripts/build_cpp.sh");
                eprintln!();
                eprintln!("If installing via 'cargo install', note that docling-ffi");
                eprintln!("requires cloning the repo and building the C++ library locally.");
                eprintln!("See: https://github.com/hivellm/transmutation/blob/main/docs/FFI.md");
                eprintln!("========================================");
                panic!("C++ library not built - see instructions above");
            }

            // Library search paths
            println!("cargo:rustc-link-search=native=libs"); // Pre-built library location
            println!("cargo:rustc-link-search=native={build_dir}");
            println!("cargo:rustc-link-search=native=cpp/build"); // Legacy location
            println!("cargo:rustc-link-search=native=docling-parse/build_linux_{arch}_docling");
            println!("cargo:rustc-link-lib=dylib=docling_ffi");

            // Add rpath for runtime library loading
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../libs");
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../{build_dir}");
        }

        #[cfg(target_os = "macos")]
        {
            use std::path::Path;

            // Detect architecture
            let arch = if cfg!(target_arch = "x86_64") {
                "x86"
            } else if cfg!(target_arch = "aarch64") {
                "ARM"
            } else {
                "ARM"
            }; // Default to ARM for Apple Silicon
            let build_dir = format!("build_macos_{arch}");

            // Check if library exists in any known location
            let lib_build = format!("{build_dir}/libdocling_ffi.dylib");
            let lib_libs = format!("libs/libdocling_ffi.dylib");
            let lib_legacy = "cpp/build/libdocling_ffi.dylib";
            let lib_docling =
                format!("docling-parse/build_macos_{arch}_docling/libdocling_ffi.dylib");

            if !Path::new(&lib_build).exists()
                && !Path::new(&lib_libs).exists()
                && !Path::new(lib_legacy).exists()
                && !Path::new(&lib_docling).exists()
            {
                eprintln!("========================================");
                eprintln!("ERROR: libdocling_ffi.dylib not found!");
                eprintln!("========================================");
                eprintln!("The 'docling-ffi' feature requires a pre-built C++ library.");
                eprintln!("Please build it first: ./scripts/build_cpp.sh");
                eprintln!();
                eprintln!("If installing via 'cargo install', note that docling-ffi");
                eprintln!("requires cloning the repo and building the C++ library locally.");
                eprintln!("See: https://github.com/hivellm/transmutation/blob/main/docs/FFI.md");
                eprintln!("========================================");
                panic!("C++ library not built - see instructions above");
            }

            println!("cargo:rustc-link-search=native=libs");
            println!("cargo:rustc-link-search=native={build_dir}");
            println!("cargo:rustc-link-search=native=cpp/build"); // Legacy location
            println!("cargo:rustc-link-search=native=docling-parse/build_macos_{arch}_docling");
            println!("cargo:rustc-link-lib=dylib=docling_ffi");

            // Add rpath for runtime library loading on macOS
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../libs");
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../{build_dir}");
        }

        // Link against standard C++ library
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
    }
}

/// Check for optional external dependencies and provide helpful messages
fn check_external_dependencies() {
    #[allow(unused_mut)]
    let mut warnings: Vec<(&str, &str, String)> = Vec::new();

    // Check pdf-to-image feature dependencies
    #[cfg(feature = "pdf-to-image")]
    {
        if !command_exists("pdftoppm") {
            warnings.push((
                "pdftoppm (poppler-utils)",
                "PDF → Image conversion",
                get_install_command("poppler-utils"),
            ));
        }
    }

    // Check office feature dependencies for image conversion
    #[cfg(all(feature = "office", feature = "pdf-to-image"))]
    {
        if !command_exists("libreoffice") && !command_exists("soffice") {
            warnings.push((
                "LibreOffice",
                "DOCX/PPTX → Image conversion",
                get_install_command("libreoffice"),
            ));
        }
    }

    // Check tesseract feature dependencies
    #[cfg(feature = "tesseract")]
    {
        if !command_exists("tesseract") {
            warnings.push((
                "Tesseract OCR",
                "Image → Text (OCR)",
                get_install_command("tesseract"),
            ));
        }
    }

    // Check ffmpeg feature dependencies
    #[cfg(any(feature = "audio", feature = "video"))]
    {
        if !command_exists("ffmpeg") {
            warnings.push((
                "FFmpeg",
                "Audio/Video processing",
                get_install_command("ffmpeg"),
            ));
        }
    }

    // Print warnings if any dependencies are missing
    if !warnings.is_empty() {
        eprintln!();
        eprintln!("╔════════════════════════════════════════════════════════════╗");
        eprintln!("║  ⚠️  Optional External Dependencies Missing             ║");
        eprintln!("╚════════════════════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("Transmutation will compile, but some features won't work:");
        eprintln!();

        for (tool, feature, install_cmd) in &warnings {
            eprintln!("  ❌ {tool}: {feature}");
            eprintln!("     Install: {install_cmd}");
        }

        eprintln!();
        eprintln!("📖 For detailed installation instructions:");
        eprintln!("   https://github.com/yourusername/transmutation/blob/main/install/README.md");
        eprintln!();
        eprintln!("💡 Quick install (all dependencies):");
        eprintln!("{}", get_quick_install_all());
        eprintln!();
    }
}

/// Check if a command exists in PATH
#[allow(dead_code)]
fn command_exists(cmd: &str) -> bool {
    Command::new(if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    })
    .arg(cmd)
    .output()
    .map(|output| output.status.success())
    .unwrap_or(false)
}

/// Get platform-specific install command for a tool
#[allow(dead_code)]
fn get_install_command(tool: &str) -> String {
    #[cfg(target_os = "linux")]
    {
        match tool {
            "poppler-utils" => "sudo apt-get install poppler-utils".to_string(),
            "libreoffice" => "sudo apt-get install libreoffice".to_string(),
            "tesseract" => "sudo apt-get install tesseract-ocr".to_string(),
            "ffmpeg" => "sudo apt-get install ffmpeg".to_string(),
            _ => format!("sudo apt-get install {tool}"),
        }
    }

    #[cfg(target_os = "macos")]
    {
        match tool {
            "poppler-utils" => "brew install poppler".to_string(),
            "libreoffice" => "brew install --cask libreoffice".to_string(),
            "tesseract" => "brew install tesseract".to_string(),
            "ffmpeg" => "brew install ffmpeg".to_string(),
            _ => format!("brew install {tool}"),
        }
    }

    #[cfg(target_os = "windows")]
    {
        match tool {
            "poppler-utils" => "choco install poppler".to_string(),
            "libreoffice" => "choco install libreoffice".to_string(),
            "tesseract" => "choco install tesseract".to_string(),
            "ffmpeg" => "choco install ffmpeg".to_string(),
            _ => format!("choco install {tool}"),
        }
    }
}

/// Get platform-specific command to install all dependencies
fn get_quick_install_all() -> &'static str {
    #[cfg(target_os = "linux")]
    {
        "   ./install/install-deps-linux.sh"
    }

    #[cfg(target_os = "macos")]
    {
        "   ./install/install-deps-macos.sh"
    }

    #[cfg(target_os = "windows")]
    {
        "   .\\install\\install-deps-windows.ps1 (or .bat)"
    }
}

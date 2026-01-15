use core_plan::Step;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use zip::read::ZipArchive;

pub fn generate_plan(
    name: &str,
    zip_data: &[u8],
    replacements: &HashMap<String, String>,
) -> Vec<Step> {
    let mut steps = Vec::new();
    let cursor = std::io::Cursor::new(zip_data);
    let mut archive = ZipArchive::new(cursor).expect("Failed to read skeleton zip");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to read zip file");
        let path = file.mangled_name(); // Sanitize path

        // Strip the first component (the root folder)
        let path_str = path.to_string_lossy();
        let components: Vec<_> = Path::new(&*path_str).components().collect();
        if components.len() < 2 {
            continue;
        }

        // Reassemble path without the first component
        let mut rel_path: std::path::PathBuf = components.into_iter().skip(1).collect();

        // OS-specific localized properties logic
        let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let current_os = std::env::consts::OS; // "linux", "macos", "windows"

        let target_suffix = match current_os {
            "macos" => ".mac",
            "windows" => ".windows",
            "linux" => ".linux",
            _ => "",
        };

        if file_name.starts_with("local.properties.") && file_name != "local.properties.example" {
            if !target_suffix.is_empty() && file_name.ends_with(target_suffix) {
                // Rename to local.properties
                // Replace the file name in rel_path
                rel_path.set_file_name("local.properties");
            } else {
                // Skip other OS files
                continue;
            }
        }

        // Path rewriting for Java package structure
        // If path starts with src/main/java/com/example/skeleton, rewrite it.
        // We rely on replacements containing <PACKAGE_DIR>
        let path_str_chk = rel_path.to_string_lossy();
        if path_str_chk.contains("src/main/java/com/example/skeleton") {
            if let Some(pkg_dir) = replacements.get("<PACKAGE_DIR>") {
                // simple string replace for the directory part
                // Note: using / for replacement might be OS sensitive, but PathBuf handles it?
                // pkg_dir is calculated using /.
                // Let's coerce to OS separator if needed or just replace string.
                let new_path_str = path_str_chk.replace(
                    "src/main/java/com/example/skeleton",
                    &format!("src/main/java/{}", pkg_dir),
                );
                rel_path = std::path::PathBuf::from(new_path_str);
            }
        }

        // Prepend the project name as the root directory
        let dest_path = Path::new(name).join(rel_path);
        let dest_path_str = dest_path.to_string_lossy().into_owned();

        if file.is_dir() {
            steps.push(Step::Mkdir {
                path: dest_path_str,
            });
        } else {
            let mut content_bytes = Vec::new();
            file.read_to_end(&mut content_bytes)
                .expect("Failed to read file content");

            if is_text_file(&dest_path_str) {
                // Try treating as UTF-8 for templating
                if let Ok(content_str) = String::from_utf8(content_bytes.clone()) {
                    let mut replaced = content_str;
                    for (k, v) in replacements {
                        replaced = replaced.replace(k, v);
                    }

                    steps.push(Step::WriteFile {
                        path: dest_path_str,
                        content: replaced,
                    });
                    continue;
                }
            }

            // Fallback to binary write
            steps.push(Step::WriteBytes {
                path: dest_path_str,
                content: content_bytes,
            });
        }
    }

    steps
}

fn is_text_file(path: &str) -> bool {
    let p = Path::new(path);
    if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
        matches!(
            ext,
            "md" | "txt"
                | "gradle"
                | "properties"
                | "java"
                | "json"
                | "xml"
                | "yml"
                | "yaml"
                | "toml"
                | "gitignore"
        )
    } else {
        path.ends_with(".gitignore")
    }
}

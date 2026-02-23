use std::fs;
use std::path::{Path, PathBuf};

pub struct ResolvedArtifact {
    pub source_path: PathBuf,
    pub target_file_name: String,
}

pub fn resolve_mod_artifact(mod_dir: &Path) -> ResolvedArtifact {
    let build_libs = mod_dir.join("build").join("libs");
    let preferred_name = jar_name_from_manifest(mod_dir);
    let guessed_name = jar_name_from_gradle_properties(mod_dir);

    let source_path = if let Some(name) = preferred_name.as_ref() {
        let expected = build_libs.join(name);
        if expected.is_file() {
            expected
        } else if let Some(candidate) = find_existing_artifact(&build_libs) {
            candidate
        } else {
            expected
        }
    } else if let Some(candidate) = find_existing_artifact(&build_libs) {
        candidate
    } else {
        build_libs.join(&guessed_name)
    };

    let target_file_name = preferred_name.unwrap_or_else(|| {
        source_path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or(guessed_name)
    });

    ResolvedArtifact {
        source_path,
        target_file_name,
    }
}

fn find_existing_artifact(build_libs: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(build_libs).ok()?;
    let mut candidates = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("jar") {
            continue;
        }

        let file_name = path
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default();

        if file_name.contains("-sources")
            || file_name.contains("-javadoc")
            || file_name.contains("-plain")
        {
            continue;
        }

        candidates.push(path);
    }

    candidates.sort();
    candidates.pop()
}

fn jar_name_from_manifest(mod_dir: &Path) -> Option<String> {
    let path = mod_dir
        .join("src")
        .join("main")
        .join("resources")
        .join("manifest.json");
    let content = fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;

    let name = json.get("Name").and_then(|v| v.as_str())?;
    let version = json.get("Version").and_then(|v| v.as_str())?;
    Some(format!("{name}-{version}.jar"))
}

fn jar_name_from_gradle_properties(mod_dir: &Path) -> String {
    let gradle_props = mod_dir.join("gradle.properties");
    if let Ok(content) = fs::read_to_string(gradle_props) {
        let mut name = "mod".to_string();
        let mut version = "1.0.0".to_string();
        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("mod_name=") {
                name = stripped.trim().to_string();
            }
            if let Some(stripped) = line.strip_prefix("mod_version=") {
                version = stripped.trim().to_string();
            }
        }
        return format!("{name}-{version}.jar");
    }

    "mod.jar".to_string()
}

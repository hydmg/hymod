pub const BUILD_GRADLE: &str = r#"apply from: 'config/dependencies.gradle' // Loads your constants

plugins {
    id 'java'
}

java {
    toolchain { languageVersion = JavaLanguageVersion.of(25) }
}

dependencies {
    compileOnly project.hytaleDependency // Links the API without bundling
}
"#;

pub const DEPS_GRADLE: &str = r#"// Isolated file for easy updates on Hytale release
ext {
    hytaleVersion = "1.0.0-SNAPSHOT" 
    hytaleDependency = "io.hytale:hytale-api:${hytaleVersion}"
    // If local path is needed:
    // hytaleDependency = files("libs/hytale-sdk.jar")
}
"#;

pub fn mod_json(id: &str, main_class: &str) -> String {
    format!(
        r#"{{
  "id": "{}",
  "mainClass": "{}"
}}
"#,
        id, main_class
    )
}

pub fn settings_gradle(name: &str) -> String {
    format!("rootProject.name = '{}'\n", name)
}

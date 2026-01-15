# Hytale Mod Skeleton

A basic skeleton for creating Hytale mods.

## Usage

This project is a template. Before starting, you need to configure it by replacing the placeholders.

### Quick Start

Run the included setup script to automatically configure your mod:

```bash
./setup.sh
```

Follow the interactive prompts to enter your mod details.

### Manual Configuration

If you prefer to configure it manually, replace the following placeholders in the project files:

| Placeholder | Description | File(s) |
| :--- | :--- | :--- |
| `<MOD_NAME>` | The human-readable name of your mod (e.g., "My Mod"). | `gradle.properties`, `manifest.json` |
| `<MOD_ID>` | The unique ID of your mod (e.g., "mymod"). | `settings.gradle` |
| `<GROUP_ID>` | The Maven group ID (e.g., "com.example"). | `gradle.properties`, `manifest.json` |
| `<VERSION>` | The initial version of your mod (e.g., "1.0.0"). | `gradle.properties`, `manifest.json` |
| `<DESCRIPTION>` | A brief description of your mod. | `manifest.json` |
| `<AUTHOR>` | The author's name. | `manifest.json` |
| `<MAIN_CLASS>` | The fully qualified name of your main class (e.g., "com.example.mymod.Main"). | `build.gradle`, `manifest.json` |
| `<PACKAGE>` | The package name for your Java sources (e.g., "com.example.mymod"). | `Main.java` |
| `<USERNAME>` | The system username for the Hytale installation path. | `local.properties.*` |

### Important Note

After configuration, make sure to move `src/main/java/com/example/skeleton/Main.java` to the correct directory matching your new package name.

## Building

Once configured, you can build the mod using Gradle:

```bash
./gradlew build
```

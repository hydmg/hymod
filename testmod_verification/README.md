# Hytale Mod Skeleton

A clean, generic starting point for creating Hytale server-side mods.

## Project Structure

- `src/main/java/com/example/skeleton/Main.java`: The main entry point of the mod.
- `build.gradle`: Gradle build configuration with dynamic Hytale path resolution.

## Setup & Configuration

This project requires the Hytale client to be installed so it can reference the `HytaleServer.jar`.

### 1. Configure Hytale Path

The build script tries to automatically detect your Hytale installation. However, for a consistent environment, it is recommended to explicitly set the path using a `local.properties` file.

We have provided templates for common operating systems. **Rename the file matching your OS to `local.properties`**:

*   **Windows**: Rename `local.properties.windows` -> `local.properties`
*   **Mac**: Rename `local.properties.mac` -> `local.properties`
*   **Linux**: Rename `local.properties.linux` -> `local.properties`

*Note: `local.properties` is git-ignored so your local path won't be committed.*

## Building

To build the mod, open a terminal in the project directory and run:

### Windows
```cmd
gradlew build
```

### Mac / Linux
```bash
./gradlew build
```

The compiled mod JAR will be generated in `build/libs/`.

## Running
This is a server-side mod. To use it:
1.  Copy the generated JAR from `build/libs/` to your Hytale server's `mods` folder.
2.  Start the Hytale Server.
3.  You should see "Hello, Hytale!" printed in the server console logs.

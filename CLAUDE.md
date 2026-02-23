# Hymod Agent Instructions

## Command policy
- Use `hymod` commands for mod workflows.
- Do not run Gradle commands directly for normal build/dev/deploy tasks.
- Prefer project-root execution unless a command explicitly needs `--path`.

## Use these `hymod` commands instead of Gradle
- Build mod artifact: `hymod build`
- Build release artifact: `hymod build --release`
- Local link to a configured local server: `hymod link [server_name]`
- Dev loop (build + link + restart): `hymod dev [target] [--path <mod_dir>]`
- Remote deploy (build + upload): `hymod deploy [server_name] [--transport rsync|scp] [--path <mod_dir>]`
- Preview deploy plan without executing: `hymod deploy [server_name] --dry-run`

## Server/config helpers
- List servers: `hymod server list`
- Add server: `hymod server add <kind> <name> <uri>`
- Set default server: `hymod server default <kind> <name>`
- Inspect server: `hymod server get <name>`
- Remove server: `hymod server remove <name>`
- Set global config: `hymod config set <key> <value>`
- Get global config value: `hymod config get <key>`
- List global config: `hymod config list`

## Project bootstrap
- Create a new mod project: `hymod new <name> [--path <dir>] [--group <group>] [--package <package>]`

## Notes
- Gradle wrappers/scripts may exist internally, but agent-driven workflows should go through `hymod`.
- If a task cannot be expressed through current `hymod` commands, call that out explicitly before falling back to direct Gradle usage.

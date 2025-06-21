# mapgen

This is a tool for generating map of a code repository.

Map is a shorter representation of code, that includes everything except functions content.  It is used to fed it to an AI agent as a context.

It is inspired by Aider.

## Usage

```bash
mapgen [SOURCES] [-o|--output OUTPUT_FILE] [-w|--watch] [-d|(-d N)|(--depth N)]
```
- `[SOURCES]` — Paths to files or directories to generate map for.  Can be a single file, a directory, or a glob pattern.
- `-o OUTPUT_FILE` or `--output OUTPUT_FILE` — Output file to write map to.  If not specified, map will be printed to stdout.
- `-w` or `--watch` — Watch for changes in sources and re-generate map on change.  Can only be used with the output file specified.
- One of the following depth options:
  - nothing (default) — infinite recursive traversal;
  - `-d` — only direct children of specified directory;
  - `-d N` or `--depth N` — depth of the traversal.

Examples:
```bash
mapgen --help
mapgen path/to/file.go path/to/file2.go
mapgen path/to/dir
mapgen path/to/**/glob-*.js

mapgen [SOURCES] --output map.json
```

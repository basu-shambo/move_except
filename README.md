# move_except

This is a tool written for the cases when you need to move a bunch of files into a new folder in the same directory. The default behaviour is to create a new directory if it doesn't exist.

## Features

- Move or copy files using glob patterns
- Exclude specific files or directories from operations
- Verbose logging for debugging
- Cross-platform support (Linux, macOS, Windows)
- The destination will be created in case it doesn't already exist

## Installation

### Prerequisites
- Rust (latest stable version recommended)
- Cargo package manager

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/move_except.git
   cd move_except
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/move_except` (or `move_except.exe` on Windows).

### Install via Cargo
```bash
cargo install --git https://github.com/yourusername/move_except.git
```

## Usage

```
move_except [options] <files_to_move> <destination> [-e, --exclude <files_or_folders_to_exclude>]
```

### Arguments
- `<files_to_move>`: One or more glob patterns for files to move/copy (e.g., `*.txt`, `src/**/*.rs`)
- `<destination>`: The target directory path

### Options
- `-c, --copy`: Copy files instead of moving them
- `-h, --help`: Display help message and exit
- `-v, --verbose`: Enable verbose output for debugging

### Exclusions
- Use `-e` or `--exclude` followed by glob patterns to exclude files or directories
- Multiple exclusions can be specified

## Examples

1. Move text files to a backup directory:
   ```bash
   move_except *.txt /backup/
   ```

2. Copy Rust source files to a temporary directory:
   ```bash
   move_except -c src/**/*.rs /tmp/
   ```

3. Move directories excluding `node_modules` and `.git`:
   ```bash
   move_except dir1 dir2 --exclude node_modules .git
   ```

4. Verbose move with exclusions:
   ```bash
   move_except -v *.log /logs/ -e error.log
   ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

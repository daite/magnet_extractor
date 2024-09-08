# Magnet extractor

This Rust program dynamically finds the `angel` program in the system's `$PATH`, runs it with a specified keyword, extracts magnet links from the output, and opens them in qBittorrent.

## Features
- Dynamically finds the `angel` program using the `$PATH` environment variable.
- Extracts magnet links from the `angel` program output using regular expressions.
- Opens magnet links in qBittorrent.

## Prerequisites

- Rust (for compiling and running the program)
- The `angel` program should be installed and accessible in your `$PATH`.
- qBittorrent (for handling magnet links)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/daite/magnet_extractor.git
   cd magnet_extractor
   ```

2. Add dependencies in `Cargo.toml`:

   ```toml
   [dependencies]
   which = "4.2"
   regex = "1"
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Usage

To run the program, provide a keyword to search via the `angel` program:

```bash
./target/release/magnet_extractor <keyword>
```

For example:

```bash
./target/release/magnet_extractor "some keyword"
```

### How It Works

1. The program checks if the `angel` program is available in the system's `$PATH` using the `which` crate.
2. If found, it runs the `angel` program with the specified keyword and extracts magnet links from the output using a regular expression.
3. The magnet links are automatically opened in qBittorrent.

### Example Output

```
Found 'angel' at: /usr/local/bin/angel
Opening magnet:?xt=urn:btih:6cf65299b5b48b077370f5675ce34b666e82cc3f in qBittorrent...
```

## Dependencies

- [which](https://crates.io/crates/which): To locate the `angel` program in the system.
- [regex](https://crates.io/crates/regex): To extract magnet links using regular expressions.

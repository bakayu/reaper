# Reaper (grep-like utility written in Rust)

A command-line tool that searches for text patterns in files.

## Overview

Reaper is a command-line tool that searches for specific text patterns in files, similar to the classic Unix `grep` utility. This implementation is written in Rust for performance, safety, and ease of use.

## Features

- Fast text searching in files
- Case-sensitive searching (default)
- Case-insensitive searching with a flag
- Clear and helpful output formatting

## Installation

- Pre-requisites: `Rust v1.82+`

- Build the project with:

```bash
git clone https://github.com/yourusername/reaper.git
cd reaper
cargo build --release
```

- The compiled binary will be available at `target/release/reaper`.

## Usage

Use the utility with the following commands: (make sure to add the path to the binary to your environment)

```bash
reaper [OPTIONS] <PATTERN> <FILE>
```

Options

- `-i, --ignore-case`: Perform case-insensitive matching.

#### Examples

- Search for "text" in a file:

```bash
repear text somefile.txt
```

- Search for "rust" case-insensitively:

```bash
reaper -i rust somefile.txt
```

or

```bash
reaper --ignore-case rust somefile.txt
```

### Tests

To run tests, run the following command:

```bash
cargo test
```

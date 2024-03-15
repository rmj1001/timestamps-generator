# Timestamp Generator

This program makes video timestamp processing speedy, efficient, and easy.

Features:

- Save to / Retrieve from disk
- Removing individual timestamps
- Viewing existing timestamps
- Clearing all timestamps from memory

## Installation

```bash
cargo install --locked cli_timestamps_generator
```

## Usage

```bash
generate-timestamps
```

The timestamps will be saved to a file called `timestamps.txt` in the directory
you called the program from.

```bash
# Use existing timestamps.txt file in the current directory
generate-timestamps --use-file
```

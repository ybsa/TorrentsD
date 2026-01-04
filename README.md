# Rust Torrent Client

A simple BitTorrent client written in Rust that supports downloading files from .torrent files.

## Features

- ✅ Bencode encoding/decoding
- ✅ .torrent file parsing (single and multi-file torrents)
- ✅ HTTP/HTTPS tracker communication
- ✅ Peer wire protocol implementation
- ✅ Multi-peer concurrent downloads
- ✅ SHA-1 piece verification
- ✅ Automatic file assembly
- ✅ Command-line interface

## Requirements

### Windows

You need the Microsoft Visual C++ Build Tools installed:

1. Download from: <https://visualstudio.microsoft.com/downloads/>
2. Install "Desktop development with C++" workload
3. Or run: `winget install Microsoft.VisualStudio.2022.BuildTools`

### macOS/Linux

Standard Rust toolchain is sufficient.

## Installation

Make sure you have Rust installed. Then build the project:

```bash
cargo build --release
```

## Usage

### Download a torrent

```bash
cargo run -- download path/to/file.torrent --output ./downloads
```

Options:

- `--output, -o`: Output directory (default: `./downloads`)
- `--port, -p`: Port to listen on (default: `6881`)

### View torrent information

```bash
cargo run -- info path/to/file.torrent
```

This displays:

- Torrent name
- Tracker URL
- Info hash
- Total size
- Piece information
- File list

## Example

```bash
# Download a torrent
cargo run --release -- download debian.torrent -o ~/Downloads

# View torrent info
cargo run -- info debian.torrent
```

## Architecture

- `bencode`: Bencode parser and encoder
- `torrent`: .torrent file parser and metainfo handling
- `tracker`: HTTP tracker client
- `peer`: Peer wire protocol and connection management
- `download`: Download orchestration and piece management
- `storage`: File I/O and piece assembly

## Limitations

- No DHT support
- No magnet link support
- Download only (no seeding)
- No peer exchange (PEX)
- No encryption
- HTTP/HTTPS trackers only (no UDP)

## License

MIT

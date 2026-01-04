# Rust Torrent Client

A high-performance BitTorrent client written in Rust with adaptive peer management and smart download optimization.

## Features

- ✅ Bencode encoding/decoding
- ✅ .torrent file parsing (single and multi-file torrents)
- ✅ HTTP/HTTPS tracker communication with auto re-announce
- ✅ Peer wire protocol implementation
- ✅ **Adaptive Pipelining** - Automatically adjusts request speed based on peer performance
- ✅ **Dynamic Peer Discovery** - Continuously finds new peers during download
- ✅ **Keep-Alive Heartbeat** - Maintains stable connections
- ✅ **Memory Efficient** - Doesn't load entire file into RAM
- ✅ Multi-peer concurrent downloads (up to 200 peers)
- ✅ SHA-1 piece verification
- ✅ Automatic file assembly
- ✅ Command-line interface

## Quick Start (Windows)

### Step 1: Install Build Tools

See [VISUAL_STUDIO_SETUP.md](VISUAL_STUDIO_SETUP.md) for detailed instructions.

### Step 2: Build

Open **Developer Command Prompt for VS 2022** and run:

```powershell
cd "c:\Users\wind xebec\OneDrive\Desktop\abc"
cargo build --release
```

### Step 3: Download a Torrent

```powershell
.\target\release\torrent-client.exe download "C:\path\to\file.torrent" -o "C:\Downloads"
```

## Usage

### Download a torrent

```bash
torrent-client download <torrent-file> [OPTIONS]
```

**Options:**

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--output` | `-o` | Output directory | `./downloads` |
| `--port` | `-p` | Listen port | `6881` |

**Examples:**

```bash
# Download to current directory
torrent-client download movie.torrent

# Download to specific folder
torrent-client download movie.torrent -o "C:\Users\wind xebec\Downloads"

# Using cargo (development)
cargo run --release -- download movie.torrent -o ./downloads
```

### View torrent information

```bash
torrent-client info <torrent-file>
```

This displays:

- Torrent name
- Tracker URL
- Info hash
- Total size
- Piece count and length
- File list

## Architecture

```
src/
├── bencode/     # Bencode parser and encoder
├── download/    # Download orchestration and piece management
│   ├── manager.rs  # Peer management, adaptive pipelining
│   └── piece.rs    # Block tracking, completion marking
├── peer/        # Peer wire protocol and connection handling
├── storage/     # File I/O and piece assembly
├── torrent/     # .torrent file parser and metainfo
├── tracker/     # HTTP tracker client
├── lib.rs       # Library exports
└── main.rs      # CLI entry point
```

## Performance Features

| Feature | Description |
|---------|-------------|
| Adaptive Pipelining | Starts with 3 requests, scales up to 20 for fast peers |
| Smart Retry | Waits 45s between tracker re-announces to avoid bans |
| Keep-Alive | Sends heartbeat every 30s to prevent disconnects |
| Memory Efficient | Uses `mark_complete()` to free RAM after piece verification |
| Dynamic Discovery | Continuously finds new peers during download |

## Limitations

- No DHT support
- No magnet link support
- Download only (no seeding)
- No peer exchange (PEX)
- No encryption
- HTTP/HTTPS trackers only (no UDP)

## License

MIT

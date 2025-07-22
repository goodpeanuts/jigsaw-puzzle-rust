[![dependency status](https://deps.rs/repo/github/goodpeanuts/jigsaw-puzzle-rust/status.svg)](https://deps.rs/repo/github/goodpeanuts/jigsaw-puzzle-rust)
[![Build Status](https://github.com/goodpeanuts/jigsaw-puzzle-rust/workflows/CI/badge.svg)](https://github.com/goodpeanuts/jigsaw-puzzle-rust/actions?workflow=CI)

# Jigsaw Puzzle Game in Rust

A jigsaw puzzle game built with Rust and egui, featuring responsive design and cross-platform support.

## 🎮 Quick Try

**[Play Online](https://goodpeanuts.github.io/jigsaw-puzzle-rust/)** - Try the game directly in your browser!

## 🛠️ Build & Run

### Prerequisites

- Rust (stable version)
- For web builds: `trunk` and `wasm32-unknown-unknown` target

### Native Build

```bash
# Clone the repository
git clone https://github.com/goodpeanuts/jigsaw-puzzle-rust.git
cd jigsaw-puzzle-rust

# Run the game
cargo run --release
```

### Web Build (WASM)

```bash
# Install required tools
rustup target add wasm32-unknown-unknown
cargo install --locked trunk

# Build and serve locally
trunk serve --port 8080

# Or build for production
trunk build --release
```

### Features

Build with specific features:
```bash
# Debug mode with console output
cargo run --features debug

# Chinese language support
cargo run --features chinese

# All features
cargo run --features debug,chinese
```

## 🎯 How to Play

1. **Select Difficulty**: Choose from Easy, Normal, or Difficult
2. **Start Game**: Click "Start" to begin the puzzle
3. **Move Pieces**: Click on puzzle pieces to swap them
4. **Use Bot**: Toggle the bot to watch automatic solving
5. **View Original**: Hover over "Original" button to see the complete image


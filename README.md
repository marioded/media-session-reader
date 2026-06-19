# media-session-reader

A cross-platform Rust library to retrieve information about the currently playing media session.

It provides a simple API to access the current track, artist, album cover, playback position, duration and playing state.

## Supported Platforms

| Platform | Backend |
|----------|---------|
| Windows 10/11 | WGSMTC |
| Linux | MPRIS |

## Features

- Get current playing track
- Get artist information
- Get album artwork
- Get playback duration
- Get current position
- Get playback state

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
media-session-reader = "0.0.1"
```

## Basic usage

```rust
use media_session_reader::current_track;

fn main() {
    if let Some(track) = current_track() {
        println!("{} - {}", track.artist, track.title);

        if track.playing {
            println!("Playing");
        } else {
            println!("Paused");
        }
    } else {
        println!("No media session found");
    }
}
```
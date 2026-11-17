# bunnymark-rust

A Rust port of the classic [raylib](https://www.raylib.com/) **bunnymark** benchmark, built with the [`raylib`](https://crates.io/crates/raylib) crate.

Hold the left mouse button to spawn bunnies and watch them bounce around the screen. The HUD shows the current bunny count, the number of batched draw calls, and the FPS — a quick way to compare rendering throughput across machines or implementations.

## Requirements

- A Rust toolchain (2024 edition)
- raylib build dependencies for your platform (on Linux you typically need `libasound2-dev`, `libx11-dev`, `libxrandr-dev`, `libxi-dev`, `libgl1-mesa-dev`, and `libxcursor-dev`)

## Build and run

```sh
cargo run --release
```

The bunnies are drawn in a single batched pass, so a release build is strongly recommended — debug builds will be dramatically slower.

## Controls

| Input | Action |
| --- | --- |
| Left mouse button (hold) | Spawn 100 bunnies per frame at the cursor |
| `P` | Pause / resume movement |

## Implementation notes

- Up to **300,000** bunnies can exist at once.
- Data is stored in parallel `Vec`s (positions, speeds, colors) for a data-oriented layout that keeps the update loop cache friendly.
- `src/main.rs` is the Rust implementation; `bunnymark.c` is the original C example kept for reference.
- Sprites are loaded from `resources/raybunny.png`.

## License

The original raylib example and its assets are licensed under the zlib/libpng license ([raylib LICENSE](https://www.raylib.com/license.html)). See `bunnymark.c` for the original copyright notice.

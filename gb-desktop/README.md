# gb-desktop

Rust Gameboy OpenGL emulator

## Prerequisites

Place Gameboy rooms at root example:

- gb-desktop
    - benches/
    - src/
    - Cargo.toml
    - gb_room.gb

## Controls

| Key        | Control |
|------------|---------|
| Z          | A       |
| X          | B       |
| Space      | Select  |
| Return     | Start   |
| Arrow Keys | D-pad   |

## Run Emulator
``sh
cargo run -- gb_room.gb
``
# hello-rs-libretro

A minimal [libretro](https://www.libretro.com/) core written in Rust, built as a learning
exercise and developer reference. If you want to write a libretro core in Rust and don't
know where to start, this is for you — and for future me.

This core displays an animated color-cycling crosshatch pattern and plays a sweeping tone
whose waveform can be cycled between square, pulse, and sawtooth by pressing a button. It
requires no ROM or content file to run.

## What this demonstrates

- The complete set of mandatory libretro exports and what each one does
- How to implement the libretro glue layer directly in Rust without wrapper crates,
  using raw `extern "C"` functions against the libretro ABI
- How to push video frames and audio samples to RetroArch each frame
- How to read input via RetroArch's poll/state callback model
- How to declare a no-content core that runs without a ROM file
- How to set the pixel format correctly (and why getting it wrong produces stripes)
- How to deploy a custom core to RetroArch on Steam on Linux

## Why no wrapper crates?

Several Rust libretro wrapper crates exist (`libretro-rs`, `rust-libretro`,
`libretro-backend`) but all are effectively unmaintained as of 2025. Rather than depend
on an abandoned abstraction, this project implements the libretro glue layer directly.
The API surface is small enough that this is straightforward, and it means you own every
inch of the stack.

## Project structure

```
src/
  lib.rs       — global statics and module declarations
  types.rs     — #[repr(C)] structs and constants mirroring libretro.h
  libretro.rs  — all extern "C" exports required by the libretro ABI
  core.rs      — HelloCore struct and runtime state
```

## The mandatory exports

RetroArch will refuse to load a core that is missing any of these symbols. The full list,
in the order RetroArch calls them during startup:

| Function | Purpose |
|---|---|
| `retro_set_environment` | Receive the environment callback; declare core capabilities |
| `retro_set_video_refresh` | Receive the video frame callback |
| `retro_set_audio_sample` | Receive the single-sample audio callback |
| `retro_set_audio_sample_batch` | Receive the batched audio callback |
| `retro_set_input_poll` | Receive the input poll callback |
| `retro_set_input_state` | Receive the input state callback |
| `retro_init` | Initialize core state |
| `retro_get_system_info` | Provide core name, version, supported extensions |
| `retro_get_system_av_info` | Provide resolution, aspect ratio, framerate, sample rate |
| `retro_load_game` | Load content (or accept null for no-content cores) |
| `retro_run` | Run one frame of video, audio, and input |
| `retro_unload_game` | Unload content |
| `retro_deinit` | Free core state |
| `retro_api_version` | Return `1` (RETRO_API_VERSION) |
| `retro_get_region` | Return NTSC (`0`) or PAL (`1`) |
| `retro_serialize_size` | Return save state buffer size, or `0` to opt out |
| `retro_serialize` | Write save state, or return `false` to opt out |
| `retro_unserialize` | Restore save state, or return `false` to opt out |
| `retro_get_memory_data` | Return pointer to a memory region, or null |
| `retro_get_memory_size` | Return size of a memory region, or `0` |
| `retro_cheat_reset` | Clear all active cheats |
| `retro_cheat_set` | Apply or remove a cheat code |
| `retro_set_controller_port_device` | Handle controller type changes |
| `retro_reset` | Soft reset |
| `retro_load_game_special` | Load multi-file content, or return `false` |

If RetroArch fails to load your core, check its log file for `Failed to load symbol:`
messages — that is how you find out which exports are missing.

## Deploying to RetroArch on Steam (Linux)

Build the core:

```bash
cargo build --release
```

The `.so` will be at `target/release/libhello_rs_libretro.so`.

Copy the `.so` to RetroArch's cores directory:

```bash
cp target/release/libhello_rs_libretro.so \
   ~/.steam/steam/steamapps/common/RetroArch/cores/
```

Copy the `.info` to RetroArch's cores directory:

```
cp libhello_rs_libretro.info \
   ~/.steam/steam/steamapps/common/RetroArch/cores/
```

In RetroArch: **Load Core → select the core → Start Core**. No content required.

## Documentation

```bash
cargo doc --open
```

All public items are documented. The generated docs include full descriptions of every
libretro callback, struct field, and constant.

## Development notes

This project was developed collaboratively with [Claude Sonnet 4.6](https://www.anthropic.com/claude)
(Anthropic). The libretro glue layer, inline documentation, and README were all written
through an iterative back-and-forth conversation.

## License

Copyright (C) 2025 David Brinovec

This program is free software: you can redistribute it and/or modify it under the terms
of the GNU Affero General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

See [LICENSE](LICENSE) for the full license text.

// hello-rs-libretro - A minimal hello world libretro core in Rust
// Copyright (C) 2025 David Brinovec
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::sync::Mutex;

use crate::core::HelloCore;

mod core;
mod libretro;
mod types;

/// The single global instance of the core.
///
/// Libretro cores are single-instance by design — RetroArch never loads more
/// than one instance of a core within the same process. This makes it safe to
/// use a global static for core state, which is necessary since the libretro
/// API uses fixed C ABI function signatures that leave no room for passing
/// state through arguments.
///
/// The [`Mutex`] wrapper ensures safe access from Rust's perspective even
/// though RetroArch calls core functions from a single thread. The
/// [`Option`] allows the core to be cleanly initialized in [`retro_init`]
/// and dropped in [`retro_deinit`].
static CORE: Mutex<Option<HelloCore>> = Mutex::new(None);

/// Callback provided by RetroArch for communicating core capabilities and
/// requesting frontend services.
///
/// This is the primary general-purpose communication channel from the core
/// back to RetroArch. It accepts a command number and a pointer to command-
/// specific data, making it a generic interface for functionality too
/// specialized to deserve its own dedicated function in the libretro API.
///
/// Set by RetroArch via [`retro_set_environment`], which is the first
/// function called when a core is loaded — before [`retro_init`]. Stored
/// as a static because it is needed in multiple places throughout the
/// core's lifecycle.
///
/// # Arguments
/// * `u32` — command number identifying what is being requested or set.
///   Constants for these commands are defined in [`crate::types`] with the
///   `RETRO_ENVIRONMENT_` prefix (e.g. [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`],
///   [`RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME`]).
/// * `*mut c_void` — pointer to command-specific data. The actual type
///   pointed to varies by command — it may be a bool, a u32, a struct, or
///   null depending on the command.
///
/// # Returns
/// `true` if RetroArch understood and accepted the command, `false` if the
/// command is unsupported or was rejected.
pub static mut ENVIRONMENT_CALLBACK: Option<
    unsafe extern "C" fn(u32, *mut std::ffi::c_void) -> bool,
> = None;

/// Callback provided by RetroArch for submitting a completed video frame.
///
/// This is the only way a core can display anything — cores cannot open
/// windows or write to the display directly. RetroArch owns the display;
/// the core hands it a pixel buffer each frame and RetroArch handles
/// scaling, shaders, recording, and everything else.
///
/// Set by RetroArch via [`retro_set_video_refresh`] before [`retro_init`]
/// is called. Called once per frame inside [`retro_run`].
///
/// # Arguments
/// * `*const c_void` — pointer to the pixel buffer. The format must match
///   what was declared via [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`]. For
///   XRGB8888 each pixel is 4 bytes in 0x00RRGGBB order.
/// * First `u32` — width of the frame in pixels.
/// * Second `u32` — height of the frame in pixels.
/// * `usize` — pitch: the number of **bytes** per row, not pixels. For a
///   tightly packed XRGB8888 buffer this is `width * 4`. RetroArch requires
///   this explicitly to support buffers where rows contain padding bytes,
///   which can occur when buffers are aligned to cache line boundaries.
pub static mut VIDEO_REFRESH_CALLBACK: Option<
    unsafe extern "C" fn(*const std::ffi::c_void, u32, u32, usize),
> = None;

/// Callback provided by RetroArch for submitting a single stereo audio sample.
///
/// One of two ways a core can submit audio to RetroArch, the other being
/// the more efficient [`AUDIO_SAMPLE_BATCH_CALLBACK`]. This variant submits
/// one stereo sample at a time and is generally only useful for cores that
/// generate audio one sample at a time naturally. For most cores including
/// this one, [`AUDIO_SAMPLE_BATCH_CALLBACK`] is preferred.
///
/// Set by RetroArch via [`retro_set_audio_sample`] before [`retro_init`]
/// is called. Not used by this core but must be stored per the libretro
/// spec.
///
/// # Arguments
/// * First `i16` — left channel sample, signed 16-bit PCM.
/// * Second `i16` — right channel sample, signed 16-bit PCM.
pub static mut AUDIO_SAMPLE_CALLBACK: Option<unsafe extern "C" fn(i16, i16)> = None;

/// Callback provided by RetroArch for submitting a batch of stereo audio samples.
///
/// The preferred way to submit audio to RetroArch, more efficient than
/// [`AUDIO_SAMPLE_CALLBACK`] since it amortizes the overhead of crossing
/// the FFI boundary across many samples at once. This core uses this
/// callback exclusively.
///
/// Set by RetroArch via [`retro_set_audio_sample_batch`] before [`retro_init`]
/// is called. Called once per frame inside [`retro_run`] after the video
/// frame has been submitted.
///
/// # Arguments
/// * `*const i16` — pointer to an interleaved stereo sample buffer in the
///   form [L, R, L, R, ...] where each sample is signed 16-bit PCM.
/// * `usize` — number of stereo frames in the buffer. Note this is frames,
///   not samples — a buffer containing 1600 i16 values represents 800
///   stereo frames.
///
/// # Returns
/// The number of stereo frames actually consumed by RetroArch. In practice
/// this is always equal to the number submitted.
pub static mut AUDIO_SAMPLE_BATCH_CALLBACK: Option<
    unsafe extern "C" fn(*const i16, usize) -> usize,
> = None;

/// Callback provided by RetroArch for polling input state.
///
/// Must be called exactly once per frame at the start of [`retro_run`]
/// before any input state is read via [`INPUT_STATE_CALLBACK`]. Calling it
/// tells RetroArch to snapshot the current state of all input devices.
/// Reading input without calling this first results in undefined behavior
/// per the libretro spec.
///
/// Set by RetroArch via [`retro_set_input_poll`] before [`retro_init`]
/// is called. Takes no arguments and returns nothing — it is purely a
/// synchronization signal.
pub static mut INPUT_POLL_CALLBACK: Option<unsafe extern "C" fn()> = None;

/// Callback provided by RetroArch for querying the state of a specific input.
///
/// Must only be called after [`INPUT_POLL_CALLBACK`] has been called to
/// snapshot controller state for the current frame. Set by RetroArch via
/// [`retro_set_input_state`] before [`retro_init`] is called.
///
/// # Arguments
/// * First `u32` — port number identifying the controller slot.
///   0 = player 1, 1 = player 2, etc.
/// * Second `u32` — device type identifying what kind of controller is
///   plugged in. 1 = RetroPad (the standard libretro gamepad, modeled
///   after a Super Nintendo controller with added L2/R2/L3/R3 buttons).
/// * Third `u32` — index. For analog sticks: 0 = left stick, 1 = right
///   stick. For digital buttons: always 0.
/// * Fourth `u32` — button or axis id. For a RetroPad: 0 = B, 1 = Y,
///   2 = Select, 3 = Start, 4 = D-pad Up, 5 = D-pad Down, 6 = D-pad Left,
///   7 = D-pad Right, 8 = A, 9 = X, 10 = L, 11 = R.
///
/// # Returns
/// For digital buttons: non-zero if pressed, zero if not pressed.
/// For analog axes: a signed 16-bit value in the range -32768 to 32767
/// representing the axis position.
pub static mut INPUT_STATE_CALLBACK: Option<unsafe extern "C" fn(u32, u32, u32, u32) -> i16> = None;

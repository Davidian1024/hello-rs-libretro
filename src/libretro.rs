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

use crate::{core::Waveform, types::RetroSystemInfo};

/// Returns the version of the libretro API this core was compiled against.
///
/// This is the first function RetroArch calls when loading a core. RetroArch
/// uses the returned value to verify ABI compatibility — if the version
/// doesn't match what RetroArch expects, it will refuse to load the core.
///
/// Always returns `1`, which is the value of `RETRO_API_VERSION` in
/// `libretro.h`. This value has been stable for many years and is unlikely
/// to change.
#[unsafe(no_mangle)]
pub extern "C" fn retro_api_version() -> u32 {
    eprintln!("retro_api_version(): started\n");
    eprintln!("retro_api_version(): finished\n");
    1
}

/// Provides static information about this core to RetroArch.
///
/// Called by RetroArch shortly after [`retro_api_version`] to identify the
/// core and learn about its content requirements. The information returned
/// here should be static — if dynamic allocation is used, the memory must
/// remain valid until [`retro_deinit`] is called.
///
/// This core uses `c""` string literals which produce static null-terminated
/// C strings with no allocation required.
///
/// # Arguments
/// * `info` — pointer to a [`RetroSystemInfo`] struct allocated by RetroArch
///   that this function fills in.
///
/// # Fields set
/// * `library_name` — the name of this core as it appears in RetroArch's UI.
/// * `library_version` — the version string shown alongside the name.
/// * `valid_extensions` — comma-separated list of file extensions this core
///   can load. Empty since this core requires no content.
/// * `need_fullpath` — if true, RetroArch passes the file path rather than
///   loading the file into memory. False since we require no content.
/// * `block_extract` — if true, RetroArch will not extract archive files
///   before passing them to the core. False since we require no content.
#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_info(info: *mut RetroSystemInfo) {
    eprintln!("retro_get_system_info(): started\n");
    unsafe {
        (*info).library_name = concat!(env!("CARGO_PKG_NAME"), "\0").as_ptr() as *const std::ffi::c_char;
        // (*info).library_name = c"hello-rs-libretro".as_ptr();
        (*info).library_version = concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as *const std::ffi::c_char;
        // (*info).library_version = c"0.1.0-build20".as_ptr();
        (*info).valid_extensions = c"".as_ptr();
        (*info).need_fullpath = false;
        (*info).block_extract = false;
    }
    eprintln!("retro_get_system_info(): finished\n");
}

/// Provides audio and video parameters for this core to RetroArch.
///
/// Called by RetroArch after [`retro_load_game`] to determine how to set up
/// its video and audio systems. The values returned here must be consistent
/// with the pixel format set via [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`] in
/// [`retro_set_environment`].
///
/// # Arguments
/// * `info` — pointer to a [`RetroSystemAvInfo`] struct allocated by RetroArch
///   that this function fills in.
///
/// # Fields set
/// * `geometry.base_width` / `geometry.base_height` — the core's native
///   resolution. RetroArch will scale this to fit the display.
/// * `geometry.max_width` / `geometry.max_height` — the maximum resolution
///   the core will ever produce. Must be at least as large as base dimensions.
/// * `geometry.aspect_ratio` — the intended display aspect ratio. Set to
///   `16.0 / 9.0` for widescreen. A value of `0.0` tells RetroArch to
///   calculate it from the base dimensions instead.
/// * `timing.fps` — the target frame rate. RetroArch uses this to drive its
///   main loop timing.
/// * `timing.sample_rate` — the audio sample rate in Hz. Must match the
///   rate at which samples are generated in [`retro_run`].
#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_av_info(info: *mut crate::types::RetroSystemAvInfo) {
    eprintln!("retro_get_system_av_info(): started\n");
    unsafe {
        (*info).geometry.base_width = 640;
        (*info).geometry.base_height = 360;
        (*info).geometry.max_width = 640;
        (*info).geometry.max_height = 360;
        (*info).geometry.aspect_ratio = 16.0 / 9.0;
        (*info).timing.fps = 60.0;
        (*info).timing.sample_rate = 48000.0;
    }
    eprintln!("retro_get_system_av_info(): finished\n");
}

/// Receives the environment callback from RetroArch and uses it to declare
/// core capabilities.
///
/// This is the first function RetroArch calls after loading the core, even
/// before [`retro_init`]. The environment callback is stored in
/// [`ENVIRONMENT_CALLBACK`] for use throughout the core's lifetime.
///
/// This function makes two environment calls:
/// * [`RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME`] — declares that this core
///   can run without any content loaded, allowing the user to start it
///   directly from RetroArch's Load Core menu without selecting a ROM.
/// * [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`] — declares that this core will
///   submit video frames in XRGB8888 format (32 bits per pixel, 8 bits per
///   channel). Without this call RetroArch assumes the legacy XRGB1555
///   format and colors will be incorrect.
///
/// # Arguments
/// * `cb` — the environment callback function pointer provided by RetroArch.
///   Accepts a command number and a pointer to command-specific data.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_environment(
    cb: unsafe extern "C" fn(u32, *mut std::ffi::c_void) -> bool,
) {
    eprintln!("retro_set_environment(): started\n");
    unsafe {
        crate::ENVIRONMENT_CALLBACK = Some(cb);

        let mut supports_no_game = true;
        cb(
            crate::types::RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
            &mut supports_no_game as *mut bool as *mut std::ffi::c_void,
        );
    }
    eprintln!("retro_set_environment(): finished\n");
}

/// Receives the video refresh callback from RetroArch.
///
/// Called by RetroArch during startup before [`retro_init`]. The callback
/// is stored in [`VIDEO_REFRESH_CALLBACK`] and called once per frame inside
/// [`retro_run`] to submit the completed pixel buffer to RetroArch for
/// display.
///
/// See [`VIDEO_REFRESH_CALLBACK`] for a description of the callback's
/// arguments.
///
/// # Arguments
/// * `cb` — the video refresh callback function pointer provided by RetroArch.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_video_refresh(
    cb: unsafe extern "C" fn(*const std::ffi::c_void, u32, u32, usize),
) {
    eprintln!("retro_set_video_refresh(): started\n");
    unsafe {
        crate::VIDEO_REFRESH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_video_refresh(): finished\n");
}

/// Receives the single-sample audio callback from RetroArch.
///
/// Called by RetroArch during startup before [`retro_init`]. Stored in
/// [`AUDIO_SAMPLE_CALLBACK`] but not used by this core — audio is submitted
/// in batches via [`AUDIO_SAMPLE_BATCH_CALLBACK`] instead, which is more
/// efficient. This callback must still be accepted and stored per the
/// libretro spec.
///
/// See [`AUDIO_SAMPLE_CALLBACK`] for a description of the callback's
/// arguments.
///
/// # Arguments
/// * `cb` — the single-sample audio callback function pointer provided by
///   RetroArch.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample(cb: unsafe extern "C" fn(i16, i16)) {
    eprintln!("retro_set_audio_sample(): started\n");
    unsafe {
        crate::AUDIO_SAMPLE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample(): finished\n");
}

/// Receives the batched audio callback from RetroArch.
///
/// Called by RetroArch during startup before [`retro_init`]. The callback
/// is stored in [`AUDIO_SAMPLE_BATCH_CALLBACK`] and called once per frame
/// inside [`retro_run`] to submit a buffer of interleaved stereo PCM samples
/// to RetroArch for playback.
///
/// This is the preferred audio submission method for this core. Submitting
/// samples in batches is more efficient than [`AUDIO_SAMPLE_CALLBACK`]
/// since it crosses the FFI boundary only once per frame rather than once
/// per sample.
///
/// See [`AUDIO_SAMPLE_BATCH_CALLBACK`] for a description of the callback's
/// arguments.
///
/// # Arguments
/// * `cb` — the batched audio callback function pointer provided by RetroArch.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample_batch(
    cb: unsafe extern "C" fn(*const i16, usize) -> usize,
) {
    eprintln!("retro_set_audio_sample_batch(): started\n");
    unsafe {
        crate::AUDIO_SAMPLE_BATCH_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_audio_sample_batch(): finished\n");
}

/// Receives the input poll callback from RetroArch.
///
/// Called by RetroArch during startup before [`retro_init`]. The callback
/// is stored in [`INPUT_POLL_CALLBACK`] and called once at the start of
/// each frame in [`retro_run`] before any input state is read. Calling it
/// signals RetroArch to snapshot the current state of all input devices.
///
/// See [`INPUT_POLL_CALLBACK`] for further details on correct usage.
///
/// # Arguments
/// * `cb` — the input poll callback function pointer provided by RetroArch.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_poll(cb: unsafe extern "C" fn()) {
    eprintln!("retro_set_input_poll(): started\n");
    unsafe {
        crate::INPUT_POLL_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_poll(): finished\n");
}

/// Receives the input state callback from RetroArch.
///
/// Called by RetroArch during startup before [`retro_init`]. The callback
/// is stored in [`INPUT_STATE_CALLBACK`] and called inside [`retro_run`]
/// after [`INPUT_POLL_CALLBACK`] has been called to query the state of
/// specific buttons or axes.
///
/// See [`INPUT_STATE_CALLBACK`] for a full description of the callback's
/// arguments and return value.
///
/// # Arguments
/// * `cb` — the input state callback function pointer provided by RetroArch.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_state(cb: unsafe extern "C" fn(u32, u32, u32, u32) -> i16) {
    eprintln!("retro_set_input_state(): started\n");
    unsafe {
        crate::INPUT_STATE_CALLBACK = Some(cb);
    }
    eprintln!("retro_set_input_state(): finished\n");
}

/// Initializes the core.
///
/// Called by RetroArch once after all callbacks have been set via the
/// `retro_set_*` functions and before [`retro_load_game`]. This is where
/// the core allocates and initializes its state.
///
/// This function creates the global [`CORE`] instance via [`HelloCore::new`]
/// and sets the pixel format to XRGB8888 via [`ENVIRONMENT_CALLBACK`]. The
/// pixel format is also set in [`retro_set_environment`] but is set again
/// here to ensure it is applied regardless of the order RetroArch calls
/// these functions.
#[unsafe(no_mangle)]
pub extern "C" fn retro_init() {
    eprintln!("retro_init(): started\n");
    unsafe {
        if let Some(cb) = crate::ENVIRONMENT_CALLBACK {
            let mut fmt = crate::types::RETRO_PIXEL_FORMAT_XRGB8888;
            cb(
                crate::types::RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
                &mut fmt as *mut u32 as *mut std::ffi::c_void,
            );
        }
    }

    let mut core = crate::CORE.lock().unwrap();
    *core = Some(crate::core::HelloCore::new());
    eprintln!("retro_init(): finished\n");
}

/// Loads content for this core.
///
/// Called by RetroArch after [`retro_init`] when the user selects content
/// to run, or immediately after [`retro_init`] with a null pointer if no
/// content is selected and the core declared support for no-content mode
/// via [`RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME`].
///
/// This core does not load any content — it runs entirely without a ROM or
/// any other input file. The `game` pointer is therefore always null and is
/// intentionally ignored.
///
/// # Arguments
/// * `game` — pointer to a [`RetroGameInfo`] struct describing the content
///   to load, or null if running in no-content mode.
///
/// # Returns
/// `true` if the core loaded successfully, `false` to signal a fatal error
/// that prevents the core from running.
#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game(_game: *const crate::types::RetroGameInfo) -> bool {
    eprintln!("retro_load_game(): started\n");
    // game will be null since we support no-content
    // just return true to signal success
    eprintln!("retro_load_game(): finished\n");
    true
}

/// Runs the core for one video frame.
///
/// This is the core's main loop, called by RetroArch once per frame at the
/// rate specified in [`retro_get_system_av_info`]. Each call to this function
/// must do three things in order:
///
/// 1. **Poll input** — call [`INPUT_POLL_CALLBACK`] to snapshot controller
///    state, then read any needed inputs via [`INPUT_STATE_CALLBACK`]. Input
///    must be polled before it is read.
/// 2. **Submit a video frame** — build a pixel buffer and pass it to
///    [`VIDEO_REFRESH_CALLBACK`]. Exactly one video frame must be submitted
///    per call.
/// 3. **Submit audio** — generate audio samples for this frame and pass them
///    to [`AUDIO_SAMPLE_BATCH_CALLBACK`]. The number of samples submitted
///    should match the sample rate divided by the frame rate — at 48000 Hz
///    and 60 fps this is 800 stereo frames per call.
///
/// This core renders an animated crosshatch pattern whose color cycles over
/// time and pans diagonally across the screen. Audio is a sweeping tone
/// whose waveform can be cycled between [`Waveform::Square`],
/// [`Waveform::Pulse`], and [`Waveform::Sawtooth`] by pressing a button.
#[unsafe(no_mangle)]
pub extern "C" fn retro_run() {
    // eprintln!("retro_run(): started\n");

    // Input

    let button_pressed = unsafe {
        if let Some(poll) = crate::INPUT_POLL_CALLBACK {
            poll();
        }
        if let Some(state) = crate::INPUT_STATE_CALLBACK {
            state(0, 1, 0, 0) != 0
        } else {
            false
        }
    };

    {
        let mut core = crate::CORE.lock().unwrap();
        let core = core.as_mut().unwrap();

        if button_pressed && !core.last_button {
            core.waveform = match core.waveform {
                Waveform::Square => { eprintln!("switching to Pulse\n"); Waveform::Pulse },
                Waveform::Pulse => { eprintln!("switching to Sawtooth\n"); Waveform::Sawtooth },
                Waveform::Sawtooth => { eprintln!("switching to Square\n"); Waveform::Square },
            }
        }
        core.last_button = button_pressed;
    }

    // Video

    let frame_count = {
        let mut core = crate::CORE.lock().unwrap();
        let core = core.as_mut().unwrap();
        core.frame_count += 1;
        core.frame_count
    };

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 360;
    const PITCH: usize = (WIDTH as usize) * 4;

    let mut buffer = vec![0u32; (WIDTH * HEIGHT) as usize];

    let r = ((frame_count >> 1) & 0xFF) as u32;
    let g = ((frame_count >> 2) & 0xFF) as u32;
    let b = ((frame_count >> 3) & 0xFF) as u32;
    let color = (r << 16) | (g << 8) | b;

    let line_color = color;
    let bg_color = 0x00000000u32;

    let spacing_x = WIDTH as usize / 16;
    let spacing_y = HEIGHT as usize / 9;

    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let on_vertical = (x + (frame_count as usize)) % spacing_x == 0;
            let on_horizontal = (y + (frame_count as usize)) % spacing_y == 0;
            buffer[y * WIDTH as usize + x] = if on_vertical || on_horizontal {
                line_color
            } else {
                bg_color
            };
        }
    }

    unsafe {
        if let Some(cb) = crate::VIDEO_REFRESH_CALLBACK {
            cb(
                buffer.as_ptr() as *const std::ffi::c_void,
                WIDTH,
                HEIGHT,
                PITCH,
            );
        }
    }

    // Audio

    let mut audio_buffer: Vec<i16> = Vec::new();
    let samples_per_frame = 48000 / 60; // 800 samples per frame

    {
        let mut core = crate::CORE.lock().unwrap();
        let core = core.as_mut().unwrap();

        core.frequency += 64.0;
        if core.frequency > 440.0 {
            core.frequency = 110.0;
        }

        for _ in 0..samples_per_frame {

            let sample = match core.waveform {
                Waveform::Square => if core.phase < 0.5 { 2000i16 } else { -2000i16 },
                Waveform::Pulse => ((core.phase * 2.0 - 1.0) * 2000.0) as i16,
                Waveform::Sawtooth => if core.phase < 0.1 { 2000i16 } else { -2000i16 },
            };

            audio_buffer.push(sample);
            audio_buffer.push(sample);

            core.phase += core.frequency / 48000.0;
            if core.phase >= 1.0 {
                core.phase -= 1.0;
            }
        }
    }

    unsafe {
        if let Some(cb) = crate::AUDIO_SAMPLE_BATCH_CALLBACK {
            cb(audio_buffer.as_ptr(), samples_per_frame as usize);
        }
    }

    // eprintln!("retro_run(): finished\n");
}

/// Deinitializes the core and frees all resources.
///
/// Called by RetroArch after [`retro_unload_game`] when the core is being
/// shut down. This is the counterpart to [`retro_init`] — anything
/// allocated there should be freed here.
///
/// Drops the global [`CORE`] instance by setting it to `None`, which
/// triggers the [`Drop`] implementation for [`HelloCore`] and frees all
/// associated state. After this function returns the core must be in a
/// clean state and ready to be initialized again via [`retro_init`] if
/// RetroArch chooses to do so.
#[unsafe(no_mangle)]
pub extern "C" fn retro_deinit() {
    eprintln!("retro_deinit(): started\n");
    let mut core = crate::CORE.lock().unwrap();
    *core = None;
    eprintln!("retro_deinit(): finished\n");
}

/// Unloads the current content.
///
/// Called by RetroArch before [`retro_deinit`] when the user closes the
/// current content or before loading new content. This is the counterpart
/// to [`retro_load_game`] — anything allocated during content loading
/// should be freed here.
///
/// Since this core loads no content in [`retro_load_game`], there is
/// nothing to unload here.
#[unsafe(no_mangle)]
pub extern "C" fn retro_unload_game() {
    eprintln!("retro_unload_game(): started\n");
    eprintln!("retro_unload_game(): finished\n");
}

/// Returns the region of the content currently loaded.
///
/// Called by RetroArch to determine whether the core is running in NTSC
/// or PAL mode, which affects timing and display characteristics. Returns
/// `0` for NTSC (60 Hz) or `1` for PAL (50 Hz).
///
/// This core always returns `0` (NTSC) since it has no region-specific
/// behavior and targets 60 fps as declared in [`retro_get_system_av_info`].
#[unsafe(no_mangle)]
pub extern "C" fn retro_get_region() -> u32 {
    eprintln!("retro_get_region(): started\n");
    eprintln!("retro_get_region(): finished\n");
    0 // NTSC
}

/// Returns the number of bytes required to serialize the core's state.
///
/// Called by RetroArch when the user requests a save state. The returned
/// value tells RetroArch how much memory to allocate before calling
/// [`retro_serialize`].
///
/// This core does not support save states and returns `0` to opt out.
/// Returning `0` causes RetroArch to skip the save state operation
/// entirely without calling [`retro_serialize`].
#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize_size() -> usize {
    eprintln!("retro_serialize_size(): started\n");
    eprintln!("retro_serialize_size(): finished\n");
    0
}

/// Serializes the core's current state into a buffer.
///
/// Called by RetroArch when the user requests a save state, immediately
/// after allocating a buffer of [`retro_serialize_size`] bytes. The core
/// should write its complete state into `data` such that it can be fully
/// restored later via [`retro_unserialize`].
///
/// This core does not support save states and returns `false` to signal
/// failure. In practice this function will never be called since
/// [`retro_serialize_size`] returns `0`.
///
/// # Arguments
/// * `data` — pointer to a buffer of [`retro_serialize_size`] bytes
///   allocated by RetroArch to receive the serialized state.
/// * `size` — the size of the buffer in bytes, equal to the value
///   returned by [`retro_serialize_size`].
///
/// # Returns
/// `true` if serialization succeeded, `false` if it failed.
#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize(_data: *mut std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_serialize(): started\n");
    eprintln!("retro_serialize(): finished\n");
    false
}

/// Restores the core's state from a previously serialized buffer.
///
/// Called by RetroArch when the user loads a save state. The core should
/// read its complete state from `data` and restore itself to exactly the
/// state it was in when [`retro_serialize`] produced that buffer.
///
/// This core does not support save states and returns `false` to signal
/// failure. In practice this function will never be called since
/// [`retro_serialize_size`] returns `0`.
///
/// # Arguments
/// * `data` — pointer to a buffer containing a previously serialized state
///   produced by [`retro_serialize`].
/// * `size` — the size of the buffer in bytes.
///
/// # Returns
/// `true` if the state was restored successfully, `false` if it failed.
#[unsafe(no_mangle)]
pub extern "C" fn retro_unserialize(_data: *const std::ffi::c_void, _size: usize) -> bool {
    eprintln!("retro_unserialize(): started\n");
    eprintln!("retro_unserialize(): finished\n");
    false
}

/// Returns a pointer to a region of the core's memory.
///
/// Called by RetroArch to expose internal memory regions for use by
/// external tools such as cheat engines, memory watchers, and
/// RetroAchievements. The `id` argument identifies which memory region
/// is being requested.
///
/// Common memory region IDs:
/// * `0` — Save RAM (SRAM)
/// * `1` — Real-time clock (RTC)
/// * `2` — System RAM
/// * `3` — Video RAM
///
/// This core does not expose any memory regions and returns a null pointer
/// for all IDs. [`retro_get_memory_size`] correspondingly returns `0`.
///
/// # Arguments
/// * `id` — identifies which memory region RetroArch is requesting.
///
/// # Returns
/// A pointer to the requested memory region, or null if the region is
/// not available.
#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_data(_id: u32) -> *mut std::ffi::c_void {
    eprintln!("retro_get_memory_data(): started\n");
    eprintln!("retro_get_memory_data(): finished\n");
    std::ptr::null_mut()
}

/// Returns the size in bytes of a region of the core's memory.
///
/// Called by RetroArch alongside [`retro_get_memory_data`] to determine
/// the size of a memory region exposed by the core. The `id` argument
/// identifies which memory region is being queried and corresponds to
/// the same IDs described in [`retro_get_memory_data`].
///
/// This core does not expose any memory regions and returns `0` for all
/// IDs, consistent with [`retro_get_memory_data`] returning a null pointer.
///
/// # Arguments
/// * `id` — identifies which memory region RetroArch is querying.
///
/// # Returns
/// The size in bytes of the requested memory region, or `0` if the region
/// is not available.
#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_size(_id: u32) -> usize {
    eprintln!("retro_get_memory_size(): started\n");
    eprintln!("retro_get_memory_size(): finished\n");
    0
}

/// Resets all active cheat codes.
///
/// Called by RetroArch when the user clears all cheats. The core should
/// disable any currently active cheat effects and restore normal behavior.
///
/// This core does not support cheats and this function is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_reset() {
    eprintln!("retro_cheat_reset(): started\n");
    eprintln!("retro_cheat_reset(): finished\n");
}

/// Applies or removes a single cheat code.
///
/// Called by RetroArch when the user enables or disables a specific cheat.
/// The core should apply or remove the effect of the cheat identified by
/// `index`.
///
/// This core does not support cheats and this function is a no-op.
///
/// # Arguments
/// * `index` — the position of this cheat in RetroArch's cheat list.
/// * `enabled` — `true` if the cheat should be applied, `false` if it
///   should be removed.
/// * `code` — a null-terminated C string containing the cheat code in
///   whatever format the core expects.
#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_set(_index: u32,_enabledd: bool, _code: *const std::ffi::c_char) {
    eprintln!("retro_cheat_set(): started\n");
    eprintln!("retro_cheat_set(): finished\n");
}

/// Notifies the core that a controller type has been set for a port.
///
/// Called by RetroArch when the user changes the controller type assigned
/// to a specific port, for example switching from a standard RetroPad to
/// an analog controller or a lightgun. The core should adjust its input
/// handling accordingly.
///
/// This core does not distinguish between controller types and this
/// function is a no-op. All input is read as a standard RetroPad via
/// [`INPUT_STATE_CALLBACK`].
///
/// # Arguments
/// * `port` — the controller port being configured (0 = player 1, etc.)
/// * `device` — the device type being assigned to the port. Common values
///   are `1` for RetroPad and `5` for analog controller.
#[unsafe(no_mangle)]
pub extern "C" fn retro_set_controller_port_device(_port: u32, _device: u32) {
    eprintln!("retro_set_controller_port_device(): started\n");
    eprintln!("retro_set_controller_port_device(): finished\n");
}

/// Resets the core to its initial state.
///
/// Called by RetroArch when the user triggers a soft reset, equivalent to
/// pressing the reset button on the original hardware. The core should
/// return to the same state it was in immediately after [`retro_load_game`]
/// returned, without reinitializing everything from scratch as
/// [`retro_init`] would.
///
/// This core has no meaningful reset behavior — the animated crosshatch
/// and audio sweep have no concept of a starting state worth restoring.
/// This function is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn retro_reset() {
    eprintln!("retro_reset(): started\n");
    eprintln!("retro_reset(): finished\n");
}

/// Loads a special type of content requiring multiple ROM files.
///
/// Called by RetroArch instead of [`retro_load_game`] for content types
/// that require more than one file simultaneously, such as multi-disc
/// games, super game boy style hardware combinations, or other scenarios
/// where a single ROM file is insufficient.
///
/// This core does not support any special content types and returns `false`
/// to signal failure for all requests.
///
/// # Arguments
/// * `game_type` — identifies the type of special content being loaded.
///   The meaning of specific values is defined by each core.
/// * `info` — pointer to an array of [`RetroGameInfo`] structs, one per
///   required file, with `num_info` entries.
/// * `num_info` — the number of [`RetroGameInfo`] entries in `info`.
///
/// # Returns
/// `true` if the content loaded successfully, `false` if it failed or
/// is not supported.
#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game_special(
    _game_type: u32,
    _info: *const crate::types::RetroGameInfo,
    _num_info: usize,
) -> bool {
    eprintln!("retro_load_game_special(): started\n");
    eprintln!("retro_load_game_special(): finished\n");
    false
}

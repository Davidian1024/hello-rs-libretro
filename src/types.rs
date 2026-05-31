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

/// The XRGB8888 pixel format — 32 bits per pixel, 8 bits per channel.
///
/// Passed to [`RETRO_ENVIRONMENT_SET_PIXEL_FORMAT`] in
/// [`retro_set_environment`] to declare the format of pixel buffers
/// submitted via [`VIDEO_REFRESH_CALLBACK`]. Each pixel is laid out in
/// memory as `0x00RRGGBB` with one ignored byte followed by 8 bits each
/// of red, green, and blue.
///
/// Note: despite being value `1` in the enum, this is the recommended
/// modern format. The legacy default `XRGB1555` is value `0` and `RGB565`
/// is value `2`.
pub const RETRO_PIXEL_FORMAT_XRGB8888: u32 = 1;

/// Environment command to set the pixel format used by this core.
///
/// Passed as the command argument to [`ENVIRONMENT_CALLBACK`] with a
/// pointer to a `u32` containing the desired pixel format constant
/// (e.g. [`RETRO_PIXEL_FORMAT_XRGB8888`]). Must be called before the
/// first video frame is submitted. If not called, RetroArch assumes the
/// legacy `XRGB1555` format and colors will be incorrect.
pub const RETRO_ENVIRONMENT_SET_PIXEL_FORMAT: u32 = 10;

/// Environment command to declare that this core can run without content.
///
/// Passed as the command argument to [`ENVIRONMENT_CALLBACK`] with a
/// pointer to a `bool` set to `true`. Allows the user to start the core
/// directly from RetroArch's Load Core menu without selecting a ROM file.
/// Without this declaration RetroArch will require content before allowing
/// the core to run.
pub const RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME: u32 = 18;

/// Corresponds to `retro_system_info` in `libretro.h`.
///
/// Filled in by [`retro_get_system_info`] to provide RetroArch with static
/// information about this core. All pointer fields must remain valid for
/// the lifetime of the core — this is satisfied by using `c""` string
/// literals which are stored in the binary's read-only data segment.
///
/// `#[repr(C)]` ensures the struct's memory layout matches what RetroArch
/// expects when it writes into the pointer passed to [`retro_get_system_info`].
#[repr(C)]
pub struct RetroSystemInfo {
    /// The name of the core as displayed in RetroArch's UI.
    pub library_name: *const std::ffi::c_char,
    /// The version string displayed alongside the core name in RetroArch's UI.
    pub library_version: *const std::ffi::c_char,
    /// A pipe-separated list of file extensions this core can load,
    /// e.g. `"nes|zip"`. Empty for cores that require no content.
    pub valid_extensions: *const std::ffi::c_char,
    /// If `true`, RetroArch passes the file path to [`retro_load_game`]
    /// rather than loading the file into memory first. Useful for cores
    /// that need to open multiple files relative to the ROM path, or for
    /// ROMs too large to load into memory at once.
    pub need_fullpath: bool,
    /// If `true`, RetroArch will not extract archive files (e.g. `.zip`)
    /// before passing them to [`retro_load_game`]. The core is responsible
    /// for handling the archive itself.
    pub block_extract: bool,
}

/// Corresponds to `retro_game_geometry` in `libretro.h`.
///
/// Describes the video output dimensions and aspect ratio of this core.
/// Part of [`RetroSystemAvInfo`], filled in by [`retro_get_system_av_info`].
#[repr(C)]
pub struct RetroGameGeometry {
    /// The core's native output width in pixels. RetroArch uses this as
    /// the base resolution for scaling.
    pub base_width: u32,
    /// The core's native output height in pixels.
    pub base_height: u32,
    /// The maximum width in pixels this core will ever produce. Must be
    /// at least as large as `base_width`.
    pub max_width: u32,
    /// The maximum height in pixels this core will ever produce. Must be
    /// at least as large as `base_height`.
    pub max_height: u32,
    /// The intended display aspect ratio, e.g. `16.0 / 9.0`. A value of
    /// `0.0` tells RetroArch to calculate the aspect ratio from
    /// `base_width` and `base_height` instead.
    pub aspect_ratio: f32,
}

/// Corresponds to `retro_system_timing` in `libretro.h`.
///
/// Describes the timing characteristics of this core's audio and video
/// output. Part of [`RetroSystemAvInfo`], filled in by
/// [`retro_get_system_av_info`].
#[repr(C)]
pub struct RetroSystemTiming {
    /// The target frame rate in frames per second. RetroArch uses this to
    /// drive its main loop timing and vsync behavior.
    pub fps: f64,
    /// The audio sample rate in Hz. Must match the rate at which samples
    /// are generated and submitted in [`retro_run`]. Set to `0.0` if the
    /// core produces no audio.
    pub sample_rate: f64,
}

/// Corresponds to `retro_system_av_info` in `libretro.h`.
///
/// Filled in by [`retro_get_system_av_info`] to provide RetroArch with
/// the video geometry and audio/video timing parameters for this core.
#[repr(C)]
pub struct RetroSystemAvInfo {
    /// Video output dimensions and aspect ratio.
    pub geometry: RetroGameGeometry,
    /// Audio and video timing parameters.
    pub timing: RetroSystemTiming,
}

/// Corresponds to `retro_game_info` in `libretro.h`.
///
/// Passed to [`retro_load_game`] by RetroArch to describe the content to
/// be loaded. When running in no-content mode the pointer to this struct
/// is null and none of its fields are valid.
#[repr(C)]
pub struct RetroGameInfo {
    /// Path to the content file on disk as a null-terminated C string.
    /// Only valid if `need_fullpath` was set to `true` in [`RetroSystemInfo`].
    /// May be null if the path is not available.
    pub path: *const std::ffi::c_char,
    /// Pointer to the content file loaded into memory by RetroArch.
    /// Only valid if `need_fullpath` was set to `false` in [`RetroSystemInfo`].
    /// Null if the file could not be loaded or if running in no-content mode.
    pub data: *const std::ffi::c_void,
    /// The size in bytes of the data pointed to by `data`. Zero if `data`
    /// is null.
    pub size: usize,
    /// Optional metadata string associated with the content, as a
    /// null-terminated C string. May be null. The format and meaning of
    /// this field is not well defined in the libretro spec and is rarely
    /// used in practice.
    pub meta: *const std::ffi::c_char,
}
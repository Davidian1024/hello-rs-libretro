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

/// The available audio waveform shapes for the tone generator.
///
/// Cycled by pressing a button during [`retro_run`]. Each variant produces
/// a distinctly different timbre at the same frequency, demonstrating the
/// characteristic sounds of early arcade hardware synthesis.
pub enum Waveform {
    /// A square wave — alternates between maximum positive and maximum
    /// negative amplitude at a 50% duty cycle. Produces a hollow, buzzy
    /// tone rich in odd harmonics.
    Square,
    /// A pulse wave — similar to square but with a narrow 10% duty cycle.
    /// Produces a thinner, harsher, more aggressive buzz than square.
    Pulse,
    /// A sawtooth wave — ramps linearly from minimum to maximum amplitude
    /// then snaps back. Produces a bright, raspy tone rich in both odd
    /// and even harmonics, characteristic of early synthesizers.
    Sawtooth,
}

/// The core's runtime state, persisted across frames via the global [`CORE`] static.
///
/// Created in [`retro_init`] via [`HelloCore::new`] and dropped in
/// [`retro_deinit`]. All fields are accessible within the crate via
/// `pub(crate)`.
pub struct HelloCore {
    /// Incremented once per call to [`retro_run`]. Used to drive the color
    /// cycling and diagonal panning of the crosshatch pattern.
    pub(crate) frame_count: u32,
    /// The current phase of the audio oscillator, in the range `0.0..1.0`.
    /// Represents the position within one complete cycle of the waveform.
    /// Advanced each sample by `frequency / sample_rate`.
    pub(crate) phase: f32,
    /// The current frequency of the audio oscillator in Hz. Sweeps upward
    /// each frame and resets when it exceeds the maximum, producing a
    /// repeating ascending tone sweep.
    pub(crate) frequency: f32,
    /// The currently active waveform shape. Cycled through [`Waveform`]
    /// variants when the player presses a button.
    pub(crate) waveform: Waveform,
    /// The button state from the previous frame. Used to detect a fresh
    /// button press (transition from unpressed to pressed) rather than
    /// triggering on every frame the button is held down.
    pub(crate) last_button: bool,
}

impl HelloCore {
    /// Creates a new [`HelloCore`] with default initial state.
    ///
    /// The oscillator starts at 220 Hz (A3) with a [`Waveform::Square`]
    /// shape, ready to sweep upward when [`retro_run`] begins calling.
    pub fn new() -> Self {
        HelloCore {
            frame_count: 0,
            phase: 0.0,
            frequency: 220.0,
            waveform: Waveform::Square,
            last_button: false,
        }
    }
}

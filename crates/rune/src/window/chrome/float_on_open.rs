//! `App::float_on_open`: tiling window managers float a toplevel that maps at a fixed size, so
//! the window maps with its minimum equal to its maximum and takes its own limits once the
//! compositor has shown its first frame.
//!
//! Not as soon as the GPU has finished that frame. A compositor may hold a commit until its
//! buffer is ready while it takes `set_min_size` and `set_max_size` as they arrive: Hyprland
//! queues the surface state behind the buffer's acquire point and copies the toplevel's pending
//! limits when the commit applies. The finished frame wakes the compositor and the app at once,
//! so limits released then can reach the toplevel before the commit applies, and the window
//! tiles. A frame callback comes only from a commit that applied, and winit holds back
//! `RedrawRequested` until the first frame's callback comes, so the limits go with the paint
//! after it.

use winit::dpi::{LogicalSize, PhysicalSize, Size};
use winit::window::{Window, WindowAttributes};

use crate::machine::Effects;
use crate::window::surface::Win;
use crate::windows::WindowOptions;

/// Where a window that opened at a fixed size is in taking its own limits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::window) enum Release {
    /// At its fixed size until its first frame is shown.
    #[default]
    Fixed,
    /// The first frame is shown: the limits go with the next paint, which comes once the
    /// compositor answers that frame.
    Owed,
    /// It has its own limits.
    Done,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::window) enum ReleaseEvent {
    /// The window's first frame was shown.
    FirstFrame,
    /// The window paints (`RedrawRequested`).
    Paint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::window) enum ReleaseEffect {
    /// Ask for the paint the limits go with.
    AskPaint,
    /// Release the limits before this paint draws, so its commit carries them.
    Release,
}

impl Release {
    pub(in crate::window) fn step(&mut self, event: ReleaseEvent) -> Effects<ReleaseEffect> {
        match (*self, event) {
            (Release::Fixed, ReleaseEvent::FirstFrame) => {
                *self = Release::Owed;
                Effects::one(ReleaseEffect::AskPaint)
            }
            (Release::Owed, ReleaseEvent::Paint) => {
                *self = Release::Done;
                Effects::one(ReleaseEffect::Release)
            }
            _ => Effects::none(),
        }
    }
}

/// `attrs` with the window's size as both its minimum and its maximum.
pub(in crate::window) fn fixed(attrs: WindowAttributes, options: &WindowOptions, size: (f32, f32)) -> WindowAttributes {
    let fixed: Size = match options.physical_size {
        Some((w, h)) => PhysicalSize::new(w, h).into(),
        None => LogicalSize::new(size.0, size.1).into(),
    };
    attrs.with_min_inner_size(fixed).with_max_inner_size(fixed)
}

/// Steps the window's release and carries out what it asks for.
pub(in crate::window) fn step(w: &mut Win, event: ReleaseEvent, options: &WindowOptions, app_min: Option<(f32, f32)>) {
    for effect in w.native.float.step(event) {
        match effect {
            ReleaseEffect::AskPaint => w.request_redraw(),
            ReleaseEffect::Release => release(&w.window, options, app_min),
        }
    }
}

/// Mapped at a fixed size, so a tiling window manager floated it: now the user can resize it.
fn release(window: &Window, options: &WindowOptions, app_min: Option<(f32, f32)>) {
    let (min, max) = options.size_limits(app_min);
    window.set_min_inner_size(Some(LogicalSize::new(min.0, min.1)));
    window.set_max_inner_size(max.map(|(w, h)| LogicalSize::new(w, h)));
}

#[cfg(test)]
#[path = "float_on_open_tests.rs"]
mod tests;

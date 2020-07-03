//! Amethyst control crate.

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2018_compatibility
)]
#![warn(clippy::all)]
#![allow(clippy::new_without_default)]

pub use self::{
    bundles::{FlyControlBundle, ArcBallControlBundle},
    components::{ArcBallControl, FlyControl},
    resources::{HideCursor, WindowFocus},
    systems::{
        build_free_rotation_system,
        build_fly_movement_system,
        build_arc_ball_rotation_system,
        build_mouse_focus_update_system,
        build_cursor_hide_system,
    },
};

mod bundles;
mod components;
mod resources;
mod systems;

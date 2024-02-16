//! Camera controller implementation.

use bevy::{app::prelude::*, ecs::prelude::*};

pub mod component;
pub mod inputs;
pub mod momentum;
pub mod motion;
pub mod projections;
pub mod smoothing;

/// Adds [`bevy_editor_cam`](crate) functionality without an input plugin or any extensions. This
/// requires an input plugin to function! If you don't use the [`crate::input::DefaultInputPlugin`],
/// you will need to provide your own.
pub struct MinimalEditorCamPlugin;

impl Plugin for MinimalEditorCamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                crate::controller::component::EditorCam::update_camera_positions,
                crate::controller::projections::update_orthographic,
                crate::controller::projections::update_perspective,
            )
                .chain()
                .after(bevy_picking_core::PickSet::Last),
        )
        .register_type::<component::EditorCam>();
    }
}

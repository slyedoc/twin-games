#![allow(clippy::type_complexity)]

mod camera_controller;
mod config;
mod editor;
mod materials;
mod shapes;

use bevy::{
    diagnostic::*,
    prelude::*,
    PipelinedDefaultPlugins, pbr2::DirectionalLightShadowMap,
};
#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;

use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

use camera_controller::CameraControllerPlugin;

use shapes::ShapePlugin;
use materials::MaterialPlugin;
use editor::EditorPlugin;
pub mod prelude {
    pub use crate::{
        camera_controller::*,
        shapes::*,
        materials::*,
        StandardEnvironmentPlugin
    };
}

pub struct StandardEnvironmentPlugin;

impl Plugin for StandardEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(WindowDescriptor {
                title: "Twin Games".to_string(),
                width: 800.0,
                height: 600.0,
                vsync: false, // disable to break 60 fps
                ..Default::default()
            })
            .add_plugins(PipelinedDefaultPlugins)
            .add_plugin(WorldInspectorPlugin::default())
            .add_plugin(EditorPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(MaterialPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            //.add_plugin(LogDiagnosticsPlugin::default())
            .add_system(control_system)
            // limiting for wasm firefox
            .insert_resource(DirectionalLightShadowMap{
                size: 2048,
            });

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_plugin(config::ConfigPlugin);
        }
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn control_system(
    #[cfg(not(target_arch = "wasm32"))]
    mut exit: EventWriter<AppExit>,
    key_input: Res<Input<KeyCode>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
) {
    if key_input.just_pressed(KeyCode::F12) {
        world_inspection.enabled = !world_inspection.enabled;
    }
    #[cfg(not(target_arch = "wasm32"))]
    if key_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

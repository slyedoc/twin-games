

use bevy::{
    math::*,
    pbr2::{AmbientLight, StandardMaterial},
    prelude::{Assets, Commands, GlobalTransform, ResMut, Transform, App},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::{shape, Mesh},
        view::{ComputedVisibility, Visibility},
    },
};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use engine::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugin(StandardEnvironmentPlugin)
        .add_plugin(InspectorPlugin::<Turn>::new().open(false))
        .add_startup_system(setup)
        .run();
}

#[derive(Default, Inspectable)]
pub struct Turn(u8);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        materials.add(Color::GREEN.into()),
    ));

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default());

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });
}

use bevy::{
    render2::{
        view::{Visibility, ComputedVisibility},
        mesh::{Mesh, shape},
        camera::PerspectiveCameraBundle},
        prelude::{App, Commands, ResMut, Assets, Transform, GlobalTransform, Color},
         math::Vec3
    };
use engine::prelude::*;

fn main() {
    App::new()
        .add_plugin(StandardEnvironmentPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        materials.add(CustomMaterial {
            color: Color::GREEN,
        }),
    ));

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(CameraController::default());
}

// disable console opening on windows
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy_obj::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_startup_system(load)
        .run();
}

fn load(mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // whale
    // Load OBJ file
    let whale_mesh_handle = asset_server.load("whale.obj");
    commands.spawn_bundle(PbrBundle {
        mesh: whale_mesh_handle,
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("whale.png")),
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0)
            .with_rotation(Quat::from_rotation_y(1.)),
        ..Default::default()
    });

    // barnacle
    // Load OBJ file
    let barnacle_mesh_handle = asset_server.load("barnacle.obj");
    commands.spawn_bundle(PbrBundle {
        mesh: barnacle_mesh_handle,
        material: materials.add(Color::rgb(0.2, 0.2, 0.9).into()),
        transform: Transform::from_xyz(0.0, 0.5, 2.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
        ..Default::default()
    });

}
mod camera;

use super::GameState;
use bevy::core::FixedTimestep;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use rand::Rng;

const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_game)
                .with_system(camera::spawn_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(keyboard_input_system)
                .with_system(camera::pan_orbit_camera),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND))
                .with_system(spawn_barnacle_on_whale),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
        );
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct GameCamera;

fn setup_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .insert(OnGameScreen);

    // whale
    // Load OBJ file
    let whale_mesh_handle = asset_server.load("models/whale.obj");
    commands
        .spawn_bundle(PbrBundle {
            mesh: whale_mesh_handle,
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("models/whale.png")),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_y(1.)),
            ..Default::default()
        })
        .insert(OnGameScreen);

    // barnacle
    // Load OBJ file
    let barnacle_mesh_handle = asset_server.load("models/barnacle.obj");
    commands
        .spawn_bundle(PbrBundle {
            mesh: barnacle_mesh_handle,
            material: materials.add(Color::rgb(0.2, 0.2, 0.9).into()),
            transform: Transform::from_xyz(0.0, 0.5, 2.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        })
        .insert(OnGameScreen);
}

fn keyboard_input_system(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = game_state.set(GameState::Menu);
    }

    if keyboard_input.pressed(KeyCode::Q) {
        let _ = game_state.set(GameState::Menu);
    }
}

fn spawn_barnacle_on_whale(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..1.0);
    let y = rng.gen_range(0.0..1.0);
    let z = rng.gen_range(0.0..1.0);
    let barnacle_mesh_handle = asset_server.load("models/barnacle.obj");
    commands
        .spawn_bundle(PbrBundle {
            mesh: barnacle_mesh_handle,
            material: materials.add(Color::rgb(0.25, 0.25, 0.1).into()),
            transform: Transform::from_xyz(x, y, z).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..Default::default()
        })
        .insert(OnGameScreen);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

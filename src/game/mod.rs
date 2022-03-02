mod camera;
mod hud;

use super::GameState;
use bevy::prelude::*;
use bevy_mod_picking::*;
use rand::Rng;

#[derive(Default, Clone)]
pub struct BarnacleAttachingMaterials {
    pub hell1: Handle<StandardMaterial>,
    pub hell2: Handle<StandardMaterial>,
    pub hell3: Handle<StandardMaterial>,
}

pub struct BarnacleCount {
    pub count: u32,
}

pub struct GamePlugin;

#[derive(Component)]
pub struct BarnacleSpawnTimer(Timer);

#[derive(Component)]
pub struct BarnacleAttachingTimer(Timer);

#[derive(Component)]
pub struct BarnacleAttachedTimer(Timer);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BarnacleCount { count: 0 })
            .insert_resource(BarnacleAttachingMaterials::default())
            .add_startup_system(setup_attaching_material)
            //.add_startup_system(camera::spawn_camera)
            .add_plugin(hud::GameHUDPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
            .add_plugin(DebugEventsPickingPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_game)
                    .with_system(camera::spawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(keyboard_input_system)
                    .with_system(camera::pan_orbit_camera)
                    .with_system(barnacle_count)
                    .with_system(print_events) //.with_system(hit_barnacle_system),
                    .with_system(update_spawn_timer) //.with_system(hit_barnacle_system),
                    .with_system(update_attached_timers) //.with_system(hit_barnacle_system),
                    .with_system(update_attaching_timers) //.with_system(hit_barnacle_system),
                    .with_system(spawn_barnacle_on_whale) //.with_system(hit_barnacle_system),
                    .with_system(update_attached_state) //.with_system(hit_barnacle_system),
                    .with_system(material_attaching_state), //.with_system(hit_barnacle_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
            )
            .add_system_to_stage(CoreStage::PostUpdate, print_events);
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
pub struct Barnacle {
    pub status: BarnacleStatus,
}

impl Barnacle {
    pub fn new() -> Barnacle {
        Barnacle {
            status: BarnacleStatus::Attaching,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum BarnacleStatus {
    Attaching,
    Attached,
    Gone,
}

fn setup_attaching_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut attaching_materials: ResMut<BarnacleAttachingMaterials>,
) {
    attaching_materials.hell1 = materials.add(Color::rgb(0.25, 0.25, 0.25).into());
    attaching_materials.hell2 = materials.add(Color::rgb(0.5, 0.5, 0.5).into());
    attaching_materials.hell3 = materials.add(Color::rgb(0.75, 0.75, 0.75).into());
}

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
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(OnGameScreen);

    commands
        .spawn()
        .insert(OnGameScreen)
        .insert(BarnacleSpawnTimer(Timer::from_seconds(1.0, true)));
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

fn barnacle_count(mut barnacle_count: ResMut<BarnacleCount>, query: Query<&Barnacle>) {
    barnacle_count.count = query
        .iter()
        .filter(|b| b.status == BarnacleStatus::Attached)
        .count() as u32;
}

fn spawn_barnacle_on_whale(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<&BarnacleSpawnTimer>,
) {
    for spawn_timer in query.iter() {
        if spawn_timer.0.just_finished() {
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
                .insert(OnGameScreen)
                .insert(Barnacle::new())
                .insert(BarnacleAttachedTimer(Timer::from_seconds(5.0, false)))
                .insert(BarnacleAttachingTimer(Timer::from_seconds(0.1, true)))
                .insert_bundle(PickableBundle::default());
        }
    }
}

fn update_attached_state(mut query: Query<(&mut Barnacle, &BarnacleAttachedTimer)>) {
    for (mut barnacle, timer) in query.iter_mut() {
        if timer.0.just_finished() {
            barnacle.status = BarnacleStatus::Attached;
        }
    }
}

fn material_attaching_state(
    attaching_materials: ResMut<BarnacleAttachingMaterials>,
    mut query: Query<(&mut Handle<StandardMaterial>, &BarnacleAttachingTimer)>,
) {
    for (mut material_handle, timer) in query.iter_mut() {
        if timer.0.just_finished() {
            match timer.0.times_finished() % 3 {
                0 => *material_handle = attaching_materials.hell1.clone(),
                1 => *material_handle = attaching_materials.hell2.clone(),
                2 => *material_handle = attaching_materials.hell3.clone(),
                _ => *material_handle = attaching_materials.hell1.clone(),
            };
        }
    }
}

pub fn print_events(mut events: EventReader<PickingEvent>, mut query: Query<&mut Barnacle>) {
    for event in events.iter() {
        match event {
            PickingEvent::Selection(e) => info!("A selection event happened: {:?}", e),
            PickingEvent::Hover(e) => info!("Egads! A hover event!? {:?}", e),
            PickingEvent::Clicked(e) => {
                info!("Gee Willikers, it's a click! {:?}", e);
                let mut barancle = query.get_mut(*e).unwrap();
                barancle.status = BarnacleStatus::Gone;
            }
        }
    }
}

fn update_spawn_timer(time: Res<Time>, mut query: Query<&mut BarnacleSpawnTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn update_attaching_timers(time: Res<Time>, mut query: Query<&mut BarnacleAttachingTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn update_attached_timers(time: Res<Time>, mut query: Query<&mut BarnacleAttachedTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

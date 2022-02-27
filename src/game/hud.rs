use super::super::GameState;
use super::BarnacleCount;
use bevy::prelude::Val::Percent;
use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.5);

pub struct GameHUDPlugin;

#[derive(Component)]
struct HUDRelated;

#[derive(Component)]
struct CountRelated;

#[derive(Component)]
pub struct GameTimer(Timer);

impl Plugin for GameHUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_hud))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(update_timer_text)
                    .with_system(update_barnacle_count_text)
                    .with_system(update_timer),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(super::despawn_screen::<HUDRelated>),
            );
    }
}

fn update_timer_text(mut query: Query<(&mut Text, &GameTimer), With<HUDRelated>>) {
    for (mut text, game_timer) in query.iter_mut() {
        text.sections[0].value = format!("{:.2}", game_timer.0.elapsed_secs());
    }
}

fn update_barnacle_count_text(
    barnacle_count: Res<BarnacleCount>,
    mut query: Query<&mut Text, With<CountRelated>>,
) {
    for (mut text) in query.iter_mut() {
        text.sections[0].value = format!("{:.2}", barnacle_count.count);
    }
}

fn update_timer(time: Res<Time>, mut query: Query<&mut GameTimer>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn setup_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let timer_text_style = TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 40.0,
        color: TEXT_COLOR,
    };

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(HUDRelated);

    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section("0.00", timer_text_style, Default::default()),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(5.0),
                    left: Val::Percent(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameTimer(Timer::from_seconds(86000.0, false)))
        .insert(HUDRelated);

    let count_text_style = TextStyle {
        font: asset_server.load("fonts/Kenney Future.ttf"),
        font_size: 40.0,
        color: TEXT_COLOR,
    };
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "-1",
                count_text_style,
                TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Right,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(5.0),
                    right: Val::Percent(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(HUDRelated)
        .insert(CountRelated);
}

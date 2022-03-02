// disable console opening on windows
#![windows_subsystem = "windows"]

mod game;
mod menu;

use bevy::prelude::*;
use bevy_obj::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Menu,
    Game,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_state(GameState::Game)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

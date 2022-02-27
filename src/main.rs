// disable console opening on windows
#![windows_subsystem = "windows"]

mod menu;
mod game;

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
        .add_state(GameState::Menu)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        //.add_startup_system(load)
        .run();
}
mod actions;
mod consts;
mod loading;
mod map;
mod player;
mod physics;

use crate::actions::ActionsPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::player::PlayerPlugin;
use crate::physics::*;

use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PhysicsPlugin)
            .insert_resource(Gravity(1000.0));

    }
}

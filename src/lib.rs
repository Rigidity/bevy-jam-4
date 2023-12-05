#![allow(clippy::type_complexity)]

mod loader;
mod menu;
mod player;

use crate::loader::LoaderPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((LoaderPlugin, MenuPlugin, PlayerPlugin))
            .insert_resource(ClearColor(Color::rgb(0.3, 0.6, 0.9)))
            .add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameCamera,
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.0,
                near: -1000.0,
                scaling_mode: ScalingMode::FixedHorizontal(1200.0),
                ..default()
            },
            ..default()
        },
    ));
}

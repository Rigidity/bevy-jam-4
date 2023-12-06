#![allow(clippy::type_complexity)]

mod level;
mod loader;
mod menu;
mod mesh_collider;
mod player;

use crate::loader::LoaderPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_xpbd_2d::prelude::*;
use level::{Level, LevelPlugin};

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
            .add_plugins((
                LoaderPlugin,
                MenuPlugin,
                LevelPlugin,
                PlayerPlugin,
                RonAssetPlugin::<Level>::new(&["level.ron"]),
                PhysicsPlugins::default(),
            ))
            .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
            .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
            .insert_resource(Gravity(Vec2::new(0.0, -9.81 * 32.0)))
            .add_systems(Startup, spawn_camera);
    }
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "main.level.ron")]
    main: Handle<Level>,
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
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.05,
            ..default()
        },
    ));
}

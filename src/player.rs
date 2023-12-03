use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct PlayerCamera;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((PlayerCamera, Camera3dBundle::default()))
        .with_children(|parent| {
            parent.spawn(Player);
        });
}

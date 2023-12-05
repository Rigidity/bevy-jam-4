use crate::{GameCamera, GameState};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_splat.run_if(in_state(GameState::InGame)));
    }
}

fn spawn_splat(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (camera, camera_transform) = camera.single();

    let Some(cursor) = window.single().cursor_position().and_then(|cursor| {
        camera
            .viewport_to_world(camera_transform, cursor)
            .map(|ray| ray.origin.truncate())
    }) else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(16.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_xyz(cursor.x, cursor.y, 0.0),
            ..default()
        });
    }
}

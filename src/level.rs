use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_xpbd_2d::prelude::*;
use serde::Deserialize;

use crate::{mesh_collider::collider_from_mesh, GameState, LevelAssets};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_level);
    }
}

#[derive(Asset, TypePath, Deserialize)]
pub struct Level {
    objects: Vec<Object>,
}

impl Level {
    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Object {
    position: Vec2,
    kind: ObjectKind,
}

impl Object {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn kind(&self) -> ObjectKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum ObjectKind {
    Platform {
        size: Vec2,
    },
    Shape {
        color: ShapeColor,
        sides: usize,
        radius: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum ShapeColor {
    Red,
    Green,
    Blue,
}

#[derive(Bundle, Default)]
struct ObjectBundle {
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
    collider: Collider,
    rigidbody: RigidBody,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inhereted_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}

fn setup_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    levels: Res<Assets<Level>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let level = levels.get(level_assets.main.clone()).unwrap();
    let platform_material = materials.add(Color::WHITE.into());
    let red_material = materials.add(Color::rgb(1.0, 0.4, 0.4).into());
    let green_material = materials.add(Color::rgb(0.4, 1.0, 0.4).into());
    let blue_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());

    for object in level.objects() {
        let mut bundle = ObjectBundle {
            transform: Transform::from_translation(object.position().extend(0.0)),
            ..default()
        };

        match object.kind() {
            ObjectKind::Platform { size } => {
                bundle.mesh = meshes.add(Mesh::from(shape::Quad::new(size))).into();
                bundle.material = platform_material.clone();
                bundle.rigidbody = RigidBody::Static;
                bundle.collider = Collider::cuboid(size.x, size.y);
            }
            ObjectKind::Shape {
                color,
                sides,
                radius,
            } => {
                let mesh = Mesh::from(shape::RegularPolygon { sides, radius });
                bundle.material = match color {
                    ShapeColor::Red => red_material.clone(),
                    ShapeColor::Green => green_material.clone(),
                    ShapeColor::Blue => blue_material.clone(),
                };
                bundle.rigidbody = RigidBody::Dynamic;
                bundle.collider = collider_from_mesh(&mesh).unwrap();
                bundle.mesh = meshes.add(mesh).into();
            }
        }

        commands.spawn(bundle);
    }
}

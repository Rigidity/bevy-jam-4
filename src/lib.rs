#![allow(clippy::type_complexity)]

mod level;
mod loader;
mod menu;
mod player;

use crate::loader::LoaderPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::Indices;
use bevy::sprite::Mesh2dHandle;
use bevy::{app::App, render::mesh::VertexAttributeValues};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_xpbd_2d::prelude::*;
use level::{Level, ObjectKind, ShapeColor};
use parry2d::shape::SharedShape;

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
                PlayerPlugin,
                RonAssetPlugin::<Level>::new(&["level.ron"]),
                PhysicsPlugins::default(),
            ))
            .add_collection_to_loading_state::<_, LevelAssets>(GameState::Loading)
            .insert_resource(ClearColor(Color::rgb(0.15, 0.15, 0.15)))
            .insert_resource(Gravity(Vec2::new(0.0, -9.81 * 32.0)))
            .add_systems(Startup, spawn_camera)
            .add_systems(OnEnter(GameState::InGame), setup_level);
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
    let green_material = materials.add(Color::rgb(0.0, 1.0, 0.4).into());
    let blue_material = materials.add(Color::rgb(0.4, 0.4, 1.0).into());

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

fn collider_from_mesh(mesh: &Mesh) -> Option<Collider> {
    extract_mesh_vertices_indices(mesh).map(|(vertices, indices)| {
        SharedShape::trimesh_with_flags(
            vertices.into_iter().map(|vertex| vertex.xy()).collect(),
            indices,
            TriMeshFlags::MERGE_DUPLICATE_VERTICES,
        )
        .into()
    })
}

type VerticesIndices = (Vec<nalgebra::Point3<f32>>, Vec<[u32; 3]>);

fn extract_mesh_vertices_indices(mesh: &Mesh) -> Option<VerticesIndices> {
    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    let indices = mesh.indices()?;

    let vtx: Vec<_> = match vertices {
        VertexAttributeValues::Float32(vtx) => {
            Some(vtx.chunks(3).map(|v| [v[0], v[1], v[2]].into()).collect())
        }
        VertexAttributeValues::Float32x3(vtx) => {
            Some(vtx.iter().map(|v| [v[0], v[1], v[2]].into()).collect())
        }
        _ => None,
    }?;

    let idx = match indices {
        Indices::U16(idx) => idx
            .chunks_exact(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
        Indices::U32(idx) => idx.chunks_exact(3).map(|i| [i[0], i[1], i[2]]).collect(),
    };

    Some((vtx, idx))
}

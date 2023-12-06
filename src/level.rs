use bevy::prelude::*;
use serde::Deserialize;

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

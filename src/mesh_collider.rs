use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_xpbd_2d::prelude::*;
use parry2d::shape::SharedShape;

pub fn collider_from_mesh(mesh: &Mesh) -> Option<Collider> {
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

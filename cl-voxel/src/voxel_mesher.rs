use godot::prelude::*;
use godot::classes::{ArrayMesh, Node3D};
use crate::meshers::{MesherType, create_mesher};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VoxelMesher {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for VoxelMesher {
    fn init(base: Base<Node3D>) -> Self {
        Self { base }
    }
}

#[godot_api]
impl VoxelMesher {
    /// Generate a mesh from an SDF field
    ///
    /// # Parameters
    /// - `mesher_type`: Type of mesher to use (e.g., SurfaceNets)
    /// - `chunk_size`: Size of the voxel chunk in voxels (e.g., 16x16x16)
    /// - `sdf_data`: A PackedFloat32Array containing SDF values in XZY order
    ///
    /// # Returns
    /// The generated ArrayMesh
    #[func]
    pub fn generate_mesh(&mut self, mesher_type: MesherType, chunk_size: Vector3i, sdf_data: PackedFloat32Array) -> Gd<ArrayMesh> {
        let mut mesher = create_mesher(mesher_type);
        mesher.generate_mesh(
            sdf_data,
            chunk_size,
            1.0,  // voxel_size always 1
            0.0,  // iso_level always 0
        ).unwrap_or_else(|| ArrayMesh::new_gd())
    }
}

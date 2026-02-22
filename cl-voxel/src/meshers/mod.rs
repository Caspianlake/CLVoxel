use godot::prelude::*;
use godot::classes::ArrayMesh;

pub mod surface_nets;

/// Enum for selecting mesher type in Godot editor
#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, PartialEq)]
#[godot(via = i32)]
pub enum MesherType {
    SurfaceNets = 0,
}

impl Default for MesherType {
    fn default() -> Self {
        MesherType::SurfaceNets
    }
}

/// Trait that all mesher implementations must implement
pub trait Mesher: Send + Sync {
    /// Generate a mesh from an SDF field
    /// 
    /// # Parameters
    /// - `sdf_data`: PackedFloat32Array containing SDF values in XZY order
    /// - `chunk_size`: Size of the voxel chunk in voxels
    /// - `voxel_size`: Size of each voxel in world units
    /// - `iso_level`: The SDF threshold for the surface (typically 0.0)
    fn generate_mesh(
        &mut self,
        sdf_data: PackedFloat32Array,
        chunk_size: Vector3i,
        voxel_size: f32,
        iso_level: f32,
    ) -> Option<Gd<ArrayMesh>>;
}

/// Factory function to create a mesher based on type
pub fn create_mesher(mesher_type: MesherType) -> Box<dyn Mesher> {
    match mesher_type {
        MesherType::SurfaceNets => Box::new(surface_nets::SurfaceNetsMesher::new()),
    }
}

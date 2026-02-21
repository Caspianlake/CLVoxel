use godot::prelude::*;
use godot::classes::{ArrayMesh, Material, Mesh, MeshInstance3D};
use crate::meshers::{Mesher, MesherType, create_mesher};

/// VoxelMesher node - Main interface for voxel mesh generation
/// 
/// This node provides a high-level interface for generating meshes from SDF fields.
/// It supports multiple meshing algorithms that can be selected via the mesher_type property.
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VoxelMesher {
    /// Type of mesher to use (e.g., SurfaceNets)
    #[export]
    #[var(get, set = set_mesher_type)]
    mesher_type: MesherType,
    
    /// Size of the voxel chunk in voxels (e.g., 16x16x16)
    #[export]
    chunk_size: Vector3i,
    
    /// Size of each voxel in world units
    #[export]
    #[var(get, set = set_voxel_size)]
    voxel_size: f32,
    
    /// SDF iso-level threshold for surface extraction (typically 0.0)
    #[export]
    iso_level: f32,
    
    /// Material to apply to generated meshes
    #[export]
    material: Option<Gd<Material>>,
    
    base: Base<Node3D>,
    
    /// Internal mesher instance (not exposed to Godot)
    current_mesher: Option<Box<dyn Mesher>>,
    
    /// Optional MeshInstance3D child for displaying the generated mesh
    mesh_instance: Option<Gd<MeshInstance3D>>,
}

#[godot_api]
impl INode3D for VoxelMesher {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            mesher_type: MesherType::default(),
            chunk_size: Vector3i::new(16, 16, 16),
            voxel_size: 1.0,
            iso_level: 0.0,
            material: None,
            base,
            current_mesher: None,
            mesh_instance: None,
        }
    }
    
    fn ready(&mut self) {
        // Initialize the mesher based on the selected type
        self.update_mesher();
        
        // Create a MeshInstance3D child if it doesn't exist
        self.ensure_mesh_instance();
    }
}

#[godot_api]
impl VoxelMesher {
    /// Generate a mesh from an SDF field
    /// 
    /// # Parameters
    /// - `sdf_evaluator`: A Callable that takes a Vector3 position and returns a float distance
    /// 
    /// # Returns
    /// The generated ArrayMesh, or null if generation failed
    #[func]
    pub fn generate_mesh(&mut self, sdf_evaluator: Callable) -> Option<Gd<ArrayMesh>> {
        if let Some(mesher) = &mut self.current_mesher {
            let mesh = mesher.generate_mesh(
                sdf_evaluator,
                self.chunk_size,
                self.voxel_size,
                self.iso_level,
            );
            
            // If we have a mesh instance, update it
            if let (Some(generated_mesh), Some(mesh_inst)) = (&mesh, &mut self.mesh_instance) {
                // Upcast ArrayMesh to Mesh and pass by reference
                let mesh_upcast: Gd<Mesh> = generated_mesh.clone().upcast();
                mesh_inst.set_mesh(&mesh_upcast);
                
                // Apply material if set
                if let Some(mat) = &self.material {
                    mesh_inst.set_surface_override_material(0, mat);
                }
            }
            
            mesh
        } else {
            godot_error!("VoxelMesher: No mesher initialized!");
            None
        }
    }
    
    /// Update the mesh using the current SDF (requires storing SDF reference)
    #[func]
    pub fn update_mesh(&mut self) {
        godot_print!("VoxelMesher: update_mesh called (requires SDF to be set)");
        // TODO: Implement when we add SDF storage
    }
    
    /// Set the mesher type and reinitialize the mesher
    #[func]
    pub fn set_mesher_type(&mut self, new_type: MesherType) {
        if self.mesher_type != new_type {
            self.mesher_type = new_type;
            self.update_mesher();
            godot_print!("VoxelMesher: Mesher type changed to {:?}", new_type);
        }
    }
    
    /// Set the voxel size and optionally regenerate mesh
    #[func]
    pub fn set_voxel_size(&mut self, size: f32) {
        if size > 0.0 {
            self.voxel_size = size;
        } else {
            godot_warn!("VoxelMesher: Voxel size must be positive, ignoring value: {}", size);
        }
    }
    
    /// Get the current mesh instance (if any)
    #[func]
    pub fn get_mesh_instance(&self) -> Option<Gd<MeshInstance3D>> {
        self.mesh_instance.clone()
    }
    
    /// Internal: Update the mesher instance based on current type
    fn update_mesher(&mut self) {
        self.current_mesher = Some(create_mesher(self.mesher_type));
        godot_print!("VoxelMesher: Initialized {:?} mesher", self.mesher_type);
    }
    
    /// Internal: Ensure a MeshInstance3D child exists
    fn ensure_mesh_instance(&mut self) {
        if self.mesh_instance.is_none() {
            let mut mesh_inst = MeshInstance3D::new_alloc();
            mesh_inst.set_name("MeshInstance3D");
            
            // Add as child
            self.base_mut().add_child(&mesh_inst);
            
            self.mesh_instance = Some(mesh_inst);
            godot_print!("VoxelMesher: Created MeshInstance3D child");
        }
    }
}

use godot::prelude::*;

mod meshers;
mod voxel_mesher;

struct ClVoxelExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ClVoxelExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // Register custom classes here
            godot_print!("CLVoxel extension initialized");
        }
    }
}

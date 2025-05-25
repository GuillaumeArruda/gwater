use godot::prelude::*;

struct GWaterExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GWaterExtension {}

mod nodes;
mod utils;
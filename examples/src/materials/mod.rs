mod custom;
use bevy::prelude::Plugin;
pub use custom::*;

pub struct MaterialPlugin;


impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(CustomMaterialPlugin);
    }
}
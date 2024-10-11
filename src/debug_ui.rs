use crate::prelude::*;

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: we need to wait for bevy 0.15 rc1 and then for bevy_egui targeting that rc...
        // if !app.is_plugin_added::<EguiPlugin>() {
        //     app.add_plugins(EguiPlugin);
        // }
    }
}

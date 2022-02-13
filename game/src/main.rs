use bevy::prelude::*;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .run();
}

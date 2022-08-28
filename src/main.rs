use bevy::asset::AssetServerSettings;
use bevy::prelude::*;

fn main() {
    App::new()
        // Enable hot reloading
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}

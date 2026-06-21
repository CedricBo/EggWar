use bevy::{
    app::{App, PluginGroup},
    image::ImageSamplerDescriptor,
    picking::mesh_picking::MeshPickingPlugin,
};

pub mod buildings;
pub mod camera_plugin;
pub mod game_plugin;
pub mod ground;
pub mod placing_building_plugin;
pub mod player_plugin;

pub fn run() {
    let mut app = App::new();

    app.add_plugins((
        bevy::DefaultPlugins {}.set(bevy::image::ImagePlugin::default_nearest()),
        MeshPickingPlugin,
        camera_plugin::CameraPlugin,
        game_plugin::GamePlugin,
        player_plugin::plugin::PlayerPlugin,
    ));

    app.run();
}

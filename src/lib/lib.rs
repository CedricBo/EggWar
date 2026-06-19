use bevy::{app::App, picking::mesh_picking::MeshPickingPlugin};

pub mod buildings;
pub mod camera_plugin;
pub mod game_plugin;
pub mod ground;
pub mod placing_building_plugin;

pub fn run() {
    let mut app = App::new();

    app.add_plugins((
        bevy::DefaultPlugins,
        MeshPickingPlugin,
        camera_plugin::CameraPlugin,
        game_plugin::GamePlugin,
    ));

    app.run();
}

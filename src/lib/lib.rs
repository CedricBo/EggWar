use bevy::app::App;

pub mod buildings;
pub mod camera_plugin;
pub mod game_plugin;

pub fn run() {
    let mut app = App::new();

    app.add_plugins((
        bevy::DefaultPlugins,
        camera_plugin::CameraPlugin,
        game_plugin::GamePlugin,
    ));

    app.run();
}

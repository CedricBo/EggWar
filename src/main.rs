use bevy::app::App;

mod camera_plugin;
mod common;
mod game_plugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        bevy::DefaultPlugins,
        camera_plugin::CameraPlugin,
        game_plugin::GamePlugin,
    ));

    app.run();
}

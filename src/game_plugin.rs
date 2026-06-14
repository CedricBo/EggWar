use bevy::app::Plugin;

#[path = "./game_world_plugin.rs"]
mod game_world_plugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(game_world_plugin::GameWorldPlugin);
    }
}

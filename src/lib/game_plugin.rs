use bevy::app::Plugin;

#[path = "./game_world_plugin.rs"]
mod game_world_plugin;

#[path = "./placing_building_plugin.rs"]
mod placing_building_plugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(game_world_plugin::GameWorldPlugin);
        app.add_plugins(placing_building_plugin::PlacingBuildingPlugin);
    }
}

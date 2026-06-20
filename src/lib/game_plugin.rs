use bevy::{
    app::{Plugin, Update},
    ecs::message::MessageReader, state::state::OnEnter,
};

use crate::placing_building_plugin::{self, messages::OnInPlacingStart, states::InPlacing};

#[path = "./game_world_plugin.rs"]
mod game_world_plugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(game_world_plugin::GameWorldPlugin);
        app.add_plugins(placing_building_plugin::plugin::PlacingBuildingPlugin);

        app.add_systems(Update, on_in_placing_start_by_message);
        app.add_systems(OnEnter(InPlacing), on_in_placing_start_by_state);
    }
}

fn on_in_placing_start_by_message(mut message_reader: MessageReader<OnInPlacingStart>) {
    for _message in message_reader.read() {
        println!("OnInPlacingStart by message");
    }
}

fn on_in_placing_start_by_state() {
    println!("OnInPlacingStart by state");
}

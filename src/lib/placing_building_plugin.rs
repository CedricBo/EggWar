mod buildings;

use bevy::{
    app::{Plugin, Update},
    ecs::system::Res,
    input::{ButtonInput, keyboard::Key},
    state::{app::AppExtStates, state::States},
};
use buildings::building::BuildingType;

pub struct PlacingBuildingPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
enum SelectedBuildingToPlace {
    None,
    Selected(BuildingType),
}

impl Plugin for PlacingBuildingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_state(SelectedBuildingToPlace::None);

        app.add_systems(Update, change_selected_building_with_keyboard);
    }
}

fn change_selected_building_with_keyboard(keyboard_inputs: Res<ButtonInput<Key>>) {
    for pressed in keyboard_inputs.get_just_pressed() {
        println!("Pressed {:?}", pressed);
    }
}

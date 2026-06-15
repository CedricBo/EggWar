mod buildings;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{IntoScheduleConfigs, common_conditions::resource_changed},
        system::{Res, ResMut},
    },
    input::{
        ButtonInput,
        keyboard::{Key, KeyCode}, mouse::MouseButton,
    },
    state::{
        app::AppExtStates,
        state::{NextState, State, States},
    },
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
        app.add_systems(
            Update,
            print_selected_on_change.run_if(resource_changed::<State<SelectedBuildingToPlace>>),
        );
    }
}

fn change_selected_building_with_keyboard(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    mut selected: ResMut<NextState<SelectedBuildingToPlace>>,
) {
    if keyboard_inputs.just_pressed(KeyCode::Digit1) {
        selected.set(SelectedBuildingToPlace::Selected(BuildingType::Grange));
    }

    if keyboard_inputs.just_pressed(KeyCode::Digit2) {
        selected.set(SelectedBuildingToPlace::Selected(BuildingType::Garden));
    }

    if keyboard_inputs.just_pressed(KeyCode::Digit3) {
        selected.set(SelectedBuildingToPlace::Selected(BuildingType::Turret));
    }

    if keyboard_inputs.just_pressed(KeyCode::Escape) || mouse_inputs.just_pressed(MouseButton::Right) {
        selected.set(SelectedBuildingToPlace::None);
    }
}

fn print_selected_on_change(state: Res<State<SelectedBuildingToPlace>>) {
    println!("{:?}", state);
}

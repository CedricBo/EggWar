use bevy::{
    app::{Plugin, Update}, camera::Camera, color::Color, ecs::{
        component::Component,
        query::With,
        schedule::{IntoScheduleConfigs, SystemCondition, common_conditions::resource_changed},
        system::{Commands, Res, ResMut, Single},
    }, gizmos::gizmos::Gizmos, input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton}, math::{Isometry2d, Vec2, Vec3Swizzles}, state::{
        app::AppExtStates,
        condition::in_state,
        state::{NextState, OnExit, State, States},
        state_scoped::DespawnOnEnter,
    }, transform::components::{GlobalTransform, Transform}, window::Window
};
use crate::buildings::building::BuildingType;

use crate::{buildings::building::Building, ground::Ground};

pub struct PlacingBuildingPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
enum SelectedBuildingToPlace {
    None,
    Selected(BuildingType),
}

#[derive(Component)]
struct Placeholder;

impl Plugin for PlacingBuildingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_state(SelectedBuildingToPlace::None);

        app.add_systems(Update, change_selected_building_with_keyboard);
        app.add_systems(
            Update,
            print_selected_on_change.run_if(resource_changed::<State<SelectedBuildingToPlace>>),
        );

        app.add_systems(OnExit(SelectedBuildingToPlace::None), create_placeholder);

        app.add_systems(
            Update,
            move_placeholder.run_if(
                in_state(SelectedBuildingToPlace::Selected(BuildingType::Garden))
                    .or(in_state(SelectedBuildingToPlace::Selected(
                        BuildingType::Grange,
                    )))
                    .or(in_state(SelectedBuildingToPlace::Selected(
                        BuildingType::Turret,
                    ))),
            ),
        );

        app.add_systems(Update, draw_placeholder_gizmos);
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

    if keyboard_inputs.just_pressed(KeyCode::Escape)
        || mouse_inputs.just_pressed(MouseButton::Right)
    {
        selected.set(SelectedBuildingToPlace::None);
    }
}

fn print_selected_on_change(state: Res<State<SelectedBuildingToPlace>>) {
    println!("{:?}", state);
}

fn create_placeholder(mut command: Commands) {
    println!("Create placeholder");
    command.spawn((
        Transform::default(),
        Placeholder,
        DespawnOnEnter(SelectedBuildingToPlace::None),
    ));
}

fn move_placeholder(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut placeholder: Single<&mut Transform, With<Placeholder>>,
    ground: Single<&GlobalTransform, With<Ground>>
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a world position based on the cursor's position.
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        placeholder.translation.x = cursor_world_pos.x;
        placeholder.translation.y = ground.translation().y;
    }
}

fn draw_placeholder_gizmos(
    mut gizmos: Gizmos,
    placeholder: Single<&Transform, With<Placeholder>>,
    state: Res<State<SelectedBuildingToPlace>>
)
{
    if let SelectedBuildingToPlace::Selected(btype) = *state.get()
    {
        let size = Building::size_for_type(btype);
        
        gizmos.rect_2d(Isometry2d::from_translation(placeholder.translation.xy()), Vec2::new(size.0,size.1), Color::linear_rgb(1.0, 0.0, 0.0));
    }

}
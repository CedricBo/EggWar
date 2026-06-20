use crate::{
    buildings::{building::BuildingType, plugin::PlaceBuilding},
    ground::Ground,
    placing_building_plugin::{
        messages::OnInPlacingStart,
        states::{InPlacing, SelectedBuildingToPlace},
    },
};

use bevy::{
    app::{Plugin, Update},
    camera::Camera,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        message::MessageWriter,
        query::{With, Without},
        schedule::{
            IntoScheduleConfigs, SystemCondition,
            common_conditions::{resource_added, resource_changed, resource_removed},
        },
        system::{Commands, Query, Res, ResMut, Single},
    },
    gizmos::gizmos::Gizmos,
    input::{ButtonInput, keyboard::KeyCode, mouse::MouseButton},
    math::{Isometry2d, Vec2, Vec3Swizzles},
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{NextState, OnEnter, State},
        state_scoped::DespawnOnExit,
    },
    transform::components::{GlobalTransform, Transform},
    window::Window,
};

use crate::buildings::building::Building;

pub struct PlacingBuildingPlugin;

#[derive(Component)]
struct Placeholder;

#[derive(Component)]
struct PlaceholderCollision;

impl Plugin for PlacingBuildingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_state(SelectedBuildingToPlace::None);
        app.add_computed_state::<InPlacing>();
        app.add_message::<OnInPlacingStart>();

        app.add_systems(Update, change_selected_building_with_keyboard);
        app.add_systems(
            Update,
            (
                print_selected_on_change.run_if(resource_changed::<State<SelectedBuildingToPlace>>),
                print_in_placing_on_change.run_if(
                    resource_added::<State<InPlacing>>.or(resource_removed::<State<InPlacing>>),
                ),
            ),
        );

        app.add_systems(
            OnEnter(InPlacing),
            (create_placeholder, emit_start_placing_message),
        );
        app.add_systems(
            Update,
            (update_placeholder, place_building_on_click)
                .chain()
                .after(create_placeholder)
                .run_if(in_state(InPlacing)),
        );

        // app.register_system(update_placeholder);

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
        selected.set_if_neq(SelectedBuildingToPlace::None);
    }
}

fn emit_start_placing_message(mut message_writer: MessageWriter<OnInPlacingStart>) {
    message_writer.write(OnInPlacingStart);
}

fn print_selected_on_change(state: Res<State<SelectedBuildingToPlace>>) {
    println!("Selected: {:?}", state);
}

fn print_in_placing_on_change(state: Option<Res<State<InPlacing>>>) {
    println!("InPlace: {:?}", state);
}

fn create_placeholder(mut command: Commands) {
    command.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Placeholder,
        DespawnOnExit(InPlacing),
    ));

    command.run_system_cached(update_placeholder);
}

fn update_placeholder(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut commands: Commands,
    placeholder_query: Single<(Entity, &mut Transform), With<Placeholder>>,
    ground_query: Single<&GlobalTransform, With<Ground>>,
    buildings: Query<(&Building, &GlobalTransform)>,
    state: Res<State<SelectedBuildingToPlace>>,
) {
    let (camera, camera_transform) = *camera_query;
    let (entity, mut placeholder_transform) = placeholder_query.into_inner();

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        && let SelectedBuildingToPlace::Selected(btype) = **state
    {
        placeholder_transform.translation.x = cursor_world_pos.x;

        let size = Building::size_for_type(btype);

        placeholder_transform.translation.y = ground_query.translation().y + (size.1 / 2.0);

        let intersect = buildings
            .iter()
            .any(|item| is_intersect_building(placeholder_transform.translation.x, size.0, item));

        let mut placeholder_entity = commands.entity(entity);

        if intersect {
            placeholder_entity.insert_if_new(PlaceholderCollision);
        } else {
            placeholder_entity.remove::<PlaceholderCollision>();
        }
    }
}

fn place_building_on_click(
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    state: Res<State<SelectedBuildingToPlace>>,
    mut place_building_writer: MessageWriter<PlaceBuilding>,
    placeholder: Single<&Transform, (With<Placeholder>, Without<PlaceholderCollision>)>,
) {
    if mouse_inputs.just_pressed(MouseButton::Left)
        && let SelectedBuildingToPlace::Selected(building_type) = **state
    {
        place_building_writer.write(PlaceBuilding {
            building_type,
            x: placeholder.translation.x,
        });
    }
}

fn draw_placeholder_gizmos(
    mut gizmos: Gizmos,
    placeholder: Single<(Entity, &Transform, Option<&PlaceholderCollision>), With<Placeholder>>,
    state: Res<State<SelectedBuildingToPlace>>,
) {
    if let SelectedBuildingToPlace::Selected(btype) = *state.get() {
        let size = Building::size_for_type(btype);

        let color = match placeholder.2 {
            Some(_) => Color::linear_rgb(1.0, 0.0, 0.0),
            None => Color::linear_rgb(0.0, 1.0, 0.0),
        };

        gizmos.rect_2d(
            Isometry2d::from_translation(placeholder.1.translation.xy()),
            Vec2::new(size.0, size.1),
            color,
        );
    }
}

fn is_intersect_building(
    position: f32,
    size: f32,
    building_and_transform: (&Building, &GlobalTransform),
) -> bool {
    let building_position = building_and_transform.1.translation().x;
    let building_size = building_and_transform.0.size().0;

    (building_position - position).abs() <= (size + building_size) / 2.0
}

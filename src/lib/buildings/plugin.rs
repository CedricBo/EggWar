use bevy::{mesh::RectangleMeshBuilder, prelude::*};

use crate::{
    buildings::building::{Building, BuildingComponent, BuildingType},
    core::components::{Blockable, Size},
    ground::Ground,
    placing_building_plugin::states::InPlacing,
};

#[derive(Message)]
pub struct PlaceBuilding {
    pub building_type: BuildingType,
    pub position: Vec2,
}

#[derive(Component)]
struct Selected;

#[derive(Component)]
pub struct StandMarker;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<PlaceBuilding>();

        app.add_systems(FixedUpdate, gizmos);
        app.add_systems(Startup, init_buildings);

        app.add_systems(Update, (place_buildings, deselect_on_right_click));
        app.add_systems(OnEnter(InPlacing), unselect_building);
    }
}

fn gizmos(
    mut gizmos: Gizmos,
    buildings: Query<(&GlobalTransform, &BuildingComponent, Option<&Selected>)>,
) {
    gizmos.circle_2d(Isometry2d::default(), 5.0, Color::linear_rgb(0.0, 1.0, 0.0));

    for (transform, building, selected) in buildings.iter() {
        let size = building.size();
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation().xy()),
            Vec2::new(size.x, size.y),
            match selected {
                Some(_) => Color::linear_rgb(0.0, 1.0, 0.0),
                None => Color::linear_rgb(0.0, 0.0, 1.0),
            },
        );

        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation().xy()),
            5.0,
            Color::linear_rgb(1.0, 1.0, 0.0),
        );
    }
}

pub fn init_buildings(mut place_building_writer: MessageWriter<PlaceBuilding>) {
    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Grange,
        position: (-50.0, 100.0).into(),
    });

    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Garden,
        position: (-250.0, 0.0).into(),
    });

    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Stand,
        position: (350.0, 150.0).into(),
    });
}

fn place_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut place_buildings_reader: MessageReader<PlaceBuilding>,
) {
    for PlaceBuilding {
        building_type,
        position,
    } in place_buildings_reader.read()
    {
        let path: String = BuildingComponent::path_for_type(*building_type).into();

        let mut entity_command = commands.spawn_empty();

        BuildingComponent::spawn_type(*building_type, &mut entity_command);

        entity_command
            .insert((
                Visibility::Visible,
                Transform::from_translation(position.extend(0.0)),
                Sprite::from_image(asset_server.load(path)),
            ))
            .observe(on_click_building.run_if(not(in_state(InPlacing))));
    }
}

fn on_click_building(
    click: On<Pointer<Click>>,
    selected: Option<Single<Entity, With<Selected>>>,
    mut commands: Commands,
) {
    if click.button == PointerButton::Primary {
        if let Some(selected) = selected
            && let Ok(mut entity_command) = commands.get_entity(*selected)
        {
            entity_command.remove::<Selected>();
        }

        if let Ok(mut entity_command) = commands.get_entity(click.entity) {
            entity_command.insert_if_new(Selected);
        }
    }
}

fn deselect_on_right_click(mouse_button: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if mouse_button.just_pressed(MouseButton::Right) {
        commands.run_system_cached(unselect_building);
    }
}

fn unselect_building(selected: Option<Single<Entity, With<Selected>>>, mut commands: Commands) {
    if let Some(selected) = selected
        && let Ok(mut entity_command) = commands.get_entity(*selected)
    {
        entity_command.remove::<Selected>();
    }
}

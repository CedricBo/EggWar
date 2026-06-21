use bevy::{mesh::RectangleMeshBuilder, prelude::*};

use crate::{
    buildings::building::{Building, BuildingType},
    ground::Ground,
    placing_building_plugin::states::InPlacing,
};

#[derive(Message)]
pub struct PlaceBuilding {
    pub building_type: BuildingType,
    pub x: f32,
}

#[derive(Component)]
struct Selected;

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

fn gizmos(mut gizmos: Gizmos, buildings: Query<(&GlobalTransform, &Building, Option<&Selected>)>) {
    gizmos.circle_2d(Isometry2d::default(), 5.0, Color::linear_rgb(0.0, 1.0, 0.0));

    for (transform, building, selected) in buildings.iter() {
        let (width, height) = building.size();
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation().xy()),
            Vec2::new(width, height),
            match selected {
                Some(_) => Color::linear_rgb(0.0, 1.0, 0.0),
                None => Color::linear_rgb(0.0, 0.0, 1.0),
            },
        );
    }
}

pub fn init_buildings(mut place_building_writer: MessageWriter<PlaceBuilding>) {
    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Grange,
        x: -50.0,
    });

    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Grange,
        x: 200.0,
    });

    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Garden,
        x: -250.0,
    });

    place_building_writer.write(PlaceBuilding {
        building_type: BuildingType::Turret,
        x: 350.0,
    });
}

fn place_buildings(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut place_buildings_reader: MessageReader<PlaceBuilding>,
    ground: Single<&GlobalTransform, With<Ground>>,
) {
    let ground_translation = ground.translation();

    for place_building in place_buildings_reader.read() {
        let building = Building::from(place_building.building_type.clone());

        let (width, height) = building.size();
        let path: String = building.asset_path().into();

        let mesh = meshes.add(RectangleMeshBuilder::new(width, height));
        let material = materials.add(Color::linear_rgb(0.0, 0.0, 0.0));

        commands
            .spawn((
                Visibility::Visible,
                Transform::from_translation(ground_translation + Vec3::new(0.0, height / 2.0, 0.0)),
                children![(
                    building,
                    children![(
                        Mesh2d(mesh),
                        MeshMaterial2d(material),
                        Transform::from_translation(-Vec3::Z)
                    )],
                    Transform::from_translation(Vec3::new(place_building.x, 0.0, 0.0)),
                    Sprite::from_image(asset_server.load(path)),
                )],
            ))
            .observe(on_click_building);
    }
}

fn on_click_building(
    click: On<Pointer<Click>>,
    childrens: Query<&Children>,
    selected: Option<Single<Entity, With<Selected>>>,
    mut commands: Commands,
    in_placing_state: Option<Res<State<InPlacing>>>,
) {
    if click.button == PointerButton::Primary
        && let None = in_placing_state
    {
        if let Some(selected) = selected
            && let Ok(mut entity_command) = commands.get_entity(*selected)
        {
            entity_command.remove::<Selected>();
        }

        if let Ok(childs) = childrens.get(click.entity)
            && let Some(child) = childs.first()
        {
            if let Ok(mut entity_command) = commands.get_entity(*child) {
                entity_command.insert_if_new(Selected);
            }
        }
    }
}

fn deselect_on_right_click(
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut commands: Commands
)
{
    if mouse_button.just_pressed(MouseButton::Right)
    {
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

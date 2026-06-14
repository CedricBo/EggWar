use bevy::{mesh::RectangleMeshBuilder, prelude::*};

use crate::game_plugin::game_world_plugin::{
    Ground,
    buildings::building::{Building, BuildingType},
};

#[derive(Message)]
pub struct PlaceBuilding {
    building_type: BuildingType,
    x: f32,
}

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<PlaceBuilding>();

        app.add_systems(FixedUpdate, gizmos);
        app.add_systems(Startup, init_buildings);

        app.add_systems(Update, place_buildings);
    }
}

fn gizmos(mut gizmos: Gizmos, buildings: Query<(&GlobalTransform, &Building)>) {
    gizmos.circle_2d(Isometry2d::default(), 5.0, Color::linear_rgb(0.0, 1.0, 0.0));

    for (transform, building) in buildings.iter() {
        let (width, height) = building.size();
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation().xy()),
            Vec2::new(width, height),
            Color::linear_rgb(0.0, 0.0, 1.0),
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
    ground: Single<&GlobalTransform, With<Ground>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut place_buildings_reader: MessageReader<PlaceBuilding>,
) {
    for place_building in place_buildings_reader.read() {
        let building = Building::from(place_building.building_type.clone());

        let (width, height) = building.size();
        let path: String = building.asset_path().into();

        let mesh = meshes.add(RectangleMeshBuilder::new(width, height));
        let material = materials.add(Color::linear_rgb(0.0, 0.0, 0.0));

        let ground_translation = ground.translation();

        commands.spawn((
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
        ));
    }
}

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    camera::visibility::{Layer, RenderLayers, Visibility},
    color::Color,
    ecs::{
        bundle::Bundle,
        children,
        component::Component,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut, Single},
    },
    gizmos::{
        config::{DefaultGizmoConfigGroup, GizmoConfigStore},
        gizmos::Gizmos,
    },
    math::{Isometry2d, Vec2, Vec3, Vec3Swizzles},
    mesh::{Mesh, Mesh2d, RectangleMeshBuilder},
    sprite::Sprite,
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

#[derive(Component)]
pub struct Ground;

#[derive(Component, Clone)]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct Building;

impl Building {
    fn Grange() -> impl Bundle {
        (
            Building,
            Size {
                width: 100.0,
                height: 100.0,
            },
        )
    }
}

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, (init_ground, init_buildings).chain());

        app.add_systems(Update, (gizmos, update_gizmos_config));
    }
}

fn init_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = Size {
        width: 10000.0,
        height: 50.0,
    };

    let mesh = meshes.add(RectangleMeshBuilder::new(size.width, size.height));

    let material = materials.add(Color::linear_rgb(38.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0));

    commands.spawn((
        Ground,
        Transform::default(),
        Visibility::Visible,
        children![(
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(Vec3::new(0.0, -size.height / 2.0, 0.0)),
        )],
    ));
}

fn init_buildings(
    mut commands: Commands,
    ground: Single<&Transform, With<Ground>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let building = Building::Grange();

    let mesh = meshes.add(RectangleMeshBuilder::new(size.width, size.height));
    let material = materials.add(Color::linear_rgb(0.0, 0.0, 0.0));

    let ground_translation = ground.translation;

    commands.spawn((
        building,
        Transform::from_translation(ground_translation + Vec3::new(10.0, size.height / 2.0, 0.0)),
        Visibility::Visible,
        children![(
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Sprite::from_image(asset_server.load("./grange.png"))
        )],
    ));
}

fn gizmos(mut gizmos: Gizmos, buildings: Query<(&Transform, &Size), With<Building>>) {
    gizmos.circle_2d(Isometry2d::default(), 5.0, Color::linear_rgb(0.0, 1.0, 0.0));

    for (transform, size) in buildings.iter() {
        gizmos.rect_2d(
            Isometry2d::from_translation(transform.translation.xy()),
            Vec2::new(size.width, size.height),
            Color::linear_rgb(0.0, 0.0, 1.0),
        );
    }
}

fn update_gizmos_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    config.line.width = 5.0;
    config.enabled = true;
    config.depth_bias = 1.0;
}

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    camera::visibility::Visibility,
    color::Color,
    ecs::{
        children,
        component::Component,
        schedule::IntoScheduleConfigs,
        system::{Commands, ResMut},
    },
    math::Vec3,
    mesh::{Mesh, Mesh2d, RectangleMeshBuilder},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{common::Size, game_plugin::game_world_plugin::buildings::plugins::init_buildings};

mod buildings;

#[derive(Component)]
pub struct Ground;

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_ground.before(init_buildings));

        app.add_plugins(buildings::plugins::BuildingsPlugin);
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

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    camera::visibility::Visibility,
    color::Color,
    ecs::{
        children,
        schedule::IntoScheduleConfigs,
        system::{Commands, ResMut},
    },
    math::Vec3,
    mesh::{Mesh, Mesh2d, RectangleMeshBuilder},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{buildings::{self, plugin::init_buildings}, ground::Ground};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_ground.before(init_buildings));

        app.add_plugins(buildings::plugin::BuildingsPlugin);
    }
}

fn init_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = (10000.0, 50.0);

    let mesh = meshes.add(RectangleMeshBuilder::new(size.0, size.1));

    let material = materials.add(Color::linear_rgb(38.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0));

    commands.spawn((
        Ground,
        Transform::from_translation(Vec3::ZERO),
        Visibility::Visible,
        children![(
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(Vec3::new(0.0, -size.1 / 2.0, 0.0)),
        )],
    ));
}

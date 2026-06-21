use bevy::{
    app::{Plugin, Startup},
    asset::asset_value,
    camera::visibility::Visibility,
    color::Color,
    ecs::{hierarchy::Children, schedule::IntoScheduleConfigs},
    math::Vec3,
    mesh::{Mesh2d, RectangleMeshBuilder},
    scene::{Scene, SceneList, SpawnListSystem, bsn, bsn_list},
    sprite_render::{ColorMaterial, MeshMaterial2d},
    transform::components::Transform,
};

use crate::{
    buildings::{self, plugin::init_buildings},
    ground::Ground,
};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, scene.spawn().before(init_buildings));

        app.add_plugins(buildings::plugin::BuildingsPlugin);
    }
}

fn scene() -> impl SceneList {
    bsn_list![ground()]
}

fn ground() -> impl Scene {
    let size = (10000.0, 50.0);

    bsn! {
        #Ground
        Ground
        Transform::from_translation(Vec3::ZERO)
        Visibility::Visible
        Children[(
            Mesh2d(asset_value(RectangleMeshBuilder::new(size.0, size.1)))
            MeshMaterial2d<ColorMaterial>(asset_value(Color::linear_rgb(38.0 / 255.0, 153.0 / 255.0, 0.0 / 255.0)))
            Transform::from_translation(Vec3::new(0.0, -size.1 / 2.0, 0.0))
        )]
    }
}

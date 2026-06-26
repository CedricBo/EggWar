use bevy::{
    app::{Plugin, Startup}, asset::{AssetServer, Assets}, ecs::system::{Commands, ResMut}, math::{Vec2, Vec3}, sprite::Sprite, transform::components::Transform
};

use crate::buildings::{self};

pub struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, create_ground);

        app.add_plugins(buildings::plugin::BuildingsPlugin);
    }
}

fn create_ground(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((Sprite {
        image: asset_server.load("grass.png"),
        custom_size: Some(Vec2::new(1000.0, 1000.0)),
        image_mode: bevy::sprite::SpriteImageMode::Tiled { tile_x: true, tile_y: true, stretch_value: 1.0 },
        ..Default::default()
    }, Transform::from_translation(-Vec3::Z)));
}

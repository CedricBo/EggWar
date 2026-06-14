use std::sync::LazyLock;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    camera::Camera2d,
    ecs::{
        query::With,
        system::{Commands, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::Key},
    mesh::Mesh,
    sprite_render::ColorMaterial,
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, init_camera);

        app.add_systems(Update, move_camera);
    }
}

fn init_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
}

fn move_camera(
    keyboard_inputs: Res<ButtonInput<Key>>,
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
) {
    let mut camera_translation = camera_transform.translation;

    static UP_KEY: LazyLock<Key> = LazyLock::new(|| Key::Character("z".into()));
    static DOWN_KEY: LazyLock<Key> = LazyLock::new(|| Key::Character("s".into()));
    static LEFT_KEY: LazyLock<Key> = LazyLock::new(|| Key::Character("q".into()));
    static RIGHT_KEY: LazyLock<Key> = LazyLock::new(|| Key::Character("d".into()));

    if keyboard_inputs.pressed(UP_KEY.clone()) {
        camera_translation.y += 1.0;
    }

    if keyboard_inputs.pressed(DOWN_KEY.clone()) {
        camera_translation.y -= 1.0;
    }

    if keyboard_inputs.pressed(LEFT_KEY.clone()) {
        camera_translation.x -= 1.0;
    }

    if keyboard_inputs.pressed(RIGHT_KEY.clone()) {
        camera_translation.x += 1.0;
    }

    camera_transform.translation = camera_translation;
}

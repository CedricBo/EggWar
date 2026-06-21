use std::sync::LazyLock;

use bevy::{
    app::{Plugin, Startup, Update},
    camera::Camera2d,
    ecs::{
        query::With,
        system::{Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    scene::{SceneList, SpawnListSystem, bsn_list},
    transform::components::Transform,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, scene.spawn());

        app.add_systems(Update, move_camera);
    }
}

fn scene() -> impl SceneList {
    bsn_list![Camera2d]
}

fn move_camera(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
) {
    let mut camera_translation = camera_transform.translation;

    static UP_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyW);
    static DOWN_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyS);
    static LEFT_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyA);
    static RIGHT_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyD);

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

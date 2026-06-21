use std::sync::LazyLock;

use bevy::{
    app::{Plugin, Startup, Update}, asset::{AssetLoader, AssetServer, Assets, Handle}, camera::Camera2d, ecs::{component::Component, query::{With, Without}, system::{Commands, Res, ResMut, Single}}, image::{Image, ImageLoaderSettings, ImageSampler}, input::{ButtonInput, keyboard::KeyCode}, math::Vec2, sprite::Sprite, transform::components::Transform
};

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, create_player);

        app.add_systems(Update, (move_player_with_keyboard, attach_camera_to_player));
    }
}

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // images: ResMut<Assets<Image>>,

) {
    let builder =
        asset_server
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest()
            });

    commands.spawn((
        Sprite {
            image: builder.load("./player.png"),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..Default::default()
        },
        Transform::default(),
        Player,
    ));
}

fn attach_camera_to_player(
    player_transform: Single<&mut Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_transform: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,

) {
    camera_transform.translation = player_transform.translation;
}

fn move_player_with_keyboard(
    keyboard_inputs: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    let mut player_translation = player_transform.translation;

    static UP_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyW);
    static DOWN_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyS);
    static LEFT_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyA);
    static RIGHT_KEY: LazyLock<KeyCode> = LazyLock::new(|| KeyCode::KeyD);

    if keyboard_inputs.pressed(UP_KEY.clone()) {
        player_translation.y += 1.0;
    }

    if keyboard_inputs.pressed(DOWN_KEY.clone()) {
        player_translation.y -= 1.0;
    }

    if keyboard_inputs.pressed(LEFT_KEY.clone()) {
        player_translation.x -= 1.0;
    }

    if keyboard_inputs.pressed(RIGHT_KEY.clone()) {
        player_translation.x += 1.0;
    }

    player_transform.translation = player_translation;
}

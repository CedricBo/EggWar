use std::ops::Deref;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, Single},
    },
    gizmos::gizmos::Gizmos,
    image::{ImageLoaderSettings, ImageSampler},
    input::{ButtonInput, keyboard::KeyCode},
    math::{Isometry2d, Vec2, Vec3Swizzles, VectorSpace},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{core::{
    components::{Blockable, Size},
    utils::is_intersect,
}, player_plugin::plugin::Direction::DOWN};

#[derive(Clone)]
enum Direction {
    TOP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Component)]
#[require(Size = Size::from(Vec2::new(64.0, 64.0)))]
struct Player {
    direction: Direction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, create_player);

        app.add_systems(
            Update,
            (
                move_player_with_keyboard,
                update_player_sprite,
                attach_camera_to_player,
                draw_gizmos,
            )
                .chain(),
        );
    }
}

fn create_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        Player {
            direction: Direction::DOWN
        },
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
    mut player: Single<(&mut Transform, &Size, &mut Player)>,
    blockable: Query<(&Size, &Transform), (With<Blockable>, Without<Player>)>,
) {
    let mut movement = Vec2::ZERO;
    let mut direction = player.2.direction.clone();

    const UP_KEY: KeyCode = KeyCode::KeyW;
    const DOWN_KEY: KeyCode = KeyCode::KeyS;
    const LEFT_KEY: KeyCode = KeyCode::KeyA;
    const RIGHT_KEY: KeyCode = KeyCode::KeyD;

    if keyboard_inputs.pressed(UP_KEY) {
        movement.y += 1.0;
        direction = Direction::TOP;
    }

    if keyboard_inputs.pressed(DOWN_KEY) {
        movement.y -= 1.0;
        direction = Direction::DOWN;
    }

    if keyboard_inputs.pressed(LEFT_KEY) {
        movement.x -= 1.0;
        direction = Direction::LEFT;
    }

    if keyboard_inputs.pressed(RIGHT_KEY) {
        movement.x += 1.0;
        direction = Direction::RIGHT;
    }

    if movement != Vec2::ZERO {
        let future_translation = player.0.translation.xy() + movement;

        let intersected = blockable.iter().find(|(size, transform)| {
            is_intersect(
                (future_translation, player.1.into()),
                (transform.translation.xy(), (*size).into()),
            )
        });

        if let Some(intersected) = intersected {
            println!("{:?}", intersected.1);
            let resolved_movement = [movement.clone(), movement.with_x(0.0), movement.with_y(0.0)]
                .into_iter()
                .find(|movement| {
                    let future_translation = player.0.translation.xy() + *movement;

                    !is_intersect(
                        (future_translation, player.1.into()),
                        (intersected.1.translation.xy(), intersected.0.into()),
                    )
                });

            if let Some(resolved_movement) = resolved_movement {
                movement = resolved_movement;
            } else {
                movement = Vec2::ZERO;
            }
        }

        player.2.direction = direction;

        player.0.translation += movement.extend(0.0);
    }
}

fn draw_gizmos(mut gizmos: Gizmos, player: Single<(&Transform, &Size), With<Player>>) {
    gizmos.circle_2d(
        Isometry2d::from_translation(player.0.translation.xy()),
        5.0,
        Color::linear_rgb(1.0, 0.0, 1.0),
    );
    gizmos.rect_2d(
        Isometry2d::from_translation(player.0.translation.xy()),
        player.1.into(),
        Color::linear_rgb(1.0, 0.0, 1.0),
    );
}

fn update_player_sprite(
    mut player: Single<(&mut Sprite, &Player)>
) {
    player.0.flip_x = match player.1.direction{
        Direction::LEFT => true,
        _ => false
    }
}
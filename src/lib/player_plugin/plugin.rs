use std::ops::Deref;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    camera::Camera2d,
    color::Color,
    ecs::{
        component::Component,
        query::{With, Without},
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut, Single},
    },
    gizmos::gizmos::Gizmos,
    image::{
        ImageLoaderSettings, ImageSampler, TextureAtlas, TextureAtlasBuilder, TextureAtlasLayout,
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{Isometry2d, UVec2, Vec2, Vec3Swizzles},
    sprite::Sprite,
    time::{Time, Timer},
    transform::components::Transform,
};

use crate::core::{
    components::{AtlasRange, Blockable, Size},
    utils::is_intersect,
};

#[derive(Clone, Eq, PartialEq)]
enum Direction {
    TOP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
enum PlayerMovement {
    Idle,
    Walking,
}

struct Atlases {
    top: AtlasRange,
    down: AtlasRange,
    side: AtlasRange,
}

struct PlayerAtlases {
    idle: Atlases,
    walking: Atlases,
}

#[derive(Component)]
#[require(Size = Size::from(Vec2::new(48.0, 64.0)))]
struct Player {
    direction: Direction,
    movement: PlayerMovement,
    atlases: PlayerAtlases,
    animation_timer: Timer,
    current_atlas_range: AtlasRange
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

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let builder =
        asset_server
            .load_builder()
            .with_settings(|settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::nearest()
            });

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 6, None, None);

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_atlases = PlayerAtlases {
        idle: Atlases {
            top: AtlasRange {
                first: 10,
                last: 14,
            },
            down: AtlasRange {
                first: 25,
                last: 28,
            },
            side: AtlasRange { first: 0, last: 3 },
        },
        walking: Atlases {
            top: AtlasRange {
                first: 15,
                last: 18,
            },
            down: AtlasRange {
                first: 20,
                last: 23,
            },
            side: AtlasRange { first: 5, last: 8 },
        },
    };

    let mut sprite = Sprite::from_atlas_image(
        builder.load("./player.png"),
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
    );

    sprite.custom_size = Some(Vec2::splat(64.0));

    commands.spawn((
        sprite,
        Transform::default(),
        Player {
            direction: Direction::DOWN,
            movement: PlayerMovement::Idle,
            current_atlas_range: player_atlases.idle.down.clone(),
            atlases: player_atlases,
            animation_timer: Timer::from_seconds(1.0 / 6.0, bevy::time::TimerMode::Repeating),
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
    player: Single<(&mut Transform, &Size, &mut Player)>,
    blockable: Query<(&Size, &Transform), (With<Blockable>, Without<Player>)>,
) {    
    const UP_KEY: KeyCode = KeyCode::KeyW;
    const DOWN_KEY: KeyCode = KeyCode::KeyS;
    const LEFT_KEY: KeyCode = KeyCode::KeyA;
    const RIGHT_KEY: KeyCode = KeyCode::KeyD;
    
    let (mut transform, size, mut player) = player.into_inner();
    let mut movement = Vec2::ZERO;
    let mut new_direction: Option<Direction> = None;
   
    if keyboard_inputs.pressed(UP_KEY) {
        movement.y += 1.0;
        new_direction = Some(Direction::TOP);
    }

    if keyboard_inputs.pressed(DOWN_KEY) {
        movement.y -= 1.0;
        new_direction = Some(Direction::DOWN);
    }

    if keyboard_inputs.pressed(LEFT_KEY) {
        movement.x -= 1.0;
        new_direction = Some(Direction::LEFT);
    }

    if keyboard_inputs.pressed(RIGHT_KEY) {
        movement.x += 1.0;
        new_direction = Some(Direction::RIGHT);
    }

    player.movement = match movement != Vec2::ZERO {
        true => PlayerMovement::Walking,
        false => PlayerMovement::Idle,
    };

    if let Some(new_direction) = new_direction && new_direction != player.direction
    {
        let atlases = match player.movement {
            PlayerMovement::Idle => &player.atlases.idle,
            PlayerMovement::Walking => &player.atlases.walking,
        };

        let range = match new_direction {
            Direction::TOP => &atlases.top,
            Direction::DOWN => &atlases.down,
            Direction::LEFT => &atlases.side,
            Direction::RIGHT => &atlases.side,
        };

        player.current_atlas_range = range.clone();
        player.direction = new_direction;
        player.animation_timer.almost_finish();
    }

    if movement != Vec2::ZERO {
        let future_translation = transform.translation.xy() + movement;

        let intersected = blockable.iter().find(|(bsize, transform)| {
            is_intersect(
                (future_translation, size.into()),
                (transform.translation.xy(), (*bsize).into()),
            )
        });

        if let Some(intersected) = intersected {
            let resolved_movement = [movement.clone(), movement.with_x(0.0), movement.with_y(0.0)]
                .into_iter()
                .find(|movement| {
                    let future_translation = transform.translation.xy() + *movement;

                    !is_intersect(
                        (future_translation, size.into()),
                        (intersected.1.translation.xy(), intersected.0.into()),
                    )
                });

            if let Some(resolved_movement) = resolved_movement {
                movement = resolved_movement;
            } else {
                movement = Vec2::ZERO;
            }
        }

        transform.translation += movement.extend(0.0);
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

fn update_player_sprite(player: Single<(&mut Sprite, &mut Player)>, time: Res<Time>) {
    let (mut sprite, mut player) = player.into_inner();

    sprite.flip_x = match player.direction {
        Direction::LEFT => false,
        Direction::RIGHT => true,
        _ => sprite.flip_x,
    };

    player.animation_timer.tick(time.delta());


    if player.animation_timer.is_finished() {
        let range = &player.current_atlas_range;

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index >= range.last || atlas.index < range.first {
                range.first
            } else {
                atlas.index + 1
            };
        }
    }
}

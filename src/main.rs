use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE_WIDTH: f32 = 20.0;
pub const PLAYER_SIZE_HEIGHT: f32 = 70.0;
pub const PLAYER_SPEED: f32 = 600.0;

pub const NUMBER_OF_BALLS: i32 = 3;
pub const BALL_SPEED: f32 = 600.0;
pub const BALL_SIZE: f32 = 32.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_ball)
        .add_system(player_movement)
        .add_system(restrict_player_movement)
        .add_system(ball_movement)
        .add_system(update_ball_direction)
        .add_system(ball_player_collision)
        .run()
}

#[derive(Component)]
pub struct Player {
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec2,
}

pub fn spawn_ball (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ) {
    let window = window_query.get_single().unwrap();

    for _ball in 0..NUMBER_OF_BALLS {
        commands.spawn(
            (
                SpriteBundle {
                    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                    texture: asset_server.load("sprites/ball_red_small.png"),
                    ..default()
                },
                Ball {
                    direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                },
            )
        );
    }
}

pub fn spawn_player (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    ) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 9.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/laserBlueVertical.png"),
                ..default()
            },
            Player {},
        )
    );
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
        });
}

pub fn ball_movement(mut ball_query: Query<(&mut Transform, &Ball)>, time: Res<Time>){
    for (mut transform, ball) in ball_query.iter_mut() {
        let direction = Vec3::new(ball.direction.x, ball.direction.y, 0.0);
        transform.translation += direction * BALL_SPEED * time.delta_seconds();
    }
}

pub fn update_ball_direction(mut ball_query: Query<(&Transform, &mut Ball)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;
    let x_min = 0.0 + half_ball_size;
    let x_max = window.width() - half_ball_size;
    let y_min = 0.0 + half_ball_size;
    let y_max = window.height() - half_ball_size;

    for (transform, mut ball) in ball_query.iter_mut() {
        let translation = transform.translation;

        if translation.x < x_min || translation.x > x_max {
            ball.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            ball.direction.y *= -1.0;
        }
    }
}

pub fn ball_player_collision(mut player_query: Query<(Entity, &Transform), With<Player>>, mut ball_query: Query<(&Transform, &mut Ball)>, window_query: Query<&Window, With<PrimaryWindow>>) {
    for (transform, mut ball) in ball_query.iter_mut() {
        let translation = transform.translation;
        if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
            let distance = player_transform
                .translation
                .distance(translation);

            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            let player_height = PLAYER_SIZE_HEIGHT / 2.0;
            let player_width = PLAYER_SIZE_WIDTH / 2.0;
            let ball_radius = BALL_SIZE / 2.0;



            if player_x + player_width > translation.x + ball_radius || player_x - player_width + ball_radius < translation.x {
                if distance < player_width + ball_radius {
                    println!("Ball hit player! X");
                    ball.direction *= -1.0;
                }
            } else if player_y + player_height > translation.y + ball_radius || player_y - player_height + ball_radius < translation.y {
                if distance < player_height + ball_radius {
                    println!("Ball hit player! Y");
                    ball.direction *= -1.0;
                }
            }
        }
    }
}
/*
pub fn ball_player_collision(mut ball_query: Query<(&Transform, &mut Ball)>, player_query: Query<&Transform, With<Player>>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let player_transform = player_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;

    for (transform, mut ball) in ball_query.iter_mut() {
        let translation = transform.translation;

        if translation.x + half_ball_size > player_transform.translation.x - PLAYER_SIZE_WIDTH / 2.0 &&
            translation.x - half_ball_size < player_transform.translation.x + PLAYER_SIZE_WIDTH / 2.0 &&
            translation.y + half_ball_size > player_transform.translation.y - PLAYER_SIZE_HEIGHT / 2.0 &&
            translation.y - half_ball_size < player_transform.translation.y + PLAYER_SIZE_HEIGHT / 2.0 {
                ball.direction.x *= -1.0;
            } else if translation.y + half_ball_size < player_transform.translation.y + PLAYER_SIZE_HEIGHT &&
            translation.y - half_ball_size > player_transform.translation.y - PLAYER_SIZE_HEIGHT &&
            translation.x + half_ball_size > player_transform.translation.x - PLAYER_SIZE_WIDTH / 2.0 &&
            translation.x - half_ball_size < player_transform.translation.x + PLAYER_SIZE_WIDTH / 2.0 {
                ball.direction.y *= -1.0;
            }

        // fix ball stuck inside player
        else if translation.y < player_transform.translation.y + PLAYER_SIZE_HEIGHT / 2.0 &&
            translation.y > player_transform.translation.y - PLAYER_SIZE_HEIGHT / 2.0 &&
            translation.x > player_transform.translation.x - PLAYER_SIZE_WIDTH / 2.0 &&
            translation.x < player_transform.translation.x + PLAYER_SIZE_WIDTH / 2.0 {
            ball.direction.y = player_transform.translation.y + PLAYER_SIZE_HEIGHT / 2.0 + 1.0;
            ball.direction.x = player_transform.translation.x + PLAYER_SIZE_WIDTH / 2.0 + 1.0;
        }
    }
} */

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut(){
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn restrict_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size_width = PLAYER_SIZE_WIDTH / 2.0;
        let half_player_size_height = PLAYER_SIZE_HEIGHT / 2.0;

        let x_min = 0.0 + half_player_size_width;
        let x_max = window.width() - half_player_size_width;
        let y_min = 0.0 + half_player_size_height;
        let y_max = window.height() - half_player_size_height;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
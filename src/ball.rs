use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Res, ResMut, Single},
    },
    math::{Vec2, primitives::Circle},
    render::{
        mesh::{Mesh, Mesh2d},
        view::window,
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::{enemy::Enemy, game_logic::WindowBounds, player::Player};

#[derive(Component)]
struct Ball;

#[derive(Component)]
enum Direction {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, (move_ball, hit_wall, hit_player, hit_enemy));
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_loader: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle { radius: 10.0 })),
        MeshMaterial2d(materials.add(Color::srgb(255.0, 255.0, 255.0))),
        Transform::from_xyz(0., 0., 2.),
        Direction::UpLeft,
    ));
}

fn move_ball(
    time: Res<Time>,
    window_bounds: Res<WindowBounds>,
    mut ball: Single<(&mut Transform, &Direction), With<Ball>>,
) {
    match ball.1 {
        Direction::UpRight => {
            if ball.0.translation.y <= window_bounds.y / 2. - 10.
                && ball.0.translation.x <= window_bounds.x / 2. - 10.
            {
                ball.0.translation.x += 300.0 * time.delta_secs();
                ball.0.translation.y += 300.0 * time.delta_secs();
            }
        }
        Direction::UpLeft => {
            if ball.0.translation.y <= window_bounds.y / 2. - 10.
                && ball.0.translation.x >= -window_bounds.x / 2. + 10.
            {
                ball.0.translation.x -= 300.0 * time.delta_secs();
                ball.0.translation.y += 300.0 * time.delta_secs();
            }
        }
        Direction::DownRight => {
            if ball.0.translation.y >= -window_bounds.y / 2. + 10.
                && ball.0.translation.x <= window_bounds.x / 2. - 10.
            {
                ball.0.translation.x += 300.0 * time.delta_secs();
                ball.0.translation.y -= 300.0 * time.delta_secs();
            }
        }
        Direction::DownLeft => {
            if ball.0.translation.y >= -window_bounds.y / 2. + 10.
                && ball.0.translation.x >= -window_bounds.x / 2. + 10.
            {
                ball.0.translation.x -= 300.0 * time.delta_secs();
                ball.0.translation.y -= 300.0 * time.delta_secs();
            }
        }
    }
}

fn hit_player(
    mut ball: Single<(&mut Transform, &mut Direction), With<Ball>>,
    player: Single<&Transform, (With<Player>, Without<Ball>, Without<Enemy>)>,
) {
    if ball.0.translation.distance(player.translation) <= 35. {
        match *ball.1 {
            Direction::UpRight => *ball.1 = Direction::UpLeft,
            Direction::UpLeft => *ball.1 = Direction::UpRight,
            Direction::DownRight => *ball.1 = Direction::DownLeft,
            Direction::DownLeft => *ball.1 = Direction::DownRight,
        }
    }
}

fn hit_enemy(
    mut ball: Single<(&mut Transform, &mut Direction), With<Ball>>,
    enemy: Single<&Transform, (With<Enemy>, Without<Player>, Without<Ball>)>,
) {
    if ball.0.translation.distance(enemy.translation) <= 35. {
        match *ball.1 {
            Direction::UpRight => *ball.1 = Direction::UpLeft,
            Direction::UpLeft => *ball.1 = Direction::UpRight,
            Direction::DownRight => *ball.1 = Direction::DownLeft,
            Direction::DownLeft => *ball.1 = Direction::DownRight,
        }
    }
}

fn hit_wall(
    window_bounds: Res<WindowBounds>,
    mut ball: Single<(&mut Transform, &mut Direction), With<Ball>>,
) {
    if ball.0.translation.y >= window_bounds.y / 2. - 10. {
        match *ball.1 {
            Direction::UpRight => *ball.1 = Direction::DownRight,
            Direction::UpLeft => *ball.1 = Direction::DownLeft,
            _ => (),
        }
    } else if ball.0.translation.y <= -window_bounds.y / 2. + 10. {
        match *ball.1 {
            Direction::DownRight => *ball.1 = Direction::UpRight,
            Direction::DownLeft => *ball.1 = Direction::UpLeft,
            _ => (),
        }
    } else if ball.0.translation.x <= -window_bounds.x / 2. + 10. {
        match *ball.1 {
            Direction::UpLeft => *ball.1 = Direction::UpRight,
            Direction::DownLeft => *ball.1 = Direction::DownRight,
            _ => (),
        }
    } else if ball.0.translation.x >= window_bounds.x / 2. - 10. {
        match *ball.1 {
            Direction::UpRight => *ball.1 = Direction::UpLeft,
            Direction::DownRight => *ball.1 = Direction::DownLeft,
            _ => (),
        }
    }
}

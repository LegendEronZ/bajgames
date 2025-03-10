use bevy::{
    app::{Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Res, ResMut, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::primitives::Rectangle,
    render::mesh::{Mesh, Mesh2d},
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Time,
    transform::components::Transform,
};

use crate::{game_logic::WindowBounds, player::Player};

#[derive(Component)]
pub struct Enemy {
    movement_speed: f32,
    pub points: i32,
}

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(Update, move_enemy);
    }
}

fn move_enemy(
    time: Res<Time>,
    mut enemy_transform: Single<&mut Transform, (With<Enemy>, Without<Player>)>,
    player_transform: Single<&Transform, (With<Player>, Without<Enemy>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    enemy_transform.translation.y = -player_transform.translation.y;
}

fn spawn_enemy(
    mut commands: Commands,
    window_bounds: Res<WindowBounds>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_loader: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_loader.load("textures/xqcL.png");

    commands.spawn((
        Enemy {
            movement_speed: 300.0,
            points: 0,
        },
        Mesh2d(meshes.add(Rectangle::new(20.0, 100.0))),
        MeshMaterial2d(materials.add(texture)),
        Transform::from_xyz(window_bounds.x / 2. - 60., 0., 2.),
        Direction::Up,
    ));
}

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

use crate::{enemy::Enemy, game_logic::WindowBounds};

#[derive(Component)]
pub struct Player {
    movement_speed: f32,
    pub points: i32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn move_player(
    time: Res<Time>,
    query: Single<(&Player, &mut Transform)>,
    window_bounds: Res<WindowBounds>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (player, mut transform) = query.into_inner();

    if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
        if transform.translation.y <= window_bounds.y / 2. - 50. {
            transform.translation.y += player.movement_speed * time.delta_secs();
        }
    }

    if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
        if transform.translation.y >= -window_bounds.y / 2. + 50. {
            transform.translation.y -= player.movement_speed * time.delta_secs();
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    window_bounds: Res<WindowBounds>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_loader: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture = asset_loader.load("textures/forsenE.png");

    commands.spawn((
        Player {
            movement_speed: 300.0,
            points: 0,
        },
        Mesh2d(meshes.add(Rectangle::new(20.0, 100.0))),
        MeshMaterial2d(materials.add(texture)),
        Transform::from_xyz(-window_bounds.x / 2. + 60., 0., 2.),
    ));
}

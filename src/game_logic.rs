use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        component::Component,
        event::{Event, EventReader},
        query::{With, Without},
        system::{Commands, Res, Resource, Single},
    },
    text::{FontSmoothing, Text2d, TextFont},
    transform::components::Transform,
};

use crate::{
    enemy::Enemy,
    player::{self, Player},
};

#[derive(Component)]
pub struct EnemyPointsText;

#[derive(Component)]
pub struct PlayerPointsText;

#[derive(Event)]
pub struct EnemyScoredEvent;

#[derive(Event)]
pub struct PlayerScoredEvent;

#[derive(Resource)]
pub struct WindowBounds {
    pub x: f32,
    pub y: f32,
}

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup_scores)
            .add_systems(Update, update_scores);
    }
}

fn update_scores(
    mut player_scored: EventReader<PlayerScoredEvent>,
    mut enemy_scored: EventReader<EnemyScoredEvent>,
    enemy: Single<&Enemy>,
    player: Single<&Player>,
    mut enemy_points_text: Single<&mut Text2d, (With<EnemyPointsText>, Without<PlayerPointsText>)>,
    mut player_points_text: Single<&mut Text2d, (With<PlayerPointsText>, Without<EnemyPointsText>)>,
) {
    if !player_scored.is_empty() {
        player_points_text.0 = format!("Player: {}", player.points);
        player_scored.clear();
    }

    if !enemy_scored.is_empty() {
        enemy_points_text.0 = format!("Enemy: {}", enemy.points);
        enemy_scored.clear();
    }
}

fn setup_scores(
    mut commands: Commands,
    window_bounds: Res<WindowBounds>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Roboto-Medium.ttf");

    let text_font = TextFont {
        font,
        font_size: 30.0,
        font_smoothing: FontSmoothing::AntiAliased,
    };

    commands.spawn((
        Text2d::new(format!("Player: 0")),
        text_font.clone(),
        Transform::from_xyz(-window_bounds.x / 2. + 100., window_bounds.y / 2. - 20., 2.),
        PlayerPointsText,
    ));

    commands.spawn((
        Text2d::new(format!("Enemy: 0")),
        text_font.clone(),
        Transform::from_xyz(window_bounds.x / 2. - 100., window_bounds.y / 2. - 20., 2.),
        EnemyPointsText,
    ));
}

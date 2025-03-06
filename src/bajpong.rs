use bevy::app::Plugin;

use crate::{
    ball::BallPlugin,
    enemy::EnemyPlugin,
    game_logic::{EnemyScoredEvent, GameLogicPlugin, PlayerScoredEvent, WindowBounds},
    player::PlayerPlugin,
    setup::SetupPlugin,
};

pub struct BajpongPlugin;

impl Plugin for BajpongPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(WindowBounds { x: 1280., y: 720. })
            .add_event::<PlayerScoredEvent>()
            .add_event::<EnemyScoredEvent>()
            .add_plugins(SetupPlugin {
                title: "Baj Pong".to_string(),
            })
            .add_plugins(BallPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(GameLogicPlugin);
    }
}

impl BajpongPlugin {}

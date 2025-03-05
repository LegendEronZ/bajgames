mod bajpong;
mod ball;
mod enemy;
mod game_logic;
mod player;
mod setup;

use bajpong::BajpongPlugin;
use bevy::{
    DefaultPlugins,
    app::App,
    asset::{AssetMetaCheck, AssetPlugin},
    prelude::PluginGroup,
    utils::default,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        }))
        .add_plugins(BajpongPlugin)
        .run();
}

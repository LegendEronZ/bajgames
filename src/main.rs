mod setup;

use bevy::{DefaultPlugins, app::App};
use setup::ScenePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin {
            title: "Baj Pong".to_string(),
        })
        .run();
}

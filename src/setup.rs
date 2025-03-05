use bevy::{
    app::{App, Plugin, Startup},
    asset::{AssetServer, Assets},
    color::Color,
    core_pipeline::{bloom::Bloom, core_2d::Camera2d},
    ecs::system::{Commands, Res, ResMut, Resource},
    math::primitives::Rectangle,
    render::{
        camera::Camera,
        mesh::{Mesh, Mesh2d},
    },
    sprite::{ColorMaterial, MeshMaterial2d},
    text::{FontSmoothing, Text2d, TextFont},
    transform::components::Transform,
    utils::default,
};

use crate::game_logic::WindowBounds;

#[derive(Resource)]
struct GameTitle(String);

pub struct SetupPlugin {
    pub title: String,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTitle(self.title.clone()))
            .add_systems(Startup, (Self::setup_camera, Self::setup_scene));
    }
}

impl SetupPlugin {
    fn setup_scene(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        window_bounds: Res<WindowBounds>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        game_title: Res<GameTitle>,
    ) {
        let font = asset_server.load("fonts/Roboto-Medium.ttf");

        let text_font = TextFont {
            font,
            font_size: 30.0,
            font_smoothing: FontSmoothing::AntiAliased,
        };

        commands.spawn((
            Text2d::new(game_title.0.clone()),
            text_font.clone(),
            Transform::from_xyz(0., window_bounds.y / 2.0 - 20.0, 0.),
        ));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(window_bounds.x, window_bounds.y))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
        ));
    }

    fn setup_camera(mut commands: Commands) {
        commands.spawn((Camera2d, Camera { ..default() }, Bloom::NATURAL));
    }
}

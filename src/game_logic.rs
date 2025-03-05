use bevy::ecs::{component::Component, system::Resource};

#[derive(Component)]
pub struct Points(i32);

#[derive(Resource)]
pub struct WindowBounds {
    pub x: f32,
    pub y: f32,
}

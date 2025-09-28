use bevy::window::PresentMode;
use bevy::prelude::*;
use rand::Rng;
use renderer::RendererPlugin;
use crate::input_map::*;
use crate::server::GameMapData;


mod renderer;
mod input_map;
mod server;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(InputMap::default())
        .insert_resource(GameMapData::new(999,1,999))
        .add_plugins(RendererPlugin)
        .run();
}
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
        .add_systems(Startup, spawn_debug_world)
        .run();
}

fn spawn_debug_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,){
        commands.spawn(PointLight {
        intensity: 1000.0,
        ..Default::default()
    });
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    //spawn in ground layer; 50x50 meter
    for i in 0..100 {
        for j in 0..=99 {
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::LinearRgba(LinearRgba {
                        red: random_color(),
                        green: random_color(),
                        blue: random_color(),
                        alpha: 1.0,
                    }),
                    ..Default::default()
                })),
                Transform::from_xyz(i as f32, -1.0, j as f32),
            ));
        }
    }
}
fn random_color() -> f32 {
    let mut rng = rand::rng();
    let value: f32 = rng.random();
    return value;
}
use bevy::prelude::*;
use crate::{input_map::*, server::{GameCoordinate, GameMapData, TileInfo, TileTag}};
pub struct RendererPlugin;
impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (build_base_tiles,spawn_camera, generate_base_render_system).chain());
        app.add_systems(
            Update,
            (
                update_render_area,
                update_keyboard_event,
            ),
        );
    }
}
const RENDER_AREA: usize = 100;
#[derive(Component)]
pub struct CameraTag;

#[derive(Component)]
pub struct LastRenderCoord{
    position: Vec3,
}
impl LastRenderCoord{
    fn new(x: f32, y: f32, z: f32) -> Self{
        Self { position: Vec3::new(x, y, z) }
    }
}

fn spawn_camera(
    mut commands: Commands,
    map: Res<GameMapData>){
    commands.spawn((
        CameraTag,
        Camera3d::default(),
        Transform::from_xyz((map.total_x_from_zero/2) as f32, 100.0, (map.total_z_from_zero/2) as f32).looking_to(Vec3::NEG_Y, Vec3::Y),//center camera in map looking down
        LastRenderCoord::new((map.total_x_from_zero/2) as f32, 100.0, (map.total_z_from_zero/2) as f32),
    ));
}

fn build_base_tiles(commands: Commands, map: Res<GameMapData>,){
        map.populate_tile_entities(commands);//populate whole map with empty tiles
    }

fn generate_base_render(
    mut commands: Commands,
    map: Res<GameMapData>,
    cam_pos: Vec3,
    tiles: Query<(&GameCoordinate, &TileInfo), With<TileTag>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    
    let cam_x = cam_pos.x as usize;
    let cam_z = cam_pos.z as usize;

    let max_x = map.total_x_from_zero;
    let max_z = map.total_z_from_zero;

    for (coord, tile) in tiles.iter() {
        let dx = wrapped_distance(coord.x, cam_x, max_x);
        let dz = wrapped_distance(coord.z, cam_z, max_z);

        if dx <= RENDER_AREA && dz <= RENDER_AREA {
            if *tile == TileInfo::Empty{
                commands.spawn((
                    Transform::from_translation(Vec3::new(coord.x as f32,coord.y as f32,coord.z as f32,)),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::LinearRgba(LinearRgba {
                        red:  0.0,
                        green:((coord.x % 50) as f32) / 50.0,
                        blue: ((coord.z % 50) as f32) / 50.0,
                        alpha: 1.0,
                    }),
                    ..Default::default()
                })),
                ));
            }
            if *tile == TileInfo::Occupied{
                commands.spawn((
                    Transform::from_translation(Vec3::new(coord.x as f32,coord.y as f32,coord.z as f32,)),
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Color::LinearRgba(LinearRgba {
                            red: 1.0,
                            green: 0.0,
                            blue: 0.0,
                            alpha: 1.0,
                        }),
                        ..Default::default()
                    })),
                ));
            }
        }
    }
}

fn wrapped_distance(a: usize, b: usize, max_index: usize) -> usize {
    let size = max_index + 1; // total number of tiles on this axis
    let diff = if a > b { a - b } else { b - a };
    diff.min(size - diff)
}


fn update_keyboard_event(
    mut camera: Query<&mut Transform, With<CameraTag>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<InputMap>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    if input.pressed(settings.key_forward) {
        direction += -Vec3::X;
    }
    if input.pressed(settings.key_backward) {
        direction += Vec3::X;
    }
    if input.pressed(settings.key_left) {
        direction += Vec3::Z;
    }
    if input.pressed(settings.key_right) {
        direction += -Vec3::Z;
    }
    let move_delta = direction.normalize_or_zero() * 10.0 *  time.delta_secs();
    transform.translation += move_delta;
}

fn update_render_area(
    commands: Commands,
    map: Res<GameMapData>,
    mut camera: Query<(&Transform,&mut LastRenderCoord), With<CameraTag>>,
    tiles: Query<(&GameCoordinate, &TileInfo), With<TileTag>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
){
    let Ok((cam, mut last)) = camera.single_mut() else { return };
    let cam_pos = cam.translation;

    let dx = (cam_pos.x - last.position.x).abs();
    let dz = (cam_pos.z - last.position.z).abs();

    if dx > (RENDER_AREA as f32 / 2.0) || dz > (RENDER_AREA as f32 / 2.0){
        generate_base_render(commands,map,cam.translation,tiles,meshes,materials);
        last.position = cam.translation;
    }
}

fn generate_base_render_system(
    commands: Commands,
    map: Res<GameMapData>,
    mut camera: Query<(&Transform,&mut LastRenderCoord), With<CameraTag>>,
    tiles: Query<(&GameCoordinate, &TileInfo), With<TileTag>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok((cam,mut last)) = camera.single_mut() else { return };
    generate_base_render(
        commands,
        map,
        cam.translation,
        tiles,
        meshes,
        materials,
    );
    last.position = cam.translation;
}
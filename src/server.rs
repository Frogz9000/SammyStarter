
use bevy::{prelude::*};
#[derive(Resource)]
pub struct GameMapData{
    pub total_x_from_zero: usize,
    pub total_y_from_zero: usize,
    pub total_z_from_zero: usize
}
impl GameMapData{
    pub fn new(x: usize, y: usize, z: usize) -> Self{
        Self {total_x_from_zero: x, total_y_from_zero:y, total_z_from_zero:z }
    }
    pub fn populate_tile_entities(&self, mut commands: Commands){
        for x_id in 0..=self.total_x_from_zero{
            for y_id in 0..=self.total_y_from_zero{
                for z_id in 0..=self.total_z_from_zero{
                    commands.spawn((
                        TileTag,
                        GameCoordinate::new(x_id, y_id, z_id),
                        TileInfo::default(),
                    ));
                }
            }
        }
    }
}

#[derive(Component)]
pub struct TileTag;

#[derive(Component)]
pub struct GameCoordinate{
    pub x: usize,
    pub y: usize,
    pub z: usize
}
impl GameCoordinate{
    pub fn new(x: usize, y: usize, z: usize) -> Self{
        Self { x:x, y:y, z:z }
    }
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileInfo{
    Empty,
    Occupied // add more tile states/objects as needed
}
impl Default for TileInfo{
    fn default() -> Self {
        Self::Empty
    }
}

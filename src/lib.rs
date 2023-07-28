mod components;
mod plugin;
mod systems;
mod traits;

pub use components::*;
pub use plugin::*;
pub use systems::*;
pub use traits::*;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender};
pub use pathing::*;

#[derive(Resource, Deref)]
pub struct PFStreamInput(Sender<(Entity, Vec2, Vec2)>);

#[derive(Resource, Deref)]
pub struct PFStreamOutput(Receiver<(Entity, Vec<Vec2>)>);

#[derive(Resource, Deref)]
pub struct PFStreamReset<PG: Resource + Clone + PathingGridMap, PP: Resource + PathingGridSpace>(Sender<(PG, PP)>);


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Resource)]
pub struct SGrid(pub DS2Map);

impl PathingGridMap for SGrid {
    fn path_find(&self, start: GridPos, end: GridPos) -> Option<Vec<GridNode>> {
        self.0.find_path(start.into(), end.into())
    }
    fn even(&self) -> (usize, usize) {
        (1, 1)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Resource)]
pub struct GridSpace {
    // pub even_offset: Vec2,
    pub offset: Vec2,
    pub scale: Vec2,
}

impl GridSpace {
    pub fn new(
        // width: usize, length: usize
    ) -> Self {
        Self {
            // even_offset: Vec2::new(
            //     if width % 2 == 0 { 0.5 } else { 0.0 },
            //     if length % 2 == 0 { 0.5 } else { 0.0 },
            // ),
            ..Default::default()
        }
    }

    pub fn position_to_index(&self, position: Vec2) -> (isize, isize) {
        (
            (((position.x - self.offset.x) / self.scale.x).round()) as isize,
            (((position.y - self.offset.y) / self.scale.y).round()) as isize,
        )
    }

    pub fn index_to_position(&self, index: (isize, isize)) -> Vec2 {
        Vec2::new(
            index
                .0 as f32
                // .clamp(self.width as isize / -2, (self.width as isize - 1) / 2) as f32
                * self.scale.x
                + self.offset.x,
                // + self.even_offset.x,
            index
                .1 as f32
                // .clamp(self.length as isize / -2, (self.length as isize - 1) / 2) as f32
                * self.scale.y
                + self.offset.y
                // + self.even_offset.y,
        )
    }
}

impl Default for GridSpace {
    fn default() -> Self {
        Self {
            // width: 0,
            // length: 0,
            offset: Vec2::default(),
            // even_offset: Vec2::default(),
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

impl PathingGridSpace for GridSpace {
    fn grid_space(&self) -> &GridSpace {
        self
    }
    fn grid_space_mut(&mut self) -> &mut GridSpace {
        self
    }
}
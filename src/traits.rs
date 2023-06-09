use crate::GridSpace;
use pathing::{GridCell, GridNode};

pub trait PathingGridMap {
    fn path_find(&self, start: GridCell, end: GridCell) -> Option<Vec<GridNode>>;
    fn size(&self) -> (usize, usize) {
        (usize::MAX, usize::MAX)
    }
    fn even(&self) -> (usize, usize);
}

pub trait PathingGridSpace {
    fn grid_space(&self) -> &GridSpace;
    fn grid_space_mut(&mut self) -> &mut GridSpace;
}

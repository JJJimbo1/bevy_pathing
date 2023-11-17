use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PathFinder {
    #[default]
    Idle,
    Queued(Vec2, Vec2),
    Ready(Vec<Vec2>),
    ReQueue(Vec<Vec2>, Vec2, Vec2),
}

impl PathFinder {
    pub fn trip(&self) -> Option<(Vec2, Vec2)> {
        match self {
            PathFinder::Queued(start, end) | PathFinder::ReQueue(_, start, end) => { Some((*start, *end)) },
            _ => { None }
        }
    }

    pub fn set_trip(&mut self, (start, end): (Vec2, Vec2)) {
        match self {
            PathFinder::Idle => { *self = PathFinder::Queued(start, end); },
            PathFinder::Queued(_, _) => { *self = PathFinder::Queued(start, end); },
            PathFinder::Ready(path) => { *self = PathFinder::ReQueue(path.clone(), start, end); },
            PathFinder::ReQueue(path, _, _) => { *self = PathFinder::ReQueue(path.clone(), start, end); },
        }
    }

    pub fn path(&self) -> Option<Vec<Vec2>> {
        match self {
            PathFinder::Ready(path) | PathFinder::ReQueue(path, _, _) => { Some(path.clone()) },
            _ => { None }
        }
    }

    pub fn path_mut(&mut self) -> Option<&mut Vec<Vec2>> {
        match self {
            PathFinder::Ready(path) | PathFinder::ReQueue(path, _, _) => { Some(path) },
            _ => { None }
        }
    }
}
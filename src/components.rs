use bevy::prelude::*;

#[derive(Debug, Default, Clone, Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn path(&self) -> Option<Vec<Vec2>> {
        match self {
            PathFinder::Ready(path) | PathFinder::ReQueue(path, _, _) => { Some(path.clone()) },
            _ => { None }
        }
    }
}
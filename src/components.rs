use bevy::prelude::*;

#[derive(Debug, Default, Component)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathFinder {
    #[default]
    Idle,
    Queued(Vec2, Vec2),
    Ready(Vec<Vec2>),
}

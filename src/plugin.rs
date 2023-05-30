use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum PathFindingSystems {
    GridSpaceUpdateSystem,
    PathFindingSystem,
}

#[derive(Debug, Clone, Copy)]
pub struct PathFindingPlugin<
    PG: Resource + PathingGridMap = OGrid,
    PP: Resource + PathingGridSpace = GridSpace,
> {
    pg: PhantomData<PG>,
    pp: PhantomData<PP>,
}

impl<PG: Resource + PathingGridMap, PP: Resource + PathingGridSpace> Default
    for PathFindingPlugin<PG, PP>
{
    fn default() -> Self {
        Self {
            pg: PhantomData,
            pp: PhantomData,
        }
    }
}

impl<PG: Resource + Clone + PathingGridMap, PP: Resource + PathingGridSpace> Plugin
    for PathFindingPlugin<PG, PP>
{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_pathfinder::<PG, PP>)
            .add_system(
                grid_space_update_system::<PG, PP>
                    .in_set(PathFindingSystems::GridSpaceUpdateSystem)
                    .run_if(resources_exist::<PG, PP /*PS*/>),
            )
            .add_system(
                path_finding_system
                    .in_set(PathFindingSystems::PathFindingSystem)
                    .after(PathFindingSystems::GridSpaceUpdateSystem)
                    .run_if(resources_exist::<PG, PP /*PS*/>),
            );
    }
}

fn resources_exist<PG: Resource + PathingGridMap, PP: Resource + PathingGridSpace>(
    grid_map: Option<Res<PG>>,
    grid_space: Option<Res<PG>>,
) -> bool {
    grid_map.is_some() && grid_space.is_some()
}

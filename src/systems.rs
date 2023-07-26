use crate::{PFStreamInput, PFStreamOutput};
use bevy::prelude::*;
use crossbeam_channel::unbounded;
use pathing::GridPos;

use crate::{PathFinder, PathingGridMap, PathingGridSpace};

//TODO: Allow the grid to change.
//TODO: Prevent encapsulation.
pub fn setup_pathfinder<PG: Resource + Clone + PathingGridMap, PP: Resource + PathingGridSpace>(
    map: Res<PG>,
    space: Res<PP>,
    mut commands: Commands,
) {
    let (input, reader) = unbounded::<(Entity, Vec2, Vec2)>();
    let (sender, output) = unbounded::<(Entity, Vec<Vec2>)>();
    let map = (*map).clone();
    let space = space.grid_space().clone();
    std::thread::spawn(move || loop {
        for (entity, start, end) in reader.iter() {
            let start_index = space.position_to_index(start);
            let end_index = space.position_to_index(end);
            if start_index == end_index {
                let _ = sender.try_send((entity, Vec::new()));
            }
            let start_cell = GridPos::from(start_index);
            let end_cell = GridPos::from(end_index);
            let path = map
                .path_find(start_cell, end_cell)
                .map(|mut nodes| {
                    nodes.remove(0);
                    nodes
                        .iter()
                        .map(|n| space.index_to_position((n.x, n.z)))
                        .collect()
                })
                .unwrap_or(Vec::default());
            let _ = sender.try_send((entity, path));
        }
    });
    commands.insert_resource(PFStreamInput(input));
    commands.insert_resource(PFStreamOutput(output));
}

pub fn grid_space_update_system<PG: Resource + PathingGridMap, PP: Resource + PathingGridSpace>(
    grid: ResMut<PG>,
    mut space: ResMut<PP>,
) {
    if grid.is_changed() {
        let (x, z) = grid.even();
        space.grid_space_mut().width = x;
        space.grid_space_mut().length = z;
        let x_offset = (x as f32 / 2.0 - 0.5).abs();
        let z_offset = (z as f32 / 2.0 - 0.5).abs();
        space.grid_space_mut().even_offset = Vec2::new(x_offset, z_offset);
    }
}

pub fn path_finding_system(
    input: Res<PFStreamInput>,
    output: Res<PFStreamOutput>,
    mut path_finders: ParamSet<(
        Query<(Entity, &PathFinder), Changed<PathFinder>>,
        Query<&mut PathFinder>,
    )>,
) {
    path_finders.p0().for_each(|(entity, path_finder)| {
        if let PathFinder::Queued(start, end) = *path_finder {
            let _ = input.try_send((entity, start, end));
        }
    });

    //TODO: When Bevy impls IterMut for mutables, use this instead.
    // let (entities, paths): (Vec<Entity>, Vec<Vec<Vec2>>) = output.try_iter().unzip();
    // path_finders.iter_many_mut(entities).zip(paths).for_each(|((entity, pf), path)| {
    // });

    output.try_iter().for_each(|(entity, path)| {
        let mut p1 = path_finders.p1();
        let Ok(mut path_finder) = p1.get_mut(entity) else { return; };
        *path_finder = PathFinder::Ready(path);
    })
}

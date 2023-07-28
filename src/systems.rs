use crate::{PFStreamInput, PFStreamOutput, PFStreamReset};
use bevy::prelude::*;
use crossbeam_channel::{unbounded, bounded};

use crate::{PathFinder, PathingGridMap, PathingGridSpace};

pub fn setup<PG: Resource + Clone + PathingGridMap, PP: Resource + PathingGridSpace>(
    map: Res<PG>,
    space: Res<PP>,
    mut commands: Commands,
) {
    let (input, reader) = unbounded::<(Entity, Vec2, Vec2)>();
    let (sender, output) = unbounded::<(Entity, Vec<Vec2>)>();
    let (reset, resets) = bounded::<(PG, PP)>(1);
    let mut map = (*map).clone();
    let mut space = space.grid_space().clone();
    std::thread::spawn(move || {
        // let mut map = map;
        // let mut space = space;
        loop {
            for (pg, pp) in resets.iter() {
                println!("Changing grid");
                map = pg;
                space = *pp.grid_space();
            }
            for (entity, start, end) in reader.iter() {
                let start_index = space.position_to_index(start);
                let end_index = space.position_to_index(end);
                if start_index == end_index {
                    println!("Start and End are the same, not pathing; Start: {:?}, End: {:?}", start_index, end_index);
                    let _ = sender.try_send((entity, Vec::new()));
                }
                let path = map
                    .path_find(start_index, end_index)
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
        }
    });
    commands.insert_resource(PFStreamInput(input));
    commands.insert_resource(PFStreamOutput(output));
    commands.insert_resource(PFStreamReset(reset));
}

pub fn grid_update<
    PG: Clone + Resource + PathingGridMap,
    PP: Clone + Resource + PathingGridSpace
> (
    grid: Res<PG>,
    space: Res<PP>,
    reset: Res<PFStreamReset<PG, PP>>,
) {
    if grid.is_changed() || space.is_changed() {
        let _ = reset.try_send((grid.clone(), space.clone()));
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

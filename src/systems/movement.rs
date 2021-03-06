use crate::prelude::*;

///Legion provides a shorthand for systems that only run a single query. Declaring
/// a system as `system(for_each)` derives the query from the system parameters --
/// and runs the system once for every matching entity. This is the same as declaring
/// a query that reads Entity and WantsToMove and iterating it as you have with other systems.
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // its safer & more efficient to use commands rather than modifying the component directly
        // adding a component that already exists replaces the old one.
        commands.add_component(want_move.entity, want_move.destination);

        // can access the details of another component with the `entry_ref` method.
        // Entities are only available if you've specified the components that they
        // use in your `read_component` or `write_component` declarations for the system
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    // its important to remove messages once they are processed. if you don't, they will
    // be processed again on the next turn.
    commands.remove(*entity);
}

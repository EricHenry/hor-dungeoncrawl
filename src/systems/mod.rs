mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        // when a system executes commands (i.e collision detection system). A hidden `flush`
        // at the end of the system tells Legion to apply the changes immediately
        // flushing after collision detection ensures that any deleted entities are gone before
        // they are rendered. It also guarantees that all systems up to that point have finished
        // executing before the next one runs. This is handy way to tame multi-threading issues
        // its a good idea to `flush` your systems after you make changes to the world - or at
        // least before you rely on those changes.
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;

use crate::prelude::*;

/// Build a scheduler used when we are in the `AwaitingInput` State
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
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
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

/// Build a scheduler used when we are in the `PlayerTurn` State
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        // flush to immediately apply the changes made in its command buffer
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

/// Build a scheduler used when we are in the `MonsterTurn` State
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        // .add_system(random_move::random_move_system())
        .add_system(chasing::chasing_system())
        // flush is called when a system makes changes to the ECS dataset.
        .flush()
        .add_system(combat::combat_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

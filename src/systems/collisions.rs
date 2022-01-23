use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // CommandBuffer is a special container legion can give you to insert instructions
    // for legion to perform after the system is finished. We will use the command buffer
    // to remove entities from the game.
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        // only care about the enemy entities that match the players position
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            // commands have the ability to create and delete entities from within systems
            commands.remove(*entity); // remove enemies that match the player's position
        })
}

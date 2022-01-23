use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // a Player entity is composed of the Player, Point, and Render components
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

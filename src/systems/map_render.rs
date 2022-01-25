use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0); // target a specific console
    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                println!("pt in map bounds, map idx = {}", idx);
                let glyph = match map.tiles[idx] {
                    TileType::Wall => to_cp437('.'),
                    TileType::Floor => to_cp437('#'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    // submitting the batch adds it to the global command list. it accepts a single integer
    // parameter, serving as a sort order. Zero renders first, ensuring that your map is
    // drawn at the beginning of the render cycle.
    draw_batch.submit(0).expect("Map Batch error");
}

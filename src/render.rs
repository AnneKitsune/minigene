use crate::*;
use uuidmap::Table;

/// Renders ascii characters.
///
/// # Panics
/// This function cannot panic under normal circumstances.
pub fn render_ascii(
    ctx: &mut Terminal,
    camera: &Camera,
    positions: &Table<Point>,
    multi_sprites: &Table<MultiSprite>,
    sprites: &Table<Sprite>,
) {
    for multi in multi_sprites.values() {
        // Look up the position using the point_id in the MultiSprite component
        if let Some(pos) = positions.get(multi.point_id) {
            for y in 0..multi.height as i32 {
                for x in 0..multi.width as i32 {
                    let idx = (x + y * multi.width as i32) as usize;
                    let screen_x = pos.x + x - camera.position.x + camera.screen_position.x;
                    let screen_y = pos.y + y - camera.position.y + camera.screen_position.y;
                    ctx.print_color(
                        screen_x,
                        screen_y,
                        multi.fg[idx],
                        multi.bg[idx],
                        multi
                            .ascii
                            .chars()
                            .nth(idx)
                            .expect("Index out of bounds for MultiSprite ascii"),
                    );
                }
            }
        }
    }
    for sprite in sprites.values() {
        // Look up the position using the point_id in the Sprite component
        if let Some(pos) = positions.get(sprite.point_id) {
            if position_inside_rect(
                pos.x - camera.position.x,
                pos.y - camera.position.y,
                0,
                0,
                camera.size.x as u32,
                camera.size.y as u32,
            ) {
                ctx.print_color(
                    pos.x - camera.position.x + camera.screen_position.x,
                    pos.y - camera.position.y + camera.screen_position.y,
                    sprite.fg,
                    sprite.bg,
                    sprite.glyph,
                );
            }
        }
    }
    ctx.flush();
}

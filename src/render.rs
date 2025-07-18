use crate::*;

/// Renders ascii characters.
///
/// # Panics
/// This function cannot panic under normal circumstances.
pub fn render_ascii(
    ctx: &mut Terminal,
    camera: &Camera,
    positions: &Components<Point>,
    multi_sprites: &Components<MultiSprite>,
    sprites: &Components<Sprite>,
) {
    for (pos, multi) in join!(&positions && &multi_sprites) {
        let multi = multi.expect("Entity in MultiSprite join should have MultiSprite component");
        let pos = pos.expect("Entity in MultiSprite join should have Point component");
        for y in 0..multi.height as i32 {
            for x in 0..multi.width as i32 {
                let idx = (x + y * multi.width as i32) as usize;
                ctx.print_color(
                    pos.x + x - camera.position.x,
                    pos.y + y - camera.position.y,
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
    for (pos, sprite) in join!(&positions && &sprites) {
        let pos = pos.expect("Entity in Sprite join should have Point component");
        let sprite = sprite.expect("Entity in Sprite join should have Sprite component");

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
    ctx.flush();
}

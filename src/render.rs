use crate::*;

/// Renders ascii characters.
pub fn render_ascii<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: &Components<Point>,
    multi_sprites: &Components<MultiSprite>,
    sprites: &Components<Sprite>,
) {
    #[cfg(not(feature = "headless"))]
    {
        // TODO add the camera check to multisprites
        for (pos, sprite) in join!(&positions && &multi_sprites) {
            sprite.unwrap().tile.render(
                ctx,
                Point::new(
                    pos.unwrap().x - camera.position.x,
                    pos.unwrap().y - camera.position.y,
                ),
            );
        }
        for (pos, sprite) in join!(&positions && &sprites) {
            let pos = pos.unwrap();
            let sprite = sprite.unwrap();

            // the position of the camera is its position in the world, not
            // its offset on screen.
            // TODO make it so we can define the camera on screen as a square.
            // (add screenspace coords)
            if camera.size.x > 0 && camera.size.y > 0 && position_inside_rect(
                pos.x - camera.position.x,
                pos.y - camera.position.y,
                0,
                0,
                camera.size.x as u32,
                camera.size.y as u32,
            ) {
                ctx.set(
                    pos.x - camera.position.x,
                    pos.y - camera.position.y,
                    sprite.fg,
                    sprite.bg,
                    sprite.glyph,
                );
            }
        }
    }
}

/// Renders 2d tile sprites characters.
/// Only available using the `opengl` feature.
#[cfg(feature = "opengl")]
pub fn render_sprites<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: &Components<Point>,
    sprites: &Components<SpriteIndex>,
    viewshed: Option<&Viewshed>,
) {
    #[cfg(not(feature = "headless"))]
    for (pos, sprite) in join!(&positions && &sprites) {
        let pos = pos.unwrap();
        let sprite = sprite.unwrap();

        if camera.size.x > 0 && camera.size.y > 0 && position_inside_rect(
            pos.x - camera.position.x,
            pos.y - camera.position.y,
            0,
            0,
            camera.size.x as u32,
            camera.size.y as u32,
        ) {
            if viewshed.is_none() || viewshed.unwrap().visible_tiles.contains(&pos) {
                ctx.add_sprite(
                    Rect::with_size(
                        (pos.x - camera.position.x) * 1,
                        (pos.y - camera.position.y) * 1,
                        // TODO make this dynamic.
                        1,
                        1,
                    ),
                    0,
                    RGBA::named(WHITE),
                    sprite.0,
                );
            // TODO this will not hide units that are not creeps, use a better way of checking for enemy units.
            // TODO this shouldn't even be in the engine at all.
            } else if sprite.0 != 9 {
                ctx.add_sprite(
                    Rect::with_size(
                        (pos.x - camera.position.x) * 1,
                        (pos.y - camera.position.y) * 1,
                        // TODO make this dynamic.
                        1,
                        1,
                    ),
                    0,
                    RGBA::from_u8(160, 160, 160, 255),
                    sprite.0,
                );
            }
        }
    }
}

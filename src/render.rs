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
        for (pos, multi) in join!(&positions && &multi_sprites) {
            let multi = multi.unwrap();
            for y in 0..multi.height as i32 {
                for x in 0..multi.width as i32 {
                    let idx = (x + y * multi.width as i32) as usize;
                    ctx.print_color(
                        pos.unwrap().x + x - camera.position.x,
                        pos.unwrap().y + y - camera.position.y,
                        multi.fg[idx],
                        multi.bg[idx],
                        multi.ascii.chars().nth(idx).unwrap(),
                    );
                }
            }
        }
        for (pos, sprite) in join!(&positions && &sprites) {
            let pos = pos.unwrap();
            let sprite = sprite.unwrap();

            // the position of the camera is its position in the world, not
            // its offset on screen.
            // TODO make it so we can define the camera on screen as a square.
            // (add screenspace coords)
            if position_inside_rect(
                pos.x - camera.position.x,
                pos.y - camera.position.y,
                0,
                0,
                camera.size.x as u32,
                camera.size.y as u32,
            ) {
                ctx.set(
                    pos.x - camera.position.x + camera.screen_position.x,
                    pos.y - camera.position.y + camera.screen_position.y,
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
    targets: &Components<RenderTarget>,
    entities: &Entities,
    viewshed: Option<&Viewshed>,
) {
    #[cfg(not(feature = "headless"))]
    for (entity, pos, sprite) in join!(&entities && &positions && &sprites) {
        if let Some(target) = targets.get(entity.unwrap()) {
            ctx.set_active_console(target.0);
        } else {
            ctx.set_active_console(1);
        }

        // will crash for entities that have no position or sprite but a target.
        let pos = pos.unwrap();
        let sprite = sprite.unwrap();

        if position_inside_rect(
            pos.x - camera.position.x,
            pos.y - camera.position.y,
            0,
            0,
            camera.size.x as u32,
            camera.size.y as u32,
        ) {
            //if viewshed.is_none() || viewshed.unwrap().visible_tiles.contains(&pos) {
            ctx.add_sprite(
                Rect::with_size(
                    (pos.x - camera.position.x + camera.screen_position.x) * 1,
                    (pos.y - camera.position.y + camera.screen_position.y) * 1,
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
            /*} else if sprite.0 != 9 {
                ctx.add_sprite(
                    Rect::with_size(
                        (pos.x - camera.position.x + camera.screen_position.x) * 1,
                        (pos.y - camera.position.y + camera.screen_position.y) * 1,
                        // TODO make this dynamic.
                        1,
                        1,
                    ),
                    0,
                    RGBA::from_u8(160, 160, 160, 255),
                    sprite.0,
                );
            }*/
        }
    }
    for (entity, pos, multi) in join!(&entities && &positions && &multi_sprites) {
        if let Some(target) = targets.get(entity.unwrap()) {
            ctx.set_active_console(target.0);
        } else {
            ctx.set_active_console(1);
        }

        // will crash for entities that have no position or sprite but a target.
        let pos = pos.unwrap();
        let multi = multi.unwrap();

        if position_inside_rect(
            pos.x - camera.position.x,
            pos.y - camera.position.y,
            0,
            0,
            camera.size.x as u32,
            camera.size.y as u32,
        ) {
            for (pos, multi) in join!(&positions && &multi_sprites) {
                let multi = multi.unwrap();
                for y in 0..multi.height as i32 {
                    for x in 0..multi.width as i32 {
                        let idx = (x + y * multi.width as i32) as usize;
                        ctx.add_sprite(
                            Rect::with_size(
                                (pos.unwrap().x + x - camera.position.x + camera.screen_position.x) * 1,
                                (pos.unwrap().y + y - camera.position.y + camera.screen_position.y) * 1,
                                1,
                                1,
                            ),
                            0,
                            RGBA::named(WHITE),
                            multi.sprite_indices[idx],
                        );
                    }
                }
            }
        }
    }
}

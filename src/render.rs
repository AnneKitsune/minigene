use crate::*;

/// Renders ascii characters.
pub fn render_ascii<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: &Components<Point>,
    multi_sprites: &Components<MultiSprite>,
    sprites: &Components<Sprite>,
) {
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
        ctx.set(
            pos.x - camera.position.x,
            pos.y - camera.position.y,
            sprite.fg,
            sprite.bg,
            sprite.glyph,
        );
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
) {
    for (pos, sprite) in (&positions, &sprites).join() {
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
    }
}

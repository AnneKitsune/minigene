use crate::*;

/// Renders ascii characters.
pub fn render_ascii<'a>(
    ctx: &mut BTerm,
    camera: &Camera,
    positions: ReadStorage<'a, Point>,
    multi_sprites: ReadStorage<'a, MultiSprite>,
    sprites: ReadStorage<'a, Sprite>,
) {
    for (pos, sprite) in (&positions, &multi_sprites).join() {
        sprite.tile.render(
            ctx,
            Point::new(pos.x - camera.position.x, pos.y - camera.position.y),
        );
    }
    for (pos, sprite) in (&positions, &sprites).join() {
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
pub fn render_sprites<'a>(
    _ctx: &mut BTerm,
    _camera: &Camera,
    positions: ReadStorage<'a, Point>,
    sprites: ReadStorage<'a, SpriteIndex>,
) {
    for (_pos, _sprite) in (&positions, &sprites).join() {
        #[cfg(feature = "opengl")]
        {
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
}

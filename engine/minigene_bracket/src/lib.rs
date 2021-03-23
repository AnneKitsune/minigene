pub use bracket_lib::prelude as bracket;

use bracket::{
    a_star_search, add_wasm_support, field_of_view, main_loop, to_cp437, Algorithm2D, BError,
    BEvent, BTerm, BTermBuilder, BaseMap, GameState, MultiTileSprite, NavigationPath, Point, Rect,
    SmallVec, SpriteSheet, VirtualKeyCode, BLACK, BLUE, EMBED, GREEN, INPUT, RED, RGBA, WHITE,
    YELLOW,
};

/// A single colored letter sprite.
pub struct Sprite {
    /// The char symbol displayed.
    pub glyph: u16,
    /// The foreground color.
    pub fg: RGBA,
    /// The background color.
    pub bg: RGBA,
}

/// The index of a 2d sprite. Created from `SpriteSheet`'s index.
pub struct SpriteIndex(pub usize);

/// A text-based sprite that is multiple tiles wide/high.
#[derive(new)]
pub struct MultiSprite {
    /// The tile.
    pub tile: MultiTileSprite,
}

/// The path calculated by the Ai that it will follow.
#[derive(new, Default)]
pub struct AiPath {
    /// The path.
    pub path: NavigationPath,
}

/// Indicates that the ai should calculate an AiPath from the current position
/// towards this destination.
#[derive(new)]
pub struct AiDestination {
    /// The destination position.
    pub target: Point,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

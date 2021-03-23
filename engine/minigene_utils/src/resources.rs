/// Sets the game speed multiplier.
#[derive(Debug, Clone)]
pub struct GameSpeed(pub f32);

impl Default for GameSpeed {
    fn default() -> Self {
        GameSpeed(1.0)
    }
}

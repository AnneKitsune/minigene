/// Indicates that the ai should calculate an AiPath from the current position
/// towards this destination.
#[derive(new)]
pub struct GotoStraight {
    /// The destination position.
    pub target: Point,
    /// The speed at which the entity moves in tiles/second.
    pub speed: f32,
}

/// Indicates that the ai should calculate an AiPath from the current position
/// towards this entity's position.
#[derive(new)]
pub struct GotoEntity {
    /// The destination entity we are trying to reach.
    pub entity: Entity,
    /// The speed at which the entity moves in tiles/second.
    pub speed: f32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

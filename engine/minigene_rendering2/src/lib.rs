use std::collections::HashSet;

// TODO consider changing this to a component?
/// Used to change the visible space of the world on screen.
#[derive(new)]
pub struct Camera {
    /// The position of the camera.
    pub position: Point,
    /// The size in tiles that the camera can view.
    pub size: Point,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::new(0, 0),
            size: Point::new(80, 50),
        }
    }
}

/// Everything we can see from.
#[derive(new)]
pub struct Viewshed {
    /// Which tiles we can see.
    pub visible_tiles: HashSet<Point>,
}

impl Default for Viewshed {
    fn default() -> Self {
        Self {
            visible_tiles: HashSet::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

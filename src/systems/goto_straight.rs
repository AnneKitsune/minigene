use crate::*;
use uuidmap::Table;

/// Moves an entity towards its target position using `GotoStraight` component.
///
/// # Panics
/// Panics if an entity has `GotoStraight` but lacks Point or destination Point.
///
/// # Errors
/// Currently always returns `Ok(())`.
pub fn goto_straight_system(
    gotos: &Table<GotoStraight>,
    positions: &mut Table<Point>,
    destinations: &Table<Point>,
) {
    for goto in gotos.values() {
        // Skip entities without position components
        let Some(pos) = positions.get_mut(goto.destination_id) else {
            continue;
        };
        // Get destination point
        let Some(dest) = destinations.get(goto.current_position_id) else {
            continue;
        };

        let mut delta_x = dest.x - pos.x;
        let mut delta_y = dest.y - pos.y;

        // If already at destination, skip
        if delta_x == 0 && delta_y == 0 {
            continue;
        }

        // Apply movement
        match delta_x.cmp(&0) {
            std::cmp::Ordering::Greater => pos.x += 1,
            std::cmp::Ordering::Less => pos.x -= 1,
            std::cmp::Ordering::Equal => {}
        }

        // Update deltas after movement
        delta_x = dest.x - pos.x;
        delta_y = dest.y - pos.y;

        // Stop if reached destination early
        if delta_x == 0 && delta_y == 0 {
            break;
        }
    }
}

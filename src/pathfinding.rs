use crate::components::{CollisionMap, Point};
use std::collections::{BinaryHeap, HashMap};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub path: Vec<Point>,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: Point,
    g_score: i32,
    f_score: i32,
    came_from: Option<Point>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const fn heuristic(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

pub fn astar(start: Point, goal: Point, collision_map: &CollisionMap) -> Option<Path> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    open_set.push(Node {
        position: start,
        g_score: 0,
        f_score: heuristic(start, goal),
        came_from: None,
    });

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));

    while let Some(current) = open_set.pop() {
        if current.position == goal {
            return reconstruct_path(&came_from, current.position);
        }

        for neighbor in get_neighbors(current.position, collision_map) {
            let tentative_g_score = g_score[&current.position] + 1;

            if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, goal));
                open_set.push(Node {
                    position: neighbor,
                    g_score: tentative_g_score,
                    f_score: f_score[&neighbor],
                    came_from: Some(current.position),
                });
            }
        }
    }

    None
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, mut current: Point) -> Option<Path> {
    let mut total_path = vec![current];
    while let Some(&next) = came_from.get(&current) {
        total_path.push(next);
        current = next;
    }
    total_path.reverse();
    Some(Path { path: total_path })
}

fn get_neighbors(position: Point, collision_map: &CollisionMap) -> Vec<Point> {
    let mut neighbors = Vec::new();
    let directions = [
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
    ];

    for direction in &directions {
        let neighbor = Point {
            x: position.x + direction.x,
            y: position.y + direction.y,
        };
        if neighbor.x < 0 || neighbor.y < 0 {
            continue;
        }
        if collision_map.is_inbound(neighbor.x as u32, neighbor.y as u32)
            && !collision_map.is_set(neighbor.x as u32, neighbor.y as u32)
        {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_no_obstacles() {
        let collision_map = CollisionMap::new(5, 5);
        let start = Point { x: 0, y: 0 };
        let goal = Point { x: 4, y: 4 };
        let path = astar(start, goal, &collision_map).unwrap();
        assert_eq!(path.path.len(), 9); // Start to goal is 8 steps + start
    }

    #[test]
    fn test_astar_with_obstacles() {
        let mut collision_map = CollisionMap::new(5, 5);
        collision_map.set(2, 2);
        collision_map.set(3, 2);
        let start = Point { x: 0, y: 0 };
        let goal = Point { x: 4, y: 4 };
        let path = astar(start, goal, &collision_map).unwrap();
        assert_eq!(path.path.len(), 9); // Start to goal is 9 steps + start
    }

    #[test]
    fn test_astar_no_path() {
        let mut collision_map = CollisionMap::new(5, 5);
        for x in 0..5 {
            collision_map.set(x, 2);
        }
        let start = Point { x: 0, y: 0 };
        let goal = Point { x: 4, y: 4 };
        let path = astar(start, goal, &collision_map);
        assert!(path.is_none());
    }

    #[test]
    fn test_astar_start_equals_goal() {
        let collision_map = CollisionMap::new(5, 5);
        let start = Point { x: 2, y: 2 };
        let goal = Point { x: 2, y: 2 };
        let path = astar(start, goal, &collision_map).unwrap();
        assert_eq!(path.path.len(), 1); // Only the start point
    }
}

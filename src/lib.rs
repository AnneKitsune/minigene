//! A minimal game engine for turn-based and action games with ECS architecture.
//!
//! This crate provides:
//! - Entity Component System (ECS) with `planck_ecs`
//! - Terminal rendering with `crossterm`
//! - Pathfinding algorithms
//! - Common game components and systems

pub extern crate game_features;
pub extern crate hibitset;

extern crate crossterm;

pub use game_clock::*;
pub use game_features::*;
pub use hibitset::BitSet as HBitSet;
pub use stopwatch2::*;

pub use game_engine_core::*;
pub use planck_ecs::*;
pub use planck_ecs_bundle::*;
pub use planck_game_features::*;

// macro re-export
pub use derive_new::*;

mod components;
mod data;
mod engine;
mod event;
mod pathfinding;
mod render;
mod resources;
mod systems;
mod terminal;
mod utils;

/// Contains commonly used imports for game development with this engine.
///
/// Import this module to conveniently access:
/// - Default components like `Point`, `Sprite`
/// - Systems like `render_system`, `input_driver`
/// - Terminal utilities
pub mod prelude;

pub use self::components::*;
pub use self::data::*;
pub use self::engine::*;
pub use self::event::*;
pub use self::pathfinding::*;
pub use self::render::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::terminal::*;
pub use self::utils::*;

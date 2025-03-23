# Data structure convention
We use an entity-component-system like library called planck_ecs.
However, instead of entities and components, we use in-memory tables from a library called uuidmap, like in a relational database.

## Defining Systems in Planck ECS

Systems in Planck ECS are simply functions that operate on resources, which can be any type implementing Default. Most often, those resources will be of type Table<T>. Here's how to work with them:

### Regular Function System

```rust
// Define a system as a regular function
fn change_color_system(colors: &mut Table<Color>) -> SystemResult {
    // System logic here
    Ok(())
}
```


### Converting to System for Dispatcher

```rust
use planck_ecs::*;
// Create world
let mut world = World::default();

// Convert function to system
let color_system = change_color_system.system();

// Build dispatcher with the system
let mut dispatcher = DispatcherBuilder::new()
    .add_system(color_system)
    .build(&mut world);

// Run systems
dispatcher.run_seq(&world).unwrap();

// Maintain world
world.maintain();
```


### Adding System to a Bundle

```rust
use planck_ecs::*;
// Define a bundle
struct GameBundle;

impl Bundle for GameBundle {
    fn systems() -> Vec<System> {
        vec![
            change_color_system.system(),
            // Add more systems here
        ]
    }
}

// Use the bundle with a dispatcher
let mut builder = DispatcherBuilder::default();
builder = GameBundle::insert(builder);
```


### System Argument Order Rules

When defining system parameters, you must follow this rule:

- All immutable references (`&`) must come before all mutable references (`&mut`)

**Correct order example:**

```rust
fn correct_system(
    colors: &Table<Color>,
    positions: &mut Table<Position>,  // Mutable references after
    velocities: &mut Table<Velocity>
) -> SystemResult {
    // System logic
    Ok(())
}
```

**Incorrect order example:**

```rust
// This won't work!
fn incorrect_system(
    positions: &mut Table<Position>,  // Mutable reference
    colors: &Table<Color>,            // Immutable reference
    velocities: &mut Table<Velocity>  // Mutable reference
) -> SystemResult {
    // System logic
    Ok(())
}
```

The correct version would be:

```rust
fn corrected_system(
    colors: &Table<Color>,            // Immutable reference first
    positions: &mut Table<Position>,  // Then mutable references
    velocities: &mut Table<Velocity>
) -> SystemResult {
    // System logic
    Ok(())
}
```

## UuidMap
Creating a table and elements
```rust
use uuidmap::Table;
let mut table = Table::<MyDataType>::default();
let uuid: u128 = table.add(my_data);
```

Function reference for Table<T>:
```rust
/// Add a new value with random key.
/// This is what you want to use 95% of the time.
pub fn add(&mut self, value: T) -> u128
/// Add a new value with manual key. Usually used during deserialization.
/// Might be used for performance reasons when using a Table as a Map.
/// For example, a map KeyCode -> GameEvent.
pub fn add_with_key(&mut self, key: u128, value: T)
/// Get a value by key.
pub fn get(&self, key: u128) -> Option<&T>
/// Get a value by key.
pub fn get_mut(&mut self, key: u128) -> Option<&mut T>
/// Remove an element using it's key.
pub fn remove(&mut self, key: u128) -> Option<T>
/// Get an iterator over the contained values.
pub fn values(&self) -> impl Iterator<Item = &T>
pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T>
/// Return an iterator over keys.
pub fn keys(&self) -> std::collections::hash_map::Keys<u128, usize>
/// Creates a Table with a specific initial capacity.
pub fn with_capacity(capacity: usize) -> Self
/// Get the number of elements stored.
pub fn count(&self) -> usize
/// Empty out everything.
pub fn clear(&mut self)

```

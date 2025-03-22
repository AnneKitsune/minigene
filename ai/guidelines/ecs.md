# Data structure convention
We use an entity-component-system like library called planck_ecs.
However, instead of entities and components, we use in-memory tables from a library called uuidmap, like in a relational database.

## Defining Systems in Planck ECS

Systems in Planck ECS are simply functions that operate on components and entities. Here's how to work with them:

### Regular Function System

```rust
// Define a system as a regular function
fn change_color_system(colors: &mut Components<Color>) -> SystemResult {
    // System logic here
    Ok(())
}
```


### Converting to System for Dispatcher

```rust
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
    entities: &Entities,           // Immutable references first
    colors: &Components<Color>,
    positions: &mut Components<Position>,  // Mutable references after
    velocities: &mut Components<Velocity>
) -> SystemResult {
    // System logic
    Ok(())
}
```

**Incorrect order example:**

```rust
// This won't work!
fn incorrect_system(
    positions: &mut Components<Position>,  // Mutable reference
    colors: &Components<Color>,            // Immutable reference
    velocities: &mut Components<Velocity>  // Mutable reference
) -> SystemResult {
    // System logic
    Ok(())
}
```

The correct version would be:

```rust
fn corrected_system(
    colors: &Components<Color>,            // Immutable reference first
    positions: &mut Components<Position>,  // Then mutable references
    velocities: &mut Components<Velocity>
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

Get/GetMut
```rust
// Get a reference to data by UUID
if let Some(data) = table.get(uuid) {
    println!("Found data: {:?}", data);
}

// Get a mutable reference
if let Some(data) = table.get_mut(uuid) {
    data.some_field = new_value;
}

```

Remove
```rust
// Remove data by UUID
if let Some(removed_data) = table.remove(uuid) {
    println!("Removed: {:?}", removed_data);
}
```


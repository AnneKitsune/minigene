Implement InputEvent enum with variants for game actions (Move, Look, etc.) and derive Debug, Clone, PartialEq for testing, including unit tests for variant creation and comparison
Create InputMapping struct using HashMap to store key-to-event bindings and key combo sequences with methods for adding/removing mappings, including unit tests for valid/invalid key bindings and combo detection
Implement ActionQueue struct with VecDeque<InputEvent> storage, methods push(), pop_last(), clear(), and process(), including unit tests for queue operations and order preservation
Add combo state tracking with timeout logic in InputMapping including current sequence buffer and timestamp, including unit tests for combo expiration and partial sequence handling
Create input processing system that converts key presses to events using InputMapping and updates ActionQueue, including unit tests for single/multi-key inputs and timeout scenarios
Implement clear (e.g., 'C') and undo (e.g., 'Z') key bindings as special InputEvent variants that modify ActionQueue directly, including unit tests for queue clearance and rollback operations
Write integration tests covering full input pipeline from key presses to queue updates with combo sequences, timeout cases, and special command keys

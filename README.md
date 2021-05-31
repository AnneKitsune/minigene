
# Minigene - A Minimalist Game Ecosystem

Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Minigene is a game engine currently specialized in ascii/2D tiled games and fast prototyping.
It is planned to support non tiled 2D and 3D in the near future.

Minigene is not a single crate, it is a collection of crates, an ecosystem for
game development.

For more information on the design choices of Minigene, visit the [blog](https://jojolepro.com/blog/2021-05-31_minigene_and_the_future/index.html).

Read the [documentation](https://docs.rs/minigene).

# Why would you use this engine?

* Gets out of your way and lets you choose how you want to build your game
* Composed of small and specifically chosen libraries
* Composable
* WASM Compatible
* Pick and choose the features that you need

# Usage
Add the following to you Cargo.toml file:
```
minigene = { git = "https://github.com/jojolepro/minigene" }
```

By default, Minigene will have the `terminal` feature enabled.
If you want it to create a window and use 2D tiling mode, use:
```
minigene = { git = "https://github.com/jojolepro/minigene", default-features = false, features = ["opengl"] }
```

Look in the examples/minimal folder for a game template.

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)


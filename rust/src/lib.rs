use godot::prelude::*;

mod player;
mod enemy;
mod main_scene;
mod hud;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

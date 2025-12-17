//! Save/Load module for the 2D geometry editor
//!
//! This module provides functionality for saving and loading selected shapes
//! from the MainScene layer to and from files.

pub mod components;
pub mod plugin;
pub mod systems;

pub use plugin::SaveLoadPlugin;

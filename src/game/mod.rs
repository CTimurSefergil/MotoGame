//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod map_change;
mod movement;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    // Order new `GameSystem` variants by adding them here:
    app.configure_sets(
        Update,
        (GameSystem::Movement, GameSystem::MapChange).chain(),
    );
    app.add_plugins((
        audio::plugin,
        movement::plugin,
        spawn::plugin,
        map_change::plugin,
    ));
}

/// High-level groupings of systems for your game.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum GameSystem {
    /// Handles player movement.
    Movement,
    /// Handles map changes.
    MapChange,
}

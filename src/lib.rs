//! Minecraft physics engine — AABB collision detection, gravity, friction, voxel shapes.
//!
//! This crate implements the per-tick physics pipeline matching vanilla
//! Minecraft's `LivingEntity.travel()` and `Entity.move()`. It is fully
//! decoupled from ECS frameworks — all state is passed as plain types
//! (`DVec3`, `Aabb`, `bool`, `f32`).
//!
//! # Module Layout
//!
//! - [`constants`] — all physics constants (gravity, drag, friction, etc.)
//! - [`voxel_shape`] — block collision geometry representation
//! - [`collision`] — per-axis sweep collision and obstacle collection
//! - [`tick`] — the main per-tick physics update
//! - [`slow_blocks`] — block speed/jump factor modifiers
//! - [`jump`] — jump impulse application

#![warn(missing_docs)]
#![deny(unsafe_code)]

use oxidized_mc_types::BlockPos;

pub mod collision;
pub mod constants;
pub mod jump;
pub mod slow_blocks;
pub mod tick;
pub mod voxel_shape;

/// Entity hitbox dimensions.
///
/// Width and height in meters (blocks). Used to reconstruct the AABB
/// after a position update.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityDimensions {
    /// Width of the hitbox (meters).
    pub width: f32,
    /// Height of the hitbox (meters).
    pub height: f32,
}

/// Error returned when a block position cannot be read.
///
/// Mirrors the relevant variants of the game's `LevelError`, but without
/// depending on the game crate's chunk types.
#[derive(Debug, thiserror::Error)]
pub enum BlockAccessError {
    /// The requested chunk is not loaded.
    #[error("chunk not loaded at ({chunk_x}, {chunk_z})")]
    ChunkNotLoaded {
        /// Chunk X coordinate.
        chunk_x: i32,
        /// Chunk Z coordinate.
        chunk_z: i32,
    },

    /// Block position is outside valid world bounds.
    #[error("position out of bounds: ({x}, {y}, {z})")]
    OutOfBounds {
        /// X coordinate.
        x: i32,
        /// Y coordinate.
        y: i32,
        /// Z coordinate.
        z: i32,
    },
}

/// Provides read access to block states in the world.
///
/// The physics engine needs to query block states for collision detection,
/// friction, and speed factors. Callers implement this trait to bridge
/// their world storage into the physics pipeline.
pub trait BlockGetter {
    /// Returns the block state ID at the given position.
    ///
    /// # Errors
    ///
    /// Returns [`BlockAccessError`] if the position is in an unloaded chunk
    /// or outside valid world bounds.
    fn get_block_state(&self, pos: BlockPos) -> Result<u32, BlockAccessError>;
}

//! Services related to Kill operations.
//!
//! This module provides traits and implementations for creating and managing Kill instances.

use crate::models::Kill;

/// Defines the core functionality for creating Kill instances.
pub trait KillService {
    /// Creates a new Kill instance.
    ///
    /// # Arguments
    ///
    /// * `killer` - The name of the player or entity that caused the kill.
    /// * `victim` - The name of the player who was killed.
    /// * `weapon` - The means of death or weapon used for the kill.
    ///
    /// # Returns
    ///
    /// A new instance of the implementing type.
    fn new(killer: String, victim: String, weapon: String) -> Self;
}

impl KillService for Kill {
    fn new(killer: String, victim: String, weapon: String) -> Self {
        Kill {
            killer,
            victim,
            weapon,
        }
    }
}
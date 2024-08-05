//! Represents individual kill events in Quake games.

/// Stores information about a single kill event.
pub struct Kill {
    /// The name of the player or entity that caused the kill.
    /// Note: This can be \<world\> for environmental deaths.
    pub killer: String,

    /// The name of the player who was killed.
    pub victim: String,

    /// The means of death or weapon used for the kill.
    pub weapon: String,
}
//! Execution stages for systems.

/// The execution stage for a system.
///
/// Systems are executed in order of their stage, from Start to End.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stage {
    /// First stage, for initial setup and preparation systems.
    Start,
    
    /// Early update stage, for systems that should run before the main update.
    EarlyUpdate,
    
    /// Main update stage, for the core game logic systems.
    Update,
    
    /// Late update stage, for systems that should run after the main update.
    LateUpdate,
    
    /// Pre-render stage, for systems that prepare rendering.
    PreRender,
    
    /// Render stage, for systems that perform actual rendering.
    Render,
    
    /// Post-render stage, for systems that run after rendering.
    PostRender,
    
    /// Final stage, for cleanup and shutdown systems.
    End,
}

impl Default for Stage {
    fn default() -> Self {
        Self::Update
    }
}

impl Stage {
    /// Returns the stage's ordinal value for sorting.
    pub fn ordinal(&self) -> usize {
        match self {
            Stage::Start => 0,
            Stage::EarlyUpdate => 1,
            Stage::Update => 2,
            Stage::LateUpdate => 3,
            Stage::PreRender => 4,
            Stage::Render => 5,
            Stage::PostRender => 6,
            Stage::End => 7,
        }
    }
    
    /// Returns all stages in execution order.
    pub fn all() -> [Stage; 8] {
        [
            Stage::Start,
            Stage::EarlyUpdate,
            Stage::Update,
            Stage::LateUpdate,
            Stage::PreRender,
            Stage::Render,
            Stage::PostRender,
            Stage::End,
        ]
    }
}

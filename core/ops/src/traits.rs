use anyhow::Result;
use hymod_core_plan::Step;

/// Trait for handling a single implementation of a Step logic.
pub trait OpHandler: Send + Sync {
    /// Execute the logic for the given step.
    /// Handlers should verify the correct Step variant is passed (e.g. via matching)
    /// and panic or error if mismatch (though Executor should prevent this).
    fn handle(&self, step: &Step) -> Result<()>;
}

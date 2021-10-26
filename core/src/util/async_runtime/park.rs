/// Trait for park a thread.
pub trait Park {
    /// Unparker type associated with this Parker.
    type Unparker: Unpark;

    /// Parker is tasked with construct unparker.
    fn unparker(&mut self) -> Self::Unparker;

    fn park(&self);
}

/// Trait for unpark a thread.
pub trait Unpark: Send + Sync + 'static {
    fn unpark(&self);
}

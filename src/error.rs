#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A greeting was requested without a usable name.
    #[error("a greeting name cannot be empty")]
    EmptyName,
}

/// Plugin result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

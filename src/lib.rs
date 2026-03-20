pub mod impls;
pub mod jets;
pub mod linalg;
pub mod traits;

/// Returns the full version string, e.g. "1.0.0+a3f7b2c" or "1.0.1-dev+f82c1a0-dirty".
pub fn version() -> &'static str {
    env!("GIT_VERSION")
}

//! Linear algebra primitives generic over `T: DifferentiableMath`.
//!
//! Provides specialized stack-allocated implementations for common sizes
//! (3×3, 6×6, 9×9) and generic `Vec`-backed implementations for arbitrary
//! \\(N \times N\\) systems. All functions work with both `f64` and Jet types.
//!
//! # Numerical tolerances
//!
//! - [`NOLAN_REL_TOL`] — relative tolerance used by scaled partial pivoting
//!   in `mat_solve` / `mat_inv` / `matN_solve` / `matN_inv` and by the
//!   relative determinant guard in `mat3_inv` / `mat3_solve`.
//! - [`NOLAN_MIN_SCALE`] — floor below which row-infinity norms are
//!   treated as zero (avoids division by zero when computing scaled
//!   pivot ratios).

/// Relative tolerance for scaled pivoting and determinant guards. Matrix
/// is treated as singular if the largest scaled pivot ratio falls below
/// this value.
///
/// The scaled-pivoting strategy itself is the Numerical Recipes §2.5
/// "implicit pivoting" prescription (Press et al., *Numerical Recipes
/// 3rd ed.*, 2007): pick the pivot maximising
/// \\(|a_{ik}| / \max_j |a_{ij}|\\) over the active sub-matrix rather
/// than just \\(|a_{ik}|\\), which is robust to mixed-scale rows where
/// a plain absolute-value pivot would pick a row whose pivot is only
/// large because the entire row is large.
pub const NOLAN_REL_TOL: f64 = 1e-14;

/// Floor for row-infinity norms in scaled partial pivoting. Rows with
/// norm below this are excluded from pivot ratio comparisons (otherwise
/// we would divide by zero or denormal floats).
///
/// Justification: `sqrt(f64::MIN_POSITIVE)` ≈ 1.5e-154. We use a
/// slightly conservative 1e-150. The physical worst case is covariance
/// in m² with entries near 1e+20 (AU²-scale uncertainty in meters), or
/// AU² covariance with nanometer entries near 1e-44 AU² — 1e-150 sits
/// well below any plausible physical input scale.
pub const NOLAN_MIN_SCALE: f64 = 1e-150;

pub mod generic;
pub mod mat3;
pub mod mat6;
pub mod mat9;
pub mod vec3;

pub use generic::{
    mahalanobis_distance_squared, mahalanobis_distance_squared_with_inv, mat_ata, mat_cholesky,
    mat_det, mat_eigenvector_max, mat_frobenius, mat_inv, mat_largest_singular_value, mat_log_det,
    mat_mul, mat_quadratic_form, mat_solve, mat_symmetric_eigen, mat_symmetrize, mat_trace,
    mat_trace_cube, mat_trace_product, mat_transpose, mat_vec_mul, vec_norm,
};
pub use mat3::{
    mat3_det, mat3_inv, mat3_mul, mat3_solve, mat3_transpose, mat3_transpose_vec_mul, mat3_vec_mul,
    sym_eigenvalues_3,
};
pub use mat6::{
    mat6_add, mat6_inv, mat6_mul, mat6_solve, mat6_symmetrize, mat6_transpose, mat6_vec_mul,
};
pub use mat9::{
    mat9_add, mat9_inv, mat9_mul, mat9_solve, mat9_symmetrize, mat9_transpose, mat9_vec_mul,
};
pub use vec3::{cross3, dot3, norm_squared3, norm3, normalize3};

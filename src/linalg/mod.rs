//! Linear algebra primitives generic over `T: DifferentiableMath`.
//!
//! Provides specialized stack-allocated implementations for common sizes
//! (3×3, 6×6, 9×9) and generic `Vec`-backed implementations for arbitrary
//! \\(N \times N\\) systems. All functions work with both `f64` and Jet types.

pub mod generic;
pub mod mat3;
pub mod mat6;
pub mod mat9;
pub mod vec3;

pub use generic::{
    mahalanobis_distance_squared, mahalanobis_distance_squared_with_inv, mat_cholesky,
    mat_eigenvector_max, mat_inv, mat_log_det, mat_quadratic_form, mat_solve, mat_symmetrize,
    mat_trace, mat_trace_product, mat_vec_mul, vec_norm,
};
pub use mat3::{
    mat3_inv, mat3_mul, mat3_solve, mat3_transpose, mat3_transpose_vec_mul, mat3_vec_mul,
};
pub use mat6::{
    mat6_add, mat6_inv, mat6_mul, mat6_solve, mat6_symmetrize, mat6_transpose, mat6_vec_mul,
};
pub use mat9::{
    mat9_add, mat9_inv, mat9_mul, mat9_solve, mat9_symmetrize, mat9_transpose, mat9_vec_mul,
};
pub use vec3::{cross3, dot3, norm_squared3, norm3, normalize3};

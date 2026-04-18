<img src="docs/nolan-icon.png" width="140" alt="nolan">

# nolan
Const-generic hyperdual numbers for automatic differentiation in Rust

<a href="https://github.com/Empyrean-Dynamics/nolan/actions/workflows/rust.yml"><img src="https://github.com/Empyrean-Dynamics/nolan/actions/workflows/rust.yml/badge.svg" alt="CI"></a>
<a href="https://claude.ai"><img src="https://img.shields.io/badge/Built%20with-Claude%20Code-D97757?logo=anthropic&logoColor=white&style=flat-square" alt="Built with Claude Code"></a>
<br>
<a href="https://www.empyrean-dynamics.com"><img src="https://img.shields.io/badge/Website-empyrean--dynamics.com-1a1a2e?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IndoaXRlIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iMTAiLz48bGluZSB4MT0iMiIgeTE9IjEyIiB4Mj0iMjIiIHkyPSIxMiIvPjxwYXRoIGQ9Ik0xMiAyYTE1LjMgMTUuMyAwIDAgMSA0IDEwIDE1LjMgMTUuMyAwIDAgMS00IDEwIDE1LjMgMTUuMyAwIDAgMS00LTEwIDE1LjMgMTUuMyAwIDAgMSA0LTEweiIvPjwvc3ZnPg==&logoColor=white&style=flat-square" alt="Website"></a>
<a href="https://github.com/Empyrean-Dynamics"><img src="https://img.shields.io/badge/GitHub-Empyrean--Dynamics-1a1a2e?logo=github&logoColor=white&style=flat-square" alt="GitHub"></a>

---

nolan provides first-, second-, and third-order jet types that propagate exact
derivatives through arbitrary computations. It is designed for astrodynamics
applications where derivatives of physical parameters (position, velocity, time)
are needed for orbit determination, covariance propagation, and sensitivity analysis.

## Types

### Jet1\<N\>

First-order jet: tracks a value and its gradient with respect to N parameters.

```rust
use nolan::jets::Jet1;

// Create a variable seeded at parameter index 0
let x = Jet1::<3>::variable(2.0, 0);
let y = Jet1::<3>::variable(3.0, 1);

let f = x * x + y;
assert_eq!(f.value, 7.0);     // 2² + 3
assert_eq!(f.grad[0], 4.0);   // ∂f/∂x = 2x = 4
assert_eq!(f.grad[1], 1.0);   // ∂f/∂y = 1
```

### Jet2\<N, H\>

Second-order jet: tracks value, gradient, and the full Hessian matrix
(stored as a lower-triangular array of size H = N(N+1)/2).

```rust
use nolan::jets::{Jet2, hess_size};

let x = Jet2::<2, { hess_size(2) }>::variable(2.0, 0);
let y = Jet2::<2, { hess_size(2) }>::variable(3.0, 1);

let f = x * x * y;
// f = x²y, value = 12
// ∂f/∂x = 2xy = 12, ∂f/∂y = x² = 4
// ∂²f/∂x² = 2y = 6, ∂²f/∂x∂y = 2x = 4, ∂²f/∂y² = 0
```

### Jet3\<N, H, T\>

Third-order jet: tracks value, gradient, Hessian, and the full third-order
tensor (stored as a lower-triangular array of size T = N(N+1)(N+2)/6).

```rust
use nolan::jets::{Jet3, hess_size, tens_size};

let x = Jet3::<1, { hess_size(1) }, { tens_size(1) }>::variable(1.0, 0);
let f = x.powi(4);
// f = x⁴, value = 1
// df/dx = 4x³ = 4
// d²f/dx² = 12x² = 12
// d³f/dx³ = 24x = 24
assert_eq!(f.tens[0], 24.0);
```

| Type | Storage (N=6) | Storage (N=9) |
|------|---------------|---------------|
| Jet1 | 56 B | 80 B |
| Jet2 | 224 B | 440 B |
| Jet3 | 672 B | 1,760 B |

### Type Aliases

```rust
type Dual = Jet1<1>;                     // Single-variable first derivative
type HyperDual = Jet2<2, 3>;            // Two-variable second derivatives
type HyperHyperDual = Jet3<2, 3, 4>;    // Two-variable third derivatives
```

## Traits

- **`Differentiable`** — Base trait: `value()`, `constant()`, `variable()`. Implemented for `f64`, `Jet1`, `Jet2`, `Jet3`.
- **`FirstOrder`** — First derivatives: `grad(i)`. Implemented for `Jet1`, `Jet2`, `Jet3`.
- **`SecondOrder`** — Second derivatives: `hess(i, j)`. Implemented for `Jet2`, `Jet3`.
- **`ThirdOrder`** — Third derivatives: `tens(i, j, k)`. Implemented for `Jet3`.
- **`DifferentiableMath`** — Transcendental functions: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `exp`, `ln`, `log`, `sqrt`, `powf`, `powi`, `abs`.
- **`AutoDiff`** — Marker combining all of the above. Use this as a generic bound.

## Generic Programming

Write functions once that work with `f64` (no derivatives), `Jet1` (gradients),
`Jet2` (Hessians), or `Jet3` (third-order tensors):

```rust
use nolan::traits::AutoDiff;

fn gravity<J: AutoDiff>(x: J, y: J, z: J, mu: f64) -> [J; 3] {
    let r = (x * x + y * y + z * z).sqrt();
    let r3 = r.powi(3);
    [-mu * x / r3, -mu * y / r3, -mu * z / r3]
}
```

## Convenience API: `differentiate`

For one-shot derivative computations, the `differentiate` module hides the
Jet seeding + extraction boilerplate:

```rust
use nolan::differentiate::{differentiate1, differentiate2_6, differentiate3_6};

// First derivatives
let (value, grad) = differentiate1([1.5, 2.0], |[x, y]| x * y + x * x);
// value = 5.25, grad = [5.0, 1.5]

// Second derivatives (6-parameter specialization avoids spelling out H)
let (value, grad, hess) =
    differentiate2_6([1.0, 0.5, 0.1, 0.0, 0.0, 0.0], |[x, y, z, _, _, _]| {
        (x * x + y * y + z * z).sqrt()
    });

// Third derivatives
let (value, grad, hess, tens) = differentiate3_6(state, |xs| compute_miss_distance(xs));
```

Six variants: `differentiate1`/`differentiate2`/`differentiate3` for scalar
\\( f: \mathbb{R}^N \to \mathbb{R} \\), plus `_vec` variants for vector-valued
\\( f: \mathbb{R}^N \to \mathbb{R}^M \\) that return the full Jacobian (and
higher-order tensors stacked per output). Specialized `differentiate2_6`,
`differentiate2_9`, `differentiate3_6`, `differentiate3_9` helpers inline the
hessian/tensor sizes for the common 6- and 9-parameter state cases.

The wrapper is essentially overhead-free — the seeding + extraction is in the
same ballpark as the compute itself (~0.4 ns overhead on a 25 ns Jet1 gravity
evaluation, see `benchmark_jets.rs`).

## Linear Algebra

Stack-allocated generic matrix operations for any `N`:

```rust
use nolan::linalg::*;

// Solve Ax = b (Gauss-Jordan with partial pivoting)
let x = mat_solve(&a, &b).unwrap();

// Cholesky decomposition (symmetric positive-definite)
let l = mat_cholesky(&cov).unwrap();  // A = L L^T

// Other operations
let inv = mat_inv(&a).unwrap();
let d2 = mahalanobis_distance_squared(&x, &mu, &cov).unwrap();
let (v, lambda) = mat_eigenvector_max(&a, 100, 1e-12);
let ld = mat_log_det(&a);
let tr = mat_trace(&a);
```

Specialized fast paths for 3x3, 6x6, and 9x9 matrices.

## Optimization

Generic nonlinear least-squares solver (Gauss-Newton / Levenberg-Marquardt):

```rust
use nolan::optimization::*;

let solution = solve_nlls(
    |x: &[f64; 3]| {
        // Return residuals + Jacobian at x
        NLLSEvaluation { residuals, jacobian, cost }
    },
    [0.0; 3],           // initial guess
    &NLLSConfig::default(),
    None,                // optional Bayesian prior
).unwrap();

println!("x = {:?}, cost = {}", solution.x, solution.cost);
```

For stateful problems, implement the `NLLSProblem<N>` trait:

```rust
impl NLLSProblem<6> for MyProblem {
    fn evaluate(&mut self, x: &[f64; 6]) -> NLLSEvaluation<6> {
        // Propagate, compute residuals, extract Jacobian
    }
}
let solution = solve(&mut problem, x0, &config, prior)?;
```

Features: LM adaptive damping, Bayesian prior augmentation, second-order
Hessian correction (`solve2`), formal covariance extraction.

## Version

```rust
println!("{}", nolan::version());
// Tagged release: "1.0.0"
// Development:    "1.0.1-dev+a3f7b2c"
// Dirty:          "1.0.1-dev+a3f7b2c-dirty"
```

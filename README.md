<img src="docs/nolan-icon.png" width="140" alt="nolan">

# nolan
Const-generic hyperdual numbers for automatic differentiation in Rust

<a href="https://github.com/Empyrean-Dynamics/nolan/releases/latest"><img src="https://img.shields.io/github/v/tag/Empyrean-Dynamics/nolan?label=Version&sort=semver&style=flat-square&color=blue" alt="Version"></a>
<a href="https://github.com/Empyrean-Dynamics/nolan/actions/workflows/rust.yml"><img src="https://img.shields.io/github/actions/workflow/status/Empyrean-Dynamics/nolan/rust.yml?label=CI&style=flat-square" alt="CI"></a>
<a href="https://empyrean-dynamics.github.io/nolan/docs/nolan/"><img src="https://img.shields.io/badge/Docs-gh--pages-blue?style=flat-square" alt="Docs"></a>
<a href="https://claude.ai"><img src="https://img.shields.io/badge/Built%20with-Claude%20Code-D97757?logo=anthropic&logoColor=white&style=flat-square" alt="Built with Claude Code"></a>
<br>
<a href="https://www.empyrean-dynamics.com"><img src="https://img.shields.io/badge/Website-empyrean--dynamics.com-1a1a2e?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IndoaXRlIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCI+PGNpcmNsZSBjeD0iMTIiIGN5PSIxMiIgcj0iMTAiLz48bGluZSB4MT0iMiIgeTE9IjEyIiB4Mj0iMjIiIHkyPSIxMiIvPjxwYXRoIGQ9Ik0xMiAyYTE1LjMgMTUuMyAwIDAgMSA0IDEwIDE1LjMgMTUuMyAwIDAgMS00IDEwIDE1LjMgMTUuMyAwIDAgMS00LTEwIDE1LjMgMTUuMyAwIDAgMSA0LTEweiIvPjwvc3ZnPg==&logoColor=white&style=flat-square" alt="Website"></a>
<a href="https://github.com/Empyrean-Dynamics"><img src="https://img.shields.io/badge/GitHub-Empyrean--Dynamics-1a1a2e?logo=github&logoColor=white&style=flat-square" alt="GitHub"></a>

*by [Empyrean Dynamics](https://www.empyrean-dynamics.com)*

---

nolan provides first- and second-order jet types that propagate exact derivatives
through arbitrary computations. It is designed for astrodynamics applications
where derivatives of physical parameters (position, velocity, time) are needed
for orbit determination, covariance propagation, and sensitivity analysis.

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

### Type Aliases

```rust
type Dual = Jet1<1>;              // Single-variable first derivative
type HyperDual = Jet2<2, 3>;     // Two-variable second derivatives
```

## Traits

- **`Differentiable`** — Base trait: `value()`, `constant()`, `variable()`. Implemented for `f64`, `Jet1`, `Jet2`.
- **`FirstOrder`** — First derivatives: `grad(i)`. Implemented for `Jet1`, `Jet2`.
- **`SecondOrder`** — Second derivatives: `hess(i, j)`. Implemented for `Jet2`.
- **`DifferentiableMath`** — Transcendental functions: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `sinh`, `cosh`, `tanh`, `exp`, `ln`, `log`, `sqrt`, `powf`, `powi`, `abs`.
- **`AutoDiff`** — Marker combining all of the above. Use this as a generic bound.

## Generic Programming

Write functions once that work with `f64` (no derivatives), `Jet1` (gradients),
or `Jet2` (Hessians):

```rust
use nolan::traits::AutoDiff;

fn gravity<J: AutoDiff>(x: J, y: J, z: J, mu: f64) -> [J; 3] {
    let r = (x * x + y * y + z * z).sqrt();
    let r3 = r.powi(3);
    [-mu * x / r3, -mu * y / r3, -mu * z / r3]
}
```

## Version

```rust
println!("{}", nolan::version());
// Tagged release: "1.0.0"
// Development:    "1.0.1-dev+a3f7b2c"
// Dirty:          "1.0.1-dev+a3f7b2c-dirty"
```

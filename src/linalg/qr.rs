//! Sequential QR least squares by Givens rotations.
//!
//! Solves \\(\min_h \lVert A h - b \rVert_2\\) for a tall \\(M \times N\\)
//! system **without ever forming \\(A^\top A\\)**.
//!
//! # Why this exists
//!
//! Forming the normal equations squares the condition number:
//! \\(\kappa(A^\top A) = \kappa(A)^2\\). For a least-squares problem whose
//! rows carry weights spanning many orders of magnitude — a joint orbit fit
//! mixing radar delay at microsecond \\(\sigma\\) with optical astrometry at
//! arcsecond \\(\sigma\\) differ by five orders in \\(\lvert J\rvert/\sigma\\)
//! — that squaring is the difference between a solvable problem and an
//! unsolvable one.
//!
//! The encouraging part, and the reason this is worth doing rather than
//! reweighting the data: the *weighted problem's* sensitivity is bounded
//! independently of the weight spread (Van Loan 1985). High dynamic range is
//! not intrinsically ill-conditioned — squaring is what destroys it. So the
//! fix belongs in the solver, not in the weights.
//!
//! # Why Givens and not Householder
//!
//! Rows are consumed one at a time into a running \\(N \times N\\)
//! \\(R\\) and an \\(N\\)-vector \\(Q^\top b\\); the full \\(M \times N\\)
//! matrix is never stored, and \\(M\\) never appears in the working set. That
//! also makes appending rows free, which is what the Levenberg-Marquardt
//! damping term needs: \\(\sqrt{\mu} D\\) enters as \\(N\\) extra rows rather
//! than as \\(\mu D^2\\) added to a squared matrix (Moré 1978). This is the
//! measurement-update form of Bierman's square-root filtering (1977).
//!
//! # References
//!
//! Lawson & Hanson 1974; Björck 1996 (weighted / "stiff" least squares);
//! Van Loan 1985; Bierman 1977; Moré 1978. See `lang/REFERENCES.md`.

/// Running upper-triangular factor \\(R\\) and transformed right-hand side
/// \\(Q^\top b\\), updated one row at a time.
///
/// Deterministic by construction: rotations are applied in a fixed column
/// order, and no row reordering or pivoting is performed, so a given row
/// sequence always produces bit-identical output.
#[derive(Debug, Clone)]
pub struct QrAccumulator<const N: usize> {
    r: [[f64; N]; N],
    qtb: [f64; N],
}

impl<const N: usize> Default for QrAccumulator<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> QrAccumulator<N> {
    /// An empty factorization (zero rows absorbed).
    pub fn new() -> Self {
        Self {
            r: [[0.0; N]; N],
            qtb: [0.0; N],
        }
    }

    /// Absorb one row \\((a, b)\\) of the system \\(A h = b\\).
    ///
    /// Rotating the row into `R` costs \\(O(N^2)\\) and touches no storage
    /// proportional to the number of rows.
    pub fn push_row(&mut self, a: &[f64; N], b: f64) {
        let mut row = *a;
        let mut rhs = b;

        for k in 0..N {
            if row[k] == 0.0 {
                continue;
            }
            // Givens rotation zeroing row[k] against R[k][k], via the
            // hypot form so an intermediate square never overflows.
            let (rkk, ak) = (self.r[k][k], row[k]);
            let norm = rkk.hypot(ak);
            if norm == 0.0 {
                continue;
            }
            let c = rkk / norm;
            let s = ak / norm;

            self.r[k][k] = norm;
            row[k] = 0.0;
            // Two arrays rotate together against one column index; the
            // iterator form would need a zip and read worse.
            #[allow(clippy::needless_range_loop)]
            for j in (k + 1)..N {
                let (rkj, aj) = (self.r[k][j], row[j]);
                self.r[k][j] = c * rkj + s * aj;
                row[j] = c * aj - s * rkj;
            }
            let (zk, rb) = (self.qtb[k], rhs);
            self.qtb[k] = c * zk + s * rb;
            rhs = c * rb - s * zk;
        }
    }

    /// Absorb \\(\sqrt{\mu}\\) times the identity as \\(N\\) extra rows with
    /// zero right-hand side — the Levenberg-Marquardt damping term in
    /// square-root form.
    ///
    /// Equivalent to adding \\(\mu I\\) to \\(A^\top A\\), but reached
    /// without squaring anything, so \\(\mu\\) can be raised far beyond the
    /// point where \\(A^\top A + \mu I\\) would overflow or lose all
    /// significance.
    pub fn push_damping(&mut self, mu: f64) {
        if mu <= 0.0 {
            return;
        }
        let root_mu = mu.sqrt();
        for i in 0..N {
            let mut row = [0.0_f64; N];
            row[i] = root_mu;
            self.push_row(&row, 0.0);
        }
    }

    /// Solve \\(R h = Q^\top b\\) by back substitution.
    ///
    /// `None` when \\(R\\) is singular or non-finite — a rank-deficient or
    /// poisoned system, reported rather than papered over with a
    /// pseudo-inverse the caller did not ask for.
    pub fn solve(&self) -> Option<[f64; N]> {
        let mut h = [0.0_f64; N];
        for i in (0..N).rev() {
            let mut acc = self.qtb[i];
            #[allow(clippy::needless_range_loop)]
            for j in (i + 1)..N {
                acc -= self.r[i][j] * h[j];
            }
            let rii = self.r[i][i];
            if rii == 0.0 || !rii.is_finite() {
                return None;
            }
            let v = acc / rii;
            if !v.is_finite() {
                return None;
            }
            h[i] = v;
        }
        Some(h)
    }

    /// The upper-triangular factor. \\(R^\top R = A^\top A\\), so this is a
    /// square root of the normal matrix — the quantity a square-root
    /// information filter carries in place of a covariance.
    pub fn r(&self) -> &[[f64; N]; N] {
        &self.r
    }
}

/// Solve \\(\min_h \lVert A h - b \rVert_2\\) for the given rows.
///
/// `rows` and `rhs` must have equal length. `None` on a rank-deficient or
/// non-finite system.
pub fn solve_least_squares<const N: usize>(rows: &[[f64; N]], rhs: &[f64]) -> Option<[f64; N]> {
    if rows.len() != rhs.len() {
        return None;
    }
    let mut qr = QrAccumulator::<N>::new();
    for (a, b) in rows.iter().zip(rhs) {
        qr.push_row(a, *b);
    }
    qr.solve()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Agreement with the normal-equations answer on a well-conditioned
    /// problem: the two are the same solution, reached two ways.
    #[test]
    fn matches_normal_equations_when_well_conditioned() {
        let rows = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
        let rhs = [1.0, 2.0, 3.05];
        let h = solve_least_squares::<2>(&rows, &rhs).expect("solvable");

        // Normal equations by hand: (A^T A) h = A^T b.
        // A^T A = [[2,1],[1,2]], A^T b = [1+3.05, 2+3.05] = [4.05, 5.05]
        // inv = 1/3 [[2,-1],[-1,2]]
        let want = [(2.0 * 4.05 - 5.05) / 3.0, (2.0 * 5.05 - 4.05) / 3.0];
        for i in 0..2 {
            assert!(
                (h[i] - want[i]).abs() < 1e-12,
                "component {i}: QR {} vs normal equations {}",
                h[i],
                want[i]
            );
        }
    }

    /// THE POINT OF THE MODULE. Rows whose scales differ by 1e8 give a
    /// normal matrix with condition number ~1e16 — at f64's limit — while
    /// the QR sees only ~1e8 and returns the right answer.
    ///
    /// The system is exactly solvable, so the true answer is known
    /// independently of either method.
    #[test]
    fn survives_row_weights_that_would_square_out_of_range() {
        const BIG: f64 = 1.0e8;
        // Exact solution [2, -3]: a stiff row and two ordinary ones.
        let rows = [[BIG, BIG], [1.0, 0.0], [0.0, 1.0]];
        let rhs = [BIG * (2.0 - 3.0), 2.0, -3.0];

        let h = solve_least_squares::<2>(&rows, &rhs).expect("solvable");
        assert!(
            (h[0] - 2.0).abs() < 1e-6 && (h[1] + 3.0).abs() < 1e-6,
            "stiff system: got {h:?}, want [2, -3]"
        );
    }

    /// Damping enters as extra rows, so mu far past the point where
    /// `A^T A + mu I` would lose all significance still yields a finite,
    /// shrinking step rather than an overflow.
    #[test]
    fn damping_stays_finite_at_extreme_mu() {
        let rows = [[1.0, 0.0], [0.0, 1.0]];
        let rhs = [1.0, 1.0];

        let mut prev = f64::INFINITY;
        for mu in [0.0_f64, 1.0, 1e10, 1e30, 1e60] {
            let mut qr = QrAccumulator::<2>::new();
            for (a, b) in rows.iter().zip(&rhs) {
                qr.push_row(a, *b);
            }
            qr.push_damping(mu);
            let h = qr.solve().expect("damped system is always full rank");
            let norm = (h[0] * h[0] + h[1] * h[1]).sqrt();
            assert!(
                norm.is_finite(),
                "mu {mu:.0e} produced a non-finite step {h:?}"
            );
            assert!(
                norm <= prev + 1e-12,
                "step norm grew with damping: mu {mu:.0e} gave {norm:.3e} after {prev:.3e}"
            );
            prev = norm;
        }
        assert!(
            prev < 1e-20,
            "at mu = 1e60 the step should be crushed to nothing, got {prev:.3e}"
        );
    }

    /// A rank-deficient system is reported, not silently pseudo-inverted.
    #[test]
    fn rank_deficient_system_is_refused() {
        let rows = [[1.0, 2.0], [2.0, 4.0]];
        let rhs = [1.0, 2.0];
        assert!(
            solve_least_squares::<2>(&rows, &rhs).is_none(),
            "a singular system must be refused rather than answered"
        );
    }

    /// R is a genuine square root of the normal matrix.
    #[test]
    fn r_transpose_r_reproduces_the_normal_matrix() {
        let rows = [[1.0, 2.0], [3.0, -1.0], [0.5, 0.25]];
        let rhs = [0.0, 0.0, 0.0];
        let mut qr = QrAccumulator::<2>::new();
        for (a, b) in rows.iter().zip(&rhs) {
            qr.push_row(a, *b);
        }
        let r = qr.r();

        for i in 0..2 {
            for j in 0..2 {
                let ata: f64 = rows.iter().map(|row| row[i] * row[j]).sum();
                let rtr: f64 = (0..2).map(|k| r[k][i] * r[k][j]).sum();
                assert!(
                    (ata - rtr).abs() < 1e-12,
                    "R^T R [{i}][{j}] = {rtr} != A^T A = {ata}"
                );
            }
        }
    }
}

//! Univariate Gaussian splitting library.
//!
//! GENERATED FILE. Do not edit by hand. To regenerate, from the
//! repository (this generator is not part of the published crate):
//!
//! ```text
//! cargo run --release --example generate_split_library > /tmp/split.rs \
//!     && mv /tmp/split.rs src/statistics/split_library.rs \
//!     && cargo fmt
//! ```
//!
//! Both halves matter. Redirecting straight onto this file truncates it
//! before cargo runs, and the crate then fails to compile because the
//! module it is being asked to regenerate no longer exists, so the
//! output goes to a temporary first. And the `cargo fmt` is part of the
//! command rather than an afterthought: the generator emits one array
//! element per line and lets rustfmt decide what fits, so its raw output
//! is not the committed file and diffing the two without it shows
//! spurious changes. See `tools/split_library_fit.rs` for what is
//! solved.
//!
//! Each entry approximates the standard normal by a mixture of `k`
//! Gaussians of a common width, with weights and means symmetric about
//! the origin, chosen to minimise the \\(L^2\\) distance between the two
//! densities subject to the mixture carrying zero mean and unit
//! variance.
//!
//! The width is not optimised alongside them. With it free the problem
//! has the trivial solution \\(\alpha_0 = 1\\), \\(\sigma = 1\\),
//! every other weight zero: one component equal to the parent, at
//! \\(L^2 = 0\\) and no split at all. Narrow components are not the
//! escape — as \\(\sigma \to 0\\) the component approaches a Dirac
//! delta and the distance diverges, measuring 2.31 at
//! \\(\sigma = 0.1\\) and 281.6 at \\(\sigma = 0.001\\) — so the
//! optimum runs the other way, toward no deflation. Vittaldev & Russell
//! therefore fix \\(\sigma\\) by a rule in `k`, and this table uses
//! their second, \\(\sigma^2 = (1/k)^{3/4}\\), corrected as below.
//! Their first, \\(\sigma^2 = 1/k\\), deflates harder but reproduces
//! the parent's tail worse — by 1.5 orders of magnitude at
//! \\(5\sigma\\) for `k` = 3, 4.7 by `k` = 7 and 7.3 by `k` = 15,
//! measured on the mixture each rule would actually ship. Their
//! third, \\(\sigma^2 = (1/k)^{1/2}\\), reproduces the tail best but
//! drives the \\(L^2\\) distance to \\(10^{-13}\\) by `k` = 12,
//! where the objective — assembled as a difference against a leading
//! term of \\(1/(2\sqrt{\pi})\\) — stops determining the fit in
//! double precision.
//!
//! # The unit-variance constraint, and the width it implies
//!
//! Vittaldev & Russell constrain only that the weights sum to one, so
//! their mixtures carry slightly less variance than the parent — 0.951
//! of it at `k` = 3. A split that sheds a fixed fraction of the variance
//! at every level compounds that loss under recursion, and
//! [`split_gaussian`](super::multivariate::split_gaussian) must
//! reproduce the covariance it was handed, so the mixture here is
//! constrained to unit variance as well.
//!
//! That constraint also fixes the width, in two stages. A mixture cannot
//! carry unit variance at the rule's width without moving its means
//! outward, and the outward move is a dilation by
//! \\(1/\sqrt{v}\\) in the variance \\(v\\) of the unconstrained
//! optimum. So stage one solves the unconstrained problem at the rule's
//! width and reads off \\(v\\); the shipped width is the rule's divided
//! by \\(\sqrt{v}\\). Stage two then solves for the weights and means
//! that are optimal AT that width under the variance constraint.
//!
//! Stopping after the dilation would be the cheaper thing to do, and it
//! is what an earlier draft of this table did, but a dilated optimum is
//! not an optimum: measured at its own width it is 38.5% worse in
//! \\(L^2\\) at `k` = 3 and 154% worse at `k` = 7. Solving at the
//! width instead is worth between 1.0 and 1.8 times the parent's mass
//! beyond \\(5\sigma\\), in the library's favour, at no run-time cost.
//!
//! # References
//!
//! - Vittaldev, V., & Russell, R. P. (2016). *Multidirectional Gaussian
//!   mixture models for nonlinear uncertainty propagation.* Computer
//!   Modeling in Engineering & Sciences 111(1): 83–117.
//!   [doi:10.3970/cmes.2016.111.083](https://doi.org/10.3970/cmes.2016.111.083)
//!   (the optimisation, its constraints, and the \\(\sigma\\) rules)
//! - Vittaldev, V., & Russell, R. P. (2016). *Space object collision
//!   probability using multidirectional Gaussian mixture models.*
//!   Journal of Guidance, Control, and Dynamics 39(9): 2163–2169.
//!   [doi:10.2514/1.G001610](https://doi.org/10.2514/1.G001610)
//!   (the same library applied to a collision integral)
//! - DeMars, K. J., Bishop, R. H., & Jah, M. K. (2013). *Entropy-based
//!   approach for uncertainty propagation of nonlinear dynamical
//!   systems.* Journal of Guidance, Control, and Dynamics 36(4):
//!   1047–1057.
//!   [doi:10.2514/1.58987](https://doi.org/10.2514/1.58987)
//!   (the \\(L^2\\) cost in closed form, and libraries for \\(k \le 5\\))

/// One entry of the univariate splitting library: a mixture of
/// equally-wide Gaussians approximating the standard normal.
///
/// The weights and means minimise the \\(L^2\\) distance to the
/// standard normal among symmetric mixtures of this width that carry
/// zero mean and unit variance. Because the variance is one, and not
/// merely close to it,
/// [`split_gaussian`](super::multivariate::split_gaussian) reproduces
/// the parent's covariance exactly rather than to within a deficit that
/// would compound under recursion.
#[derive(Clone, Copy, Debug)]
pub struct UnivariateSplit {
    /// Component weights, ordered by increasing mean. Positive, summing
    /// to one, and decreasing away from the centre.
    pub weights: &'static [f64],
    /// Component means in units of the parent standard deviation,
    /// increasing and antisymmetric about zero.
    pub means: &'static [f64],
    /// The standard deviation shared by every component, in units of the
    /// parent standard deviation. Always less than one: this is the
    /// factor by which a split deflates the spread along its direction.
    pub sigma: f64,
}

/// Univariate split entries for `k` = 2 through 15, indexed by
/// `k - 2`.
static LIBRARY: [UnivariateSplit; 14] = [
    // K = 2
    UnivariateSplit {
        weights: &[5e-1, 5e-1],
        means: &[-5.924566950883456e-1, 5.924566950883456e-1],
        sigma: 8.056022991805541e-1,
    },
    // K = 3
    UnivariateSplit {
        weights: &[
            2.1148105118924626e-1,
            5.770378976215075e-1,
            2.1148105118924626e-1,
        ],
        means: &[-1.128427913490078e0, 0e0, 1.128427913490078e0],
        sigma: 6.792800562031741e-1,
    },
    // K = 4
    UnivariateSplit {
        weights: &[
            9.774289733860012e-2,
            4.022571026613999e-1,
            4.022571026613999e-1,
            9.774289733860012e-2,
        ],
        means: &[
            -1.523744584192022e0,
            -4.762536843600862e-1,
            4.762536843600862e-1,
            1.523744584192022e0,
        ],
        sigma: 6.030286898148022e-1,
    },
    // K = 5
    UnivariateSplit {
        weights: &[
            4.827179190625536e-2,
            2.49619973547412e-1,
            4.042164690926653e-1,
            2.49619973547412e-1,
            4.827179190625536e-2,
        ],
        means: &[
            -1.8390689395552644e0,
            -8.604153052026345e-1,
            0e0,
            8.604153052026345e-1,
            1.8390689395552644e0,
        ],
        sigma: 5.512514634364102e-1,
    },
    // K = 6
    UnivariateSplit {
        weights: &[
            2.5027032362934482e-2,
            1.4996505210629618e-1,
            3.2500791553076935e-1,
            3.2500791553076935e-1,
            1.4996505210629618e-1,
            2.5027032362934482e-2,
        ],
        means: &[
            -2.1041072597434916e0,
            -1.1817745801886683e0,
            -3.8479762858851696e-1,
            3.8479762858851696e-1,
            1.1817745801886683e0,
            2.1041072597434916e0,
        ],
        sigma: 5.130986332875812e-1,
    },
    // K = 7
    UnivariateSplit {
        weights: &[
            1.3473830246686011e-2,
            8.956735843118202e-2,
            2.3657901702873985e-1,
            3.207595885867842e-1,
            2.3657901702873985e-1,
            8.956735843118202e-2,
            1.3473830246686011e-2,
        ],
        means: &[
            -2.334455089218924e0,
            -1.4586579137518876e0,
            -7.097614374332678e-1,
            0e0,
            7.097614374332678e-1,
            1.4586579137518876e0,
            2.334455089218924e0,
        ],
        sigma: 4.8336660766755946e-1,
    },
    // K = 8
    UnivariateSplit {
        weights: &[
            7.47916153197379e-3,
            5.371714659210443e-2,
            1.6388504587258299e-1,
            2.749186460033388e-1,
            2.749186460033388e-1,
            1.6388504587258299e-1,
            5.371714659210443e-2,
            7.47916153197379e-3,
        ],
        means: &[
            -2.5392290789945875e0,
            -1.7025523727629626e0,
            -9.92058805320598e-1,
            -3.265478549464021e-1,
            3.265478549464021e-1,
            9.92058805320598e-1,
            1.7025523727629626e0,
            2.5392290789945875e0,
        ],
        sigma: 4.592594696550158e-1,
    },
    // K = 9
    UnivariateSplit {
        weights: &[
            4.259368334001569e-3,
            3.2482918141023755e-2,
            1.1064245569135389e-1,
            2.1748974576715682e-1,
            2.702510241329278e-1,
            2.1748974576715682e-1,
            1.1064245569135389e-1,
            3.2482918141023755e-2,
            4.259368334001569e-3,
        ],
        means: &[
            -2.7242392354040854e0,
            -1.9210077918727382e0,
            -1.2422630937802974e0,
            -6.115996364086348e-1,
            0e0,
            6.115996364086348e-1,
            1.2422630937802974e0,
            1.9210077918727382e0,
            2.7242392354040854e0,
        ],
        sigma: 4.3913573787823457e-1,
    },
    // K = 10
    UnivariateSplit {
        weights: &[
            2.4796112832677158e-3,
            1.9837172820827767e-2,
            7.372111056366933e-2,
            1.6391822748973e-1,
            2.400438778425052e-1,
            2.400438778425052e-1,
            1.6391822748973e-1,
            7.372111056366933e-2,
            1.9837172820827767e-2,
            2.4796112832677158e-3,
        ],
        means: &[
            -2.8934395938584236e0,
            -2.119227408756427e0,
            -1.467389590146299e0,
            -8.652885979859235e-1,
            -2.861491163456929e-1,
            2.861491163456929e-1,
            8.652885979859235e-1,
            1.467389590146299e0,
            2.119227408756427e0,
            2.8934395938584236e0,
        ],
        sigma: 4.2196242790118543e-1,
    },
    // K = 11
    UnivariateSplit {
        weights: &[
            1.471420158265756e-3,
            1.2240425289027341e-2,
            4.8825266364636875e-2,
            1.198414974303585e-1,
            1.9971511609060083e-1,
            2.3581254933422147e-1,
            1.9971511609060083e-1,
            1.198414974303585e-1,
            4.8825266364636875e-2,
            1.2240425289027341e-2,
            1.471420158265756e-3,
        ],
        means: &[
            -3.049667430033239e0,
            -2.3009528004231616e0,
            -1.6723502880719883e0,
            -1.094321488382714e0,
            -5.41664168050244e-1,
            0e0,
            5.41664168050244e-1,
            1.094321488382714e0,
            1.6723502880719883e0,
            2.3009528004231616e0,
            3.049667430033239e0,
        ],
        sigma: 4.070525752957795e-1,
    },
    // K = 12
    UnivariateSplit {
        weights: &[
            8.881202630111685e-4,
            7.631444359228776e-3,
            3.227947043294205e-2,
            8.593140352259405e-2,
            1.5901754275473012e-1,
            2.1425201866749383e-1,
            2.1425201866749383e-1,
            1.5901754275473012e-1,
            8.593140352259405e-2,
            3.227947043294205e-2,
            7.631444359228776e-3,
            8.881202630111685e-4,
        ],
        means: &[
            -3.195003053291169e0,
            -2.4689387152716726e0,
            -1.8607056188714444e0,
            -1.3033931574163766e0,
            -7.729873796668987e-1,
            -2.5625375531817307e-1,
            2.5625375531817307e-1,
            7.729873796668987e-1,
            1.3033931574163766e0,
            1.8607056188714444e0,
            2.4689387152716726e0,
            3.195003053291169e0,
        ],
        sigma: 3.9392847434455225e-1,
    },
    // K = 13
    UnivariateSplit {
        weights: &[
            5.441933226246798e-4,
            4.805671571868656e-3,
            2.1357375050178963e-2,
            6.0856164888452254e-2,
            1.2282847707769283e-1,
            1.843332120561161e-1,
            2.10549812066133e-1,
            1.843332120561161e-1,
            1.2282847707769283e-1,
            6.0856164888452254e-2,
            2.1357375050178963e-2,
            4.805671571868656e-3,
            5.441933226246798e-4,
        ],
        means: &[
            -3.3310900886534567e0,
            -2.6253271920751304e0,
            -2.035161757435209e0,
            -1.4959602325872816e0,
            -9.846549598392709e-1,
            -4.888226962569674e-1,
            0e0,
            4.888226962569674e-1,
            9.846549598392709e-1,
            1.4959602325872816e0,
            2.035161757435209e0,
            2.6253271920751304e0,
            3.3310900886534567e0,
        ],
        sigma: 3.822456090403123e-1,
    },
    // K = 14
    UnivariateSplit {
        weights: &[
            3.380141997885757e-4,
            3.0553201454316453e-3,
            1.4164608626087767e-2,
            4.2764048908819255e-2,
            9.28718765571429e-2,
            1.5249788472154377e-1,
            1.9430824684118614e-1,
            1.9430824684118614e-1,
            1.5249788472154377e-1,
            9.28718765571429e-2,
            4.2764048908819255e-2,
            1.4164608626087767e-2,
            3.0553201454316453e-3,
            3.380141997885757e-4,
        ],
        means: &[
            -3.459188999240793e0,
            -2.771769663735716e0,
            -2.1977918438503528e0,
            -1.6746279506734372e0,
            -1.179993966523547e0,
            -7.020772008201291e-1,
            -2.330865833704965e-1,
            2.330865833704965e-1,
            7.020772008201291e-1,
            1.179993966523547e0,
            1.6746279506734372e0,
            2.1977918438503528e0,
            2.771769663735716e0,
            3.459188999240793e0,
        ],
        sigma: 3.717478558152139e-1,
    },
    // K = 15
    UnivariateSplit {
        weights: &[
            2.1257231252390233e-4,
            1.9603680467700432e-3,
            9.42636799106789e-3,
            2.9912415058702974e-2,
            6.916523136977962e-2,
            1.2258363634676506e-1,
            1.71197927477646e-1,
            1.9108296279348902e-1,
            1.71197927477646e-1,
            1.2258363634676506e-1,
            6.916523136977962e-2,
            2.9912415058702974e-2,
            9.42636799106789e-3,
            1.9603680467700432e-3,
            2.1257231252390233e-4,
        ],
        means: &[
            -3.5802927175699226e0,
            -2.909563979991977e0,
            -2.3502154276199354e0,
            -1.8414068554498033e0,
            -1.3615214011111707e0,
            -8.992274098378706e-1,
            -4.472216988975605e-1,
            0e0,
            4.472216988975605e-1,
            8.992274098378706e-1,
            1.3615214011111707e0,
            1.8414068554498033e0,
            2.3502154276199354e0,
            2.909563979991977e0,
            3.5802927175699226e0,
        ],
        sigma: 3.622399826257431e-1,
    },
];

/// The smallest component count the library tabulates.
pub const MIN_SPLIT_COMPONENTS: usize = 2;

/// The largest component count the library tabulates.
///
/// Both a usefulness limit and, just above it, a numerical one. A caller
/// pays one propagation per component, so 15 already costs five times
/// the three-component split, and the return has flattened: across
/// `k` = 13 to 15 the mixture's mass beyond \\(4\sigma\\) climbs from
/// 0.71 to 0.91 of the parent's while its mass beyond \\(5\sigma\\)
/// climbs only from 0.012 to 0.033. No `k` under this rule reproduces
/// the deep tail, so raising `k` is not the lever that would.
///
/// Measured, the solve stays determined one step past the table, at
/// `k` = 16, and stops there: from `k` = 17 to 25 every start stalls at
/// a scaled gradient between \\(3\times10^{-2}\\) and
/// \\(8\times10^{-2}\\), which the fitter refuses rather than
/// tabulates. The unit-variance parameterisation eliminates the
/// innermost mean through a square root, and the further the weights
/// spread the worse that recovery is conditioned.
pub const MAX_SPLIT_COMPONENTS: usize = 15;

/// The library entry for `k` components, or `None` outside
/// [`MIN_SPLIT_COMPONENTS`]`..=`[`MAX_SPLIT_COMPONENTS`].
pub fn univariate_split(k: usize) -> Option<&'static UnivariateSplit> {
    if !(MIN_SPLIT_COMPONENTS..=MAX_SPLIT_COMPONENTS).contains(&k) {
        return None;
    }
    Some(&LIBRARY[k - MIN_SPLIT_COMPONENTS])
}

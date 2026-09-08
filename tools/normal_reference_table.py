#!/usr/bin/env python3
"""Emit correctly rounded standard-normal references as a Rust table.

The crate's `normal_cdf`, `normal_sf` and `normal_pdf` claim a RELATIVE
accuracy across the whole range where the result is representable as a
double, which runs to roughly -37.5 sigma before the lower tail goes
subnormal. A claim like that cannot be checked against another double
precision routine, because the routine under test and the routine it is
checked against share the same error floor. So the references are
computed in arbitrary precision with mpmath and rounded once, at the end,
to the nearest double.

Values emitted per grid point:

    x    the argument, an exactly representable double
    cdf  Phi(x) = erfc(-x / sqrt(2)) / 2, correctly rounded
    pdf  phi(x) = exp(-x^2 / 2) / sqrt(2 pi), correctly rounded

A second table carries correctly rounded Phi(hi) - Phi(lo) over brackets
chosen to exercise the three regimes the difference has to survive: both
arguments on one side of the origin and far apart, both on one side and
close enough to cancel, and straddling the origin.

The survival function needs no column of its own: Q(x) = Phi(-x) exactly,
and the grid is symmetric, so the test reads Q(x) off the row at -x.

Regenerate with, from the repository root:

    uv run --with mpmath==1.3.0 python tools/normal_reference_table.py

which writes tests/data/normal_reference_table.rs. Pass an explicit
output path as the first argument to write somewhere else. The pinned
mpmath version keeps the table byte-reproducible; any 1.x release gives
the same doubles, since 60 decimal digits is far more than the 17 the
rounding needs.

mpmath: Johansson, F. et al. (2023). mpmath: a Python library for
arbitrary-precision floating-point arithmetic, version 1.3.0.
https://mpmath.org
"""

import sys
from pathlib import Path

from mpmath import mp, erfc, exp, sqrt, pi, mpf

# 60 decimal digits: the smallest value in the table is near 1e-316, and
# 60 digits leaves ~40 digits of headroom over the 17 that round-tripping
# a double requires, at every point on the grid.
mp.dps = 60

# Anchors the accuracy claim is quoted at, plus the seams of the
# implementation and of the approximation it replaces. 1.75 is where the
# series hands over to the continued fraction and is where the whole
# function's worst error lives, so the grid is dense on both sides of it;
# 2.0 is a seam of the DENSITY, where its exponent starts being split;
# 8.0 is where the previous Abramowitz and Stegun form hard-clamped to 0
# and 1.
ANCHORS = [
    0.0,
    0.5,
    1.0,
    2.0,
    3.0,
    4.0,
    5.0,
    6.0,
    7.0,
    8.0,
    10.0,
    15.0,
    20.0,
    30.0,
    37.0,
    # Straddle the series/continued-fraction seam of the CDF, where the
    # series pays for its one subtraction and the error is largest. The
    # worst argument in the whole function is 1.7465700000009603.
    1.6875,
    1.71875,
    1.73,
    1.7375,
    1.74,
    1.7434,
    1.74657,
    1.7465700000009603,
    1.749,
    1.75,
    1.751,
    1.76,
    1.78,
    1.8125,
    # Straddle the seam of the DENSITY, at 2.0.
    1.9375,
    1.96875,
    1.99,
    1.999,
    2.001,
    2.01,
    2.03125,
    2.0625,
    # Straddle the clamp of the replaced approximation.
    7.9,
    7.99,
    8.01,
    8.1,
    # The last argument whose lower tail is still a normal double, and
    # the gradual-underflow band beyond it, down to the last argument
    # whose tail is a representable nonzero subnormal (38.4854...).
    37.5,
    37.6,
    37.7,
    37.8,
    37.9,
    38.0,
    38.1,
    38.2,
    38.3,
    38.4,
    38.45,
]


# Brackets for the difference table. Each is (hi, lo) with hi >= lo, and
# they are grouped by which regime of `normal_cdf_difference` they land
# in, including the narrow same-tail brackets the routine cannot rescue.
DIFFERENCE_BRACKETS = [
    # Straddling the origin.
    (1.0, -1.0),
    (0.5, -0.5),
    (3.0, -3.0),
    (0.001, -0.001),
    (0.1, -5.0),
    (5.0, -0.1),
    (1.7465700000009603, -1.7465700000009603),
    # Both above it, wide enough that the larger tail dominates.
    (2.0, 1.0),
    (8.0, 4.0),
    (20.0, 6.0),
    (37.0, 30.0),
    # Both below it.
    (-1.0, -2.0),
    (-4.44, -5.95),
    (-30.0, -37.0),
    # One endpoint exactly at the origin. These are the brackets a
    # same-side arm would ruin, because the tail at zero is 1/2 and
    # subtracting it from a tail just beside it keeps nothing.
    (1e-09, 0.0),
    (1e-06, 0.0),
    (0.001, 0.0),
    (2.0, 0.0),
    (0.0, -1e-09),
    (0.0, -0.001),
    (0.0, -2.0),
    # Narrow brackets deep in one tail, where the two tails cancel and
    # no arrangement of them recovers the lost figures.
    (6.001, 6.0),
    (6.0, 5.999),
    (20.001, 20.0),
    (-6.0, -6.001),
    # Narrow brackets inside the SERIES band, where the same happens for
    # the same reason: the loss follows the width, not the branch.
    (1.001, 1.0),
    (1.000000001, 1.0),
    (0.501, 0.5),
    (1.500000001, 1.5),
]


def grid():
    """The symmetric grid, ascending, with no duplicates.

    Every point is built as a Python float first and only then handed to
    mpmath, for two reasons. The reference must be evaluated at the very
    double the table emits, not at the decimal or rational that names it;
    and two different rationals can round to one double, so deduplicating
    anywhere but on the double itself leaves a repeated row.
    """
    xs = set()

    # Dense through the bulk, where the series runs.
    for i in range(0, 21):
        xs.add(i / 8)

    # Through the near tail, where the continued fraction is deepest.
    for i in range(10, 33):
        xs.add(i / 4)

    # Out to the underflow edge, where the exponent split earns its keep.
    for i in range(16, 77):
        xs.add(i / 2)

    # A dense band around the handover at 1.75. The eighth-spaced grid
    # above has nothing between 1.625 and 1.75, which is the gap the
    # function's worst argument sits in.
    for i in range(680, 821):
        xs.add(i / 400)

    xs.update(ANCHORS)

    positive = sorted(x for x in xs if x > 0.0)
    return [mpf(-x) for x in reversed(positive)] + [mpf(0.0)] + [mpf(x) for x in positive]


def normal_cdf(x):
    return erfc(-x / sqrt(2)) / 2


def normal_pdf(x):
    return exp(-x * x / 2) / sqrt(2 * pi)


def rust_f64(value):
    """A double literal that round-trips to `value`'s nearest double."""
    text = repr(float(value))
    if text in ("inf", "-inf", "nan"):
        raise ValueError(f"{value} is not finite")
    # Rust wants a decimal point or an exponent on every f64 literal.
    if "." not in text and "e" not in text:
        text += ".0"
    return text


def main():
    out = (
        Path(sys.argv[1])
        if len(sys.argv) > 1
        else Path(__file__).resolve().parent.parent / "tests" / "data" / "normal_reference_table.rs"
    )

    points = grid()
    rows = []
    for x in points:
        rows.append(
            "    ({}, {}, {}),".format(
                rust_f64(x), rust_f64(normal_cdf(x)), rust_f64(normal_pdf(x))
            )
        )

    difference_rows = []
    # 400 digits, not 60, and only for this table. Phi(37) is 1 minus
    # 5.7e-300, and at 60 digits that IS one: the difference Phi(37) -
    # Phi(30) then evaluates to exactly zero, losing the whole answer to
    # cancellation inside the reference itself. 400 digits leaves room
    # for the smallest tail on the list with three hundred to spare, and
    # keeps the reference a direct evaluation of Phi(hi) - Phi(lo)
    # rather than a rearrangement that would share the structure of the
    # routine under test.
    with mp.workdps(400):
        for hi, lo in DIFFERENCE_BRACKETS:
            exact = normal_cdf(mpf(hi)) - normal_cdf(mpf(lo))
            difference_rows.append(
                "    ({}, {}, {}),".format(rust_f64(hi), rust_f64(lo), rust_f64(exact))
            )

    difference_header = f"""

/// `(hi, lo, Phi(hi) - Phi(lo))`, evaluated at 400 decimal digits and
/// rounded once. The extra digits are not decoration: at 60 the
/// reference for a bracket deep in one tail cancels to zero.
pub const NORMAL_DIFFERENCE_REFERENCES: [(f64, f64, f64); {len(difference_rows)}] = [
"""

    header = f"""// Generated by tools/normal_reference_table.py — do not edit by hand.
//
// Correctly rounded standard-normal references: for each argument `x`,
// the doubles nearest to
//
//   Phi(x) = erfc(-x / sqrt(2)) / 2   and   phi(x) = exp(-x^2 / 2) / sqrt(2 pi)
//
// evaluated at 60 decimal digits with mpmath 1.3.0 and rounded once, at
// the end, to nearest. The survival function is read off the mirrored
// row, since Q(x) = Phi(-x) exactly and the grid is symmetric.
//
// Regenerate from the repository root with
//
//   uv run --with mpmath==1.3.0 python tools/normal_reference_table.py
//
// mpmath: Johansson, F. et al. (2023). mpmath: a Python library for
// arbitrary-precision floating-point arithmetic, version 1.3.0.
// <https://mpmath.org>

/// `(x, Phi(x), phi(x))`, ascending in `x`, symmetric about the origin.
pub const NORMAL_REFERENCES: [(f64, f64, f64); {len(rows)}] = [
"""

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(
        header
        + "\n".join(rows)
        + "\n];\n"
        + difference_header
        + "\n".join(difference_rows)
        + "\n];\n"
    )
    print(f"{len(rows)} points and {len(difference_rows)} brackets written to {out}")


if __name__ == "__main__":
    main()

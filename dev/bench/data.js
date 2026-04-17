window.BENCHMARK_DATA = {
  "lastUpdate": 1776402564510,
  "repoUrl": "https://github.com/Empyrean-Dynamics/nolan",
  "entries": {
    "Nolan Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "0cbb1980d4d376fb942a91e0892d78f1b60b46b6",
          "message": "Split Jet<O, N> into separate Jet1<N> and Jet2<N, H> structs\n\nReplace the unified Jet<ORDER, N> with two separate structs to eliminate\nwasted hessian storage. Jet1<N> stores only value + gradient (no hessian),\nreducing Jet1<6> from 2,104 bytes to 56 bytes (37.5x). Jet2<N, H> stores\nthe exact hessian size needed via a const generic H = N*(N+1)/2.\n\nEach type gets its own operator impls, math functions (via macro helpers\nfor unary ops), and trait impls. The trait hierarchy (Differentiable,\nFirstOrder, SecondOrder, DifferentiableMath, AutoDiff) is unchanged.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-05T12:00:57-08:00",
          "tree_id": "385e96649f8b96bb53bfca54662106228fa91c0d",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/0cbb1980d4d376fb942a91e0892d78f1b60b46b6"
        },
        "date": 1772741191364,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 101,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "3d4d7e333a5cc4ed8d4ae5bdca22491e4aec2607",
          "message": "Fix clippy approx_constant error in variable() tests\n\nReplace test values 3.14 and 2.71 (which clippy flags as approximate\nPI and E constants) with 1.23 and 4.56.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-07T00:15:17-08:00",
          "tree_id": "309393c79fdc06a1841951bec469eac95e19ddfb",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/3d4d7e333a5cc4ed8d4ae5bdca22491e4aec2607"
        },
        "date": 1772871579369,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "03c51fb9c0fa9df5b62cdcdfbac0fc048f2b785a",
          "message": "Remove unused parameters and state modules\n\nBoth modules (Parameter/ParameterConfig and CartesianState/Vec3) are not\nused by any downstream crate in the workspace (villeneuve, scott, kubrick).\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-16T09:27:27-07:00",
          "tree_id": "a341e15b307d371361a264634aa809b0d366c572",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/03c51fb9c0fa9df5b62cdcdfbac0fc048f2b785a"
        },
        "date": 1773678786485,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 94,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 87,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "f1d08be01d6e445bd09d771b42d1e752b9fc8128",
          "message": "Add git-based versioning, README, docs CI, and release workflow\n\n- build.rs: derive version from git tags (tag → 1.0.0, post-tag →\n  1.0.1-dev+hash, dirty → +hash-dirty)\n- lib.rs: add version() function exposing the build-time version string\n- README: project overview with badges (version, CI, docs, Claude Code)\n- CI: add docs job that builds cargo doc and deploys to gh-pages\n- CI: add release workflow that creates GitHub Releases on v* tags\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-16T09:52:39-07:00",
          "tree_id": "ed9577274673ca8ed6ef23fe0b10ccd95ff78941",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/f1d08be01d6e445bd09d771b42d1e752b9fc8128"
        },
        "date": 1773680256772,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "97ca52c9fde12f2a36a1a7896516446a90bf4bc0",
          "message": "Add version bump check to CI\n\nPRs now fail if Cargo.toml version has not been bumped relative to main.\nSkips the check when no version tags exist yet (pre-first-release).\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-16T10:25:55-07:00",
          "tree_id": "33c9f887a43b3e7508ba9e606dd7cdccd98b0622",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/97ca52c9fde12f2a36a1a7896516446a90bf4bc0"
        },
        "date": 1773682242314,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 89,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "ce14e0d5ac64354b04c019d4cd0917277ebc847b",
          "message": "Remove public docs deployment\n\nGitHub Pages on the free plan publishes publicly. Remove the docs CI\njob and badge until private Pages is available.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-16T11:01:51-07:00",
          "tree_id": "7d1476cb0928f923512c97aaff54a023f0de7f6a",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/ce14e0d5ac64354b04c019d4cd0917277ebc847b"
        },
        "date": 1773684373575,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 81,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "0e0d96906bdf03ab649d4dd2cb9d2281d158e31b",
          "message": "Remove version bump check from CI\n\nKeep CI simple: build, lint, test, benchmark on PRs and main pushes.\nReleases are manual: tag vX.Y.Z and push to trigger the release workflow.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-16T13:25:05-07:00",
          "tree_id": "b332c9a74cf02e6893565404a58ffbc77c897ec8",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/0e0d96906bdf03ab649d4dd2cb9d2281d158e31b"
        },
        "date": 1773692987681,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 95,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "9b42ef16354aba9e47bfaaad0ffa456b56da21b5",
          "message": "Add extract_grad/extract_hess to traits, Copy supertrait on Differentiable\n\nAdd provided methods extract_grad<M> and extract_hess<M> to FirstOrder\nand SecondOrder traits for convenient gradient/Hessian extraction into\nfixed-size arrays. Move Copy into Differentiable supertraits since all\nconsumers already require it. Redefine AutoDiff as DifferentiableMath +\nFirstOrder.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-21T00:14:41-07:00",
          "tree_id": "9538b532baea264cea98ff5026a475de86d4a28d",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/9b42ef16354aba9e47bfaaad0ffa456b56da21b5"
        },
        "date": 1774077608758,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 99,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 197,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "a4a20347a52364fbcdbdab23b6970f459afc2678",
          "message": "Add extract_grad/extract_hess to traits, Copy supertrait on Differentiable\n\nAdd provided methods extract_grad<M> and extract_hess<M> to FirstOrder\nand SecondOrder traits for convenient gradient/Hessian extraction into\nfixed-size arrays. Move Copy into Differentiable supertraits since all\nconsumers already require it. Redefine AutoDiff as DifferentiableMath +\nFirstOrder.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-22T18:12:30-07:00",
          "tree_id": "d39429d907d1ad118c46eaed24c98d4d2051fef0",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/a4a20347a52364fbcdbdab23b6970f459afc2678"
        },
        "date": 1774228693035,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 200,
            "range": "± 11",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "92db3907e0d14a5d6d27c4f56b3857062a457475",
          "message": "Bump version to 1.1.1\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-26T20:53:23-07:00",
          "tree_id": "1d13ed18a10b535180635dd07092f9c3fdcbc7fa",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/92db3907e0d14a5d6d27c4f56b3857062a457475"
        },
        "date": 1774583986739,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 71,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 179,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "9abdff092a8a30ae9b3d890b477dd472142b661f",
          "message": "Add linalg module with generic linear algebra primitives\n\nAdd linalg module and bump version to 1.2.0.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-26T22:50:55-07:00",
          "tree_id": "0b7e1fd26b17ac04b199ad1f481511d82646d5ae",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/9abdff092a8a30ae9b3d890b477dd472142b661f"
        },
        "date": 1774591181583,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 96,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 182,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 896,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1303,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 246,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 434,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3212,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5562,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 409,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 978,
            "range": "± 6",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "c7c6a9cc0f0c626980a55ccc83f5b67b1944f2d9",
          "message": "Add generic NLLS solver and bump to v1.3.0\n\nNew nolan::optimization module with a nonlinear least-squares solver\ngeneric over const N (parameter count). The caller implements\nNLLSProblem<N> to provide residuals + Jacobians; the solver handles\niteration, normal equations, LM damping, convergence, and covariance.\n\nFeatures:\n- Gauss-Newton and Levenberg-Marquardt with adaptive damping\n- Optional Bayesian prior augmentation\n- Second-order solver (solve2) with Hessian correction: J^T J + Σ r_i H_i\n- Closure-based convenience API (solve_nlls)\n- Pre-weighting convention: caller pre-multiplies by weight Cholesky\n- Stack-allocated normal equations via const generics\n- Zero external dependencies\n\nDesigned for:\n- scott: orbit determination (N=6 state, N=9 state+nongrav)\n- villeneuve: maneuver targeting (N=3 Δv, N=6 state match)\n\n9 tests: linear system, overdetermined, Rosenbrock, circle fit,\nprior augmentation, singular detection, covariance, LM vs GN, error.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-04-12T17:56:17-07:00",
          "tree_id": "38440f8e275d0bd307985248673400f00c3785a4",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/c7c6a9cc0f0c626980a55ccc83f5b67b1944f2d9"
        },
        "date": 1776042363527,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 96,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 90,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 182,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 891,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1340,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 246,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 431,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3245,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5629,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 407,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 994,
            "range": "± 8",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "13591af3a514d0dd0c1dd8dfb72122b8e86a83e9",
          "message": "Add generic NLLS solver and linalg utilities, bump to v1.3.0\n\nNew nolan::optimization module with a nonlinear least-squares solver\ngeneric over const N. The caller implements NLLSProblem<N> to provide\nresiduals + Jacobians; the solver handles iteration, normal equations,\nLM damping, convergence, and covariance extraction.\n\nNLLS features:\n- Gauss-Newton and Levenberg-Marquardt with adaptive damping\n- Optional Bayesian prior augmentation\n- Second-order solver (solve2) with Hessian correction\n- Closure-based convenience API (solve_nlls)\n- Pre-weighting convention, stack-allocated normal equations\n\nNew generic linalg functions (ported from villeneuve):\n- mat_cholesky<N>: Cholesky decomposition for SPD matrices\n- mat_log_det<N>: log-determinant via LU\n- mat_trace<N>, mat_trace_product<N>: trace operations\n- mat_vec_mul<N>: matrix-vector product\n- vec_norm<N>: Euclidean norm\n- mahalanobis_distance_squared<N>: statistical distance\n- mat_eigenvector_max<N>: power iteration for largest eigenvalue\n\n17 new tests (9 NLLS + 8 linalg). Zero external dependencies.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-04-12T18:15:54-07:00",
          "tree_id": "b11ce53d884b0755444f08dead0b07b169af3e76",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/13591af3a514d0dd0c1dd8dfb72122b8e86a83e9"
        },
        "date": 1776043462260,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 97,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 184,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 889,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1308,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 246,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 429,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3229,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5625,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 409,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 999,
            "range": "± 38",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "d8a22664553b06b0e518d4fd82d7ead48ca07c05",
          "message": "Update README with linalg and optimization docs\n\nAdd sections for the generic linalg functions (Cholesky, Mahalanobis,\neigenvector, log-det, trace) and the NLLS optimization module\n(solve_nlls, NLLSProblem trait, LM/GN, Bayesian prior, solve2).\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-04-12T18:20:08-07:00",
          "tree_id": "1ffe0af9d78926a167a669e0cf2978e19b8b5f17",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/d8a22664553b06b0e518d4fd82d7ead48ca07c05"
        },
        "date": 1776043713217,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 107,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 94,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 180,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 150,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 938,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1411,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 241,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 457,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3493,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5924,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 414,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 998,
            "range": "± 21",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "9abdff092a8a30ae9b3d890b477dd472142b661f",
          "message": "Add linalg module with generic linear algebra primitives\n\nAdd linalg module and bump version to 1.2.0.\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-03-26T22:50:55-07:00",
          "tree_id": "0b7e1fd26b17ac04b199ad1f481511d82646d5ae",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/9abdff092a8a30ae9b3d890b477dd472142b661f"
        },
        "date": 1776057529669,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 97,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 91,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 73,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 182,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 924,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1311,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 248,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 429,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3270,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5569,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 410,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 984,
            "range": "± 41",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "21a563d5ad0ed04646412c10629add32192e12f7",
          "message": "Update README with linalg and optimization docs\n\nAdd sections for the generic linalg functions (Cholesky, Mahalanobis,\neigenvector, log-det, trace) and the NLLS optimization module\n(solve_nlls, NLLSProblem trait, LM/GN, Bayesian prior, solve2).\n\nCo-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>",
          "timestamp": "2026-04-13T21:40:45-07:00",
          "tree_id": "1ffe0af9d78926a167a669e0cf2978e19b8b5f17",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/21a563d5ad0ed04646412c10629add32192e12f7"
        },
        "date": 1776142155241,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 96,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 71,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 182,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 2,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 889,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1338,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 246,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 429,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3253,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5621,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 409,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 979,
            "range": "± 28",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "moeyensj@gmail.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "committer": {
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "2eae9749dcd6ea179a9a411c488aa7866557bcb2",
          "message": "Remove Cargo.lock from version control\n\nLibrary crates should not commit Cargo.lock per Cargo convention.\n\nCo-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-04-16T21:58:22-07:00",
          "tree_id": "06a867b1d486a0f7776bb46a98c68e3eb50b4e72",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/2eae9749dcd6ea179a9a411c488aa7866557bcb2"
        },
        "date": 1776402563515,
        "tool": "cargo",
        "benches": [
          {
            "name": "jet1_6_constant",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_constant",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_constant",
            "value": 38,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 573,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 536,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 4126,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_extract_grad",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_extract_grad",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_extract_hess",
            "value": 73,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 160,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 350,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_f64",
            "value": 1,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dot3_jet1_6",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cross3_jet1_6",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 118,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 873,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1222,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 186,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 373,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3259,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5609,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 472,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 1110,
            "range": "± 47",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}
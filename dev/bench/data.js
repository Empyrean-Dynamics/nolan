window.BENCHMARK_DATA = {
  "lastUpdate": 1778351169944,
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
          "id": "4650d28d8b30017481b1cb38a46018365d5eb752",
          "message": "Jet performance optimizations: inline indexing, shared transcendentals, assign ops\n\n- Add #[inline(always)] to hess_index, tens_index, hess_size, tens_size.\n  Add branchless hess_idx and tens_idx for canonical-ordered hot loops,\n  eliminating Option<usize> unwrap and sort branches per hess/tens element.\n- Rewrite all Jet2 unary math (sin, cos, tan, asin, acos, atan, sinh,\n  cosh, tanh, exp, ln, sqrt) to compute shared intermediates once instead\n  of redundantly calling transcendentals in phi/phi_p/phi_pp expressions.\n  Same for Jet1 exp and sqrt.\n- Replace all hess_index(i,j).unwrap() with hess_idx(i,j) and\n  tens_index(i,j,k).unwrap() with tens_idx(i,j,k) in jet_ops.rs and\n  jet_math.rs hot loops.\n- Add AddAssign, SubAssign, MulAssign, DivAssign for Jet1, Jet2, Jet3\n  (both Self and f64 operands). Add corresponding bounds to Differentiable\n  trait.\n- Add norm_squared3 helper (plain dot product, no sqrt). Rewrite norm3 to\n  use it.\n\nBenchmark impact (Jet2<6,21>):\n  jet2_6_mul:           53.7ns -> 45.9ns  (-14.5%)\n  jet2_6_sin:           51.7ns -> 47.9ns  (-7.3%)\n  jet3_6_mul:           399ns  -> 239ns   (-40.3%)\n  jet3_6_sin:           362ns  -> 184ns   (-49.3%)\n  jet3_6_gravity_accel: 2.68µs -> 1.31µs  (-51.2%)\n  jet2_6_extract_hess:  39.6ns -> 21.4ns  (-45.9%)\n\nCo-Authored-By: Claude Opus 4.6 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-04-16T22:04:18-07:00",
          "tree_id": "0a4ae77435d6603edf5a4122061c816e496eb376",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/4650d28d8b30017481b1cb38a46018365d5eb752"
        },
        "date": 1776402907271,
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
            "name": "jet3_6_constant",
            "value": 59,
            "range": "± 2",
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
            "value": 37,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 186,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 376,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 259,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2222,
            "range": "± 22",
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
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 515,
            "range": "± 3",
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
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
            "value": 139,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 923,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1313,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 248,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 435,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3227,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5705,
            "range": "± 62",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 407,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 983,
            "range": "± 7",
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
          "id": "43e50051fffb26aa716fbfec820a8d307b6c573d",
          "message": "Runtime-dispatched differentiate_dyn for order escalation\n\nAdds a single-entry differentiate_dyn(order, x, f) that picks Jet1/2/3\nat runtime based on an Order enum — mirrors villeneuve's propagate(config)\ndispatch pattern. Motivating use case: propagate with Jet2 covariance by\ndefault and escalate to Jet3 at close approaches when a nonlinearity\ndiagnostic trips.\n\nThe caller passes a function as an AutoDiffFn<N, M> trait impl (typically\na zero-sized struct), so the same function body works with any Jet type.\nReturns a Derivatives<N, M> enum with First/Second/Third variants plus\nuniform accessors (values, jacobian, hessians, tensors). Hessian and\ntensor fields are boxed, keeping the enum small (~200 B) regardless of\nvariant so First dispatch stays cheap.\n\nSpecialized differentiate_dyn_6 and differentiate_dyn_9 inline the\nhessian/tensor widths for the common 6- and 9-parameter state cases.\n\n5 new tests cover all three orders matching the flat API bit-for-bit,\nthe escalation scenario pattern, plus a scalar N=9 example. 3 new benches\n(differentiate_dyn_6_3_{first,second,third}_gravity) show dispatch\noverhead is ~7 ns over the flat API for First; Second/Third pay one and\ntwo Box allocs respectively.\n\nREADME gains a Runtime-dispatched section with the escalation example;\nlang/tribal/performance-baselines.md records the dispatcher timings.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-04-18T13:39:45-07:00",
          "tree_id": "3c3d3de170b2c7f4b28923f7b083a4e385f60c29",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/43e50051fffb26aa716fbfec820a8d307b6c573d"
        },
        "date": 1776545835016,
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
            "name": "jet3_6_constant",
            "value": 59,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_variable",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 57,
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
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 185,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 401,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 84,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 412,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 122,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 262,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 264,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 242,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 295,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 693,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 277,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2119,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 856,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 146,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1319,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 957,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 824,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5728,
            "range": "± 40",
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
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 122,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 511,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1783,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 229,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1720,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 281,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2677,
            "range": "± 30",
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
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
            "value": 139,
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
            "value": 1283,
            "range": "± 22",
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
            "value": 434,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3238,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5619,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 408,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 979,
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
          "id": "f262546d44d383a3dd5bf446cf3f043717faa293",
          "message": "Add NLLSProblem::constrain_step for problem-driven step bounds\n\nExtends the NLLS solver so implementations can clamp Gauss-Newton /\nLevenberg-Marquardt updates before they are applied. LM's lambda damping\nlimits steps in proportion to J^T J; when J itself is near-zero (orbit\ndetermination with rough seeds, close-approach geometries), that\nadaptation cannot recover. constrain_step gives the problem a hook to\nenforce subvector-norm or parameter-wise bounds orthogonal to lambda.\n\nDefault impl is a no-op. Called in both solve() and solve2() after\nmat_solve and before the convergence check, so step_tolerance reflects\nthe clamped step. Wires nolan up for scott's r/v fractional bounds on\nOD iterates (bd empyrean-my3x).\n\nBump nolan 1.5.0 -> 1.5.1.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-04-20T22:05:08-07:00",
          "tree_id": "5dca664af235379d218bf9446f71b71edbba5a76",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/f262546d44d383a3dd5bf446cf3f043717faa293"
        },
        "date": 1776748958766,
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
            "name": "jet3_6_constant",
            "value": 58,
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
            "name": "jet2_6_variable",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 62,
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
            "name": "jet2_6_add",
            "value": 37,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 181,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 352,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 413,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 126,
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
            "name": "jet2_6_sin",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 303,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 252,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 243,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 249,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 121,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 721,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 275,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2112,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 864,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 146,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1316,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 114,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 961,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 67,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 865,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5666,
            "range": "± 176",
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
            "value": 47,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 117,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 506,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1790,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 234,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1729,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 60,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 280,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2653,
            "range": "± 14",
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
            "range": "± 1",
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
            "value": 8,
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
            "value": 1326,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 248,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 435,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3236,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5581,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 409,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 977,
            "range": "± 5",
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
          "id": "c1f605cf84abe5060ac8c8d8fca53aa4e679dee3",
          "message": "Add BSD-3-Clause license\n\nLicense the crate as BSD 3-Clause in Joachim Moeyens's name; required\nmetadata for `cargo publish` and a prerequisite for any public\nrelease. Pairs the LICENSE file with `license = \"BSD-3-Clause\"` in\nthe package manifest and extends the `include` list so cargo\npackaging picks up `LICENSE` and `README.md` alongside the source\ntree.\n\nBSD 3-Clause is permissive (no copyleft, no patent grant beyond\nimplicit) and a clean ecosystem citizen — compatible with Apache-2.0\n/ MIT consumers, which keeps villeneuve / scott / empyrean-core (and\nany future closed-source siblings) flexible about their own\nredistribution choices.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-07T14:30:05-07:00",
          "tree_id": "cf84a3f07335362c55e22919aa9a98632e190acb",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/c1f605cf84abe5060ac8c8d8fca53aa4e679dee3"
        },
        "date": 1778190530916,
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
            "name": "jet3_6_constant",
            "value": 59,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_variable",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 57,
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
            "name": "jet2_6_add",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 181,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 66,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 401,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 398,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 122,
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
            "name": "jet2_6_sin",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 266,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 251,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 249,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 247,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 724,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 274,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2116,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 866,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 146,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1330,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 114,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 980,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 862,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5729,
            "range": "± 36",
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
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 459,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1780,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1718,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 291,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2604,
            "range": "± 26",
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 890,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1330,
            "range": "± 5",
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
            "value": 435,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3259,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5757,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 409,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 976,
            "range": "± 10",
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
          "id": "1d57780c81bb6703cf8e3e673c22214726ca5c27",
          "message": "ci(release): dispatch sibling-tag-push to empyrean-validation on tag\n\nAdds a final step to the Release workflow that fires the validation\nworkflow in empyrean-validation via repository_dispatch whenever\na `v*` tag is pushed.\n\nWiring matches the validation receiver's \"Cross-repo dispatch\"\ndoc block: villeneuve / scott / nolan tag pushes fire\n`sibling-tag-push` because the tag is the moment this repo commits\nto \"this is the version downstream consumers (empyrean-core's pin,\nthen the four distribution channels) will adopt.\" Validation gates\nthat transition. Pre-tag main pushes do not dispatch.\n\nStep runs after the existing release-job steps so a failing\nrelease-build does not waste a validation-suite slot.\n\nRequires `VALIDATION_DISPATCH_TOKEN` — a fine-grained PAT scoped\nto the empyrean-validation repo only with `actions: write`. Per\nthe release runbook, mint via Settings → Personal access tokens\n→ fine-grained → resource owner Empyrean-Dynamics → repository\naccess \"Only select repositories\" → check `empyrean-validation`\n→ permissions: Actions = Read and write. Store as the\n`VALIDATION_DISPATCH_TOKEN` repo secret on this repo.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-08T23:43:18-07:00",
          "tree_id": "b174f03deab2942c6fbb24388dd4f46623699ee4",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/1d57780c81bb6703cf8e3e673c22214726ca5c27"
        },
        "date": 1778310056215,
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
            "name": "jet3_6_constant",
            "value": 59,
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
            "name": "jet2_6_variable",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 60,
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
            "name": "jet2_6_add",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 181,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 401,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 398,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 30,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 127,
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
            "name": "jet2_6_sin",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 268,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 57,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 250,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 47,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 249,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 246,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 724,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 273,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2129,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 866,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 146,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1331,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 125,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 980,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 906,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5716,
            "range": "± 36",
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
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 117,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 457,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1772,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 227,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1732,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 291,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2614,
            "range": "± 59",
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
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
            "value": 933,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1341,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 248,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 435,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3249,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5712,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 461,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 974,
            "range": "± 9",
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
          "id": "c1f605cf84abe5060ac8c8d8fca53aa4e679dee3",
          "message": "Add BSD-3-Clause license\n\nLicense the crate as BSD 3-Clause in Joachim Moeyens's name; required\nmetadata for `cargo publish` and a prerequisite for any public\nrelease. Pairs the LICENSE file with `license = \"BSD-3-Clause\"` in\nthe package manifest and extends the `include` list so cargo\npackaging picks up `LICENSE` and `README.md` alongside the source\ntree.\n\nBSD 3-Clause is permissive (no copyleft, no patent grant beyond\nimplicit) and a clean ecosystem citizen — compatible with Apache-2.0\n/ MIT consumers, which keeps villeneuve / scott / empyrean-core (and\nany future closed-source siblings) flexible about their own\nredistribution choices.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-07T14:30:05-07:00",
          "tree_id": "cf84a3f07335362c55e22919aa9a98632e190acb",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/c1f605cf84abe5060ac8c8d8fca53aa4e679dee3"
        },
        "date": 1778310426763,
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
            "name": "jet3_6_constant",
            "value": 58,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_variable",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_variable",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_add",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_add",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 183,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 400,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 397,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 121,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sin",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sin",
            "value": 57,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 265,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 250,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 249,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 7,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 245,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 724,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 273,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 2131,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 80,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 866,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 146,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1330,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 115,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 980,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 872,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5718,
            "range": "± 104",
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
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 123,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 462,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1793,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 15,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 227,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1721,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 292,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2645,
            "range": "± 54",
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
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 95,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 146,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 906,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1325,
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
            "value": 435,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3197,
            "range": "± 186",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5715,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 412,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 976,
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
            "email": "moeyensj@users.noreply.github.com",
            "name": "Joachim Moeyens",
            "username": "moeyensj"
          },
          "distinct": true,
          "id": "09c78229e0b96b266d2d4d70670cbccf911236f1",
          "message": "Dispatch sibling-tag-push to empyrean-validation on tag\n\nAdds a final step to the Release workflow that fires the\nvalidation workflow in empyrean-validation via repository_dispatch\nwhenever a `v*` tag is pushed.\n\nWiring matches the validation receiver's \"Cross-repo dispatch\"\ndoc block: villeneuve / scott / nolan tag pushes fire\n`sibling-tag-push` because the tag is the moment nolan commits\nto \"this is the version downstream consumers (empyrean-core's\npin, then the four distribution channels) will adopt.\"\nValidation gates that transition. Pre-tag main pushes do not\ndispatch.\n\nThe step runs after the existing release-job steps, so a failing\nrelease-build does not waste a validation-suite slot.\n\nRequires `VALIDATION_DISPATCH_TOKEN` — a fine-grained PAT scoped\nto the empyrean-validation repo only with `actions: write` (plus\nthe auto-required `metadata: read`). Mint via Settings → Personal\naccess tokens → fine-grained → resource owner Empyrean-Dynamics\n→ repository access \"Only select repositories\" → check\nempyrean-validation → permissions: Actions = Read and write.\nStore as the `VALIDATION_DISPATCH_TOKEN` repo secret.\n\nCo-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>",
          "timestamp": "2026-05-09T11:08:25-07:00",
          "tree_id": "b174f03deab2942c6fbb24388dd4f46623699ee4",
          "url": "https://github.com/Empyrean-Dynamics/nolan/commit/09c78229e0b96b266d2d4d70670cbccf911236f1"
        },
        "date": 1778351168352,
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
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_constant",
            "value": 65,
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
            "name": "jet2_6_variable",
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_variable",
            "value": 64,
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
            "name": "jet2_6_add",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_add",
            "value": 203,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul",
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul",
            "value": 372,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_div",
            "value": 11,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_div",
            "value": 93,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_div",
            "value": 403,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_mul_scalar",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_mul_scalar",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_mul_scalar",
            "value": 136,
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
            "name": "jet2_6_sin",
            "value": 65,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sin",
            "value": 279,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_cos",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_cos",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_cos",
            "value": 269,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_sqrt",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_sqrt",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_sqrt",
            "value": 260,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_powi_3",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_powi_3",
            "value": 56,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_powi_3",
            "value": 262,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_atan2",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_atan2",
            "value": 119,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_atan2",
            "value": 609,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_6_gravity_accel",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_6_gravity_accel",
            "value": 293,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_gravity_accel",
            "value": 1957,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_add",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_add",
            "value": 82,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_add",
            "value": 570,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_mul",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_mul",
            "value": 159,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_mul",
            "value": 1035,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_sin",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_sin",
            "value": 123,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_sin",
            "value": 763,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jet1_9_gravity_accel",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_gravity_accel",
            "value": 711,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_gravity_accel",
            "value": 5367,
            "range": "± 149",
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
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet2_9_extract_hess",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_6_extract_tens",
            "value": 385,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jet3_9_extract_tens",
            "value": 1307,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_gravity_magnitude",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate2_6_gravity_magnitude",
            "value": 243,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate3_6_gravity_magnitude",
            "value": 1610,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate1_6_3_gravity_accel",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_first_gravity",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_second_gravity",
            "value": 295,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "differentiate_dyn_6_3_third_gravity",
            "value": 2722,
            "range": "± 49",
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
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "norm3_jet1_6",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_f64",
            "value": 104,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_f64",
            "value": 150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_solve_jet1_6",
            "value": 966,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "mat6_inv_jet1_6",
            "value": 1376,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_f64",
            "value": 241,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_f64",
            "value": 456,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_solve_jet1_9",
            "value": 3404,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "mat9_inv_jet1_9",
            "value": 5919,
            "range": "± 100",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_6_f64",
            "value": 414,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "mat_solve_9_f64",
            "value": 1004,
            "range": "± 24",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}
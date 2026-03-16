window.BENCHMARK_DATA = {
  "lastUpdate": 1773680258753,
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
      }
    ]
  }
}
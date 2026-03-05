window.BENCHMARK_DATA = {
  "lastUpdate": 1772741192034,
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
      }
    ]
  }
}
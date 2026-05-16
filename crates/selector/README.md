# selector

[Back to repository root](../../README.md)

`selector` is an experimental feature-selection crate. It combines Rust-native numerical routines with an optional Python extension layer so the same core logic can be used from both Rust and Python.

## What This Crate Covers

- Orthogonal feature selection
- Distance-correlation scoring
- Logistic-regression support for classification-oriented scoring paths (`ScoreType.LogitGradient`)
- Python bindings behind the `python` feature

## Layout

```text
selector
├── Cargo.toml
├── README.md
├── _notes
├── pyproject.toml
├── python
└── src
    ├── distance_correlation.rs
    ├── lib.rs
    ├── logistic_regression.rs
    ├── logistic_regression
    │   ├── estimator.rs
    │   ├── models.rs
    │   └── test.rs
    ├── main.rs
    ├── orthogonal_selector.rs
    ├── orthogonal_selector
    │   ├── omp.rs
    │   ├── scores.rs
    │   └── test.rs
    ├── prelude.rs
    └── python_api.rs
```

## What Lives Where

- `src/lib.rs` is the library entry point and re-exports the main public surface.
- `src/distance_correlation.rs` contains the distance-correlation implementation.
- `src/orthogonal_selector.rs` wires the orthogonal-selection module.
- `src/orthogonal_selector/omp.rs` contains the main selection flow.
- `src/orthogonal_selector/scores.rs` contains scoring strategies and score types.
- `src/logistic_regression.rs` wires the logistic-regression module.
- `src/logistic_regression/estimator.rs` and `models.rs` contain the estimator internals and supporting model code.
- `src/python_api.rs` exposes the Python module when building with `--features python`.
- `pyproject.toml` and `python/` hold Maturin-based Python packaging metadata.
- `src/main.rs` is a scratch/demo entry point rather than a stable user-facing CLI.

## Rust API Surface

The crate currently re-exports:

- `DistanceCorrelation`
- `OrthogonalSelector`
- `OrthogonalError`
- `ScoreType`
- `LogisticRegressionParams`

## Python API

When built with the `python` feature, the generated Python module exposes:

- `OrthogonalSelector`
- `distance_correlation`

Accepted score-type strings for the Python selector are:

- `residual_variance_ratio`
- `squared_partial_correlation`
- `logit_gradient`

The current binding expects `data` as a 2D feature matrix and `y` as a 2D array with shape `(n_rows, 1)`. Both as float32.

Example:

```python
from selector import OrthogonalSelector, distance_correlation

model = OrthogonalSelector(
    fixed_feature_indices=[],  # list[int]
    score_type="squared_partial_correlation",
    min_score=0.05,
    center_features=True,
    n_jobs=2,
)

selected, scores = model.fit(data, y)
dcor_scores = distance_correlation(data, y, n_jobs=4, sample_size=None)
```

## Common Commands

From the repository root:

```bash
cargo test -p selector
cargo check -p selector --features python
cd crates/selector/ && python -m maturin build --features python
```

Built wheels are written under `target/wheels/`.

## Notes

- This crate is exploratory and the internal module boundaries are still moving.
- The library API is the main integration surface; `src/main.rs` is best treated as a sandbox.

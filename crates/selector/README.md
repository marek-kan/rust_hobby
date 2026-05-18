# selector

[Back to repository root](../../README.md)

`selector` is an experimental feature-selection crate. It combines Rust-native numerical routines with an optional Python extension layer so the same core logic can be used from both Rust and Python.

## What This Crate Covers

- Orthogonal feature selection
- Distance-correlation scoring
- Logistic-regression support for classification-oriented scoring paths (`logit_gradient`)
- Python bindings behind the `python` feature

## Layout

```text
selector
├── Cargo.toml
├── README.md
├── _notes
├── pyproject.toml
├── py_src
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
- `pyproject.toml` and `py_src/selector/` hold Maturin-based Python packaging metadata and the Python package shim.
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

Accepted score-type strings for the Python selector dictate the evaluation metric used to select the next feature:

- **`residual_variance_ratio`**: Measures feature novelty. It selects the candidate feature that retains the highest proportion of its original variance after being orthogonalized against the already-selected features. Note that this metric evaluates linear independence from the selected set and does not involve the target variable directly.
- **`squared_partial_correlation`**: Suitable for regression tasks. It evaluates candidate features based on their squared partial correlation with the target variable, controlling for the impact of the features that have already been selected.
- **`logit_gradient`**: Suitable for classification tasks. It selects the candidate feature that produces the largest improvement in logistic-regression log loss for the currently selected features. It supports both binary and multiclass target arrays.

The current binding expects `data` as a 2D feature matrix and `y` as a 2D array with shape `(n_rows, 1)`. Both as float32.

Example:

```python
import numpy as np
from selector import OrthogonalSelector, distance_correlation

# Generate random float32 data
n_samples, n_features = 1000, 20
data = np.random.randn(n_samples, n_features).astype(np.float32)
y = (data[:, 0] * 2.0 + data[:, 3] + np.random.randn(n_samples)).astype(np.float32).reshape(-1, 1)

# Initialize the orthogonal feature selector
model = OrthogonalSelector(
    fixed_feature_indices=[0],  # Can be empty list, i.e., `[]`
    score_type="squared_partial_correlation",
    min_score=0.01,
    center_features=True,
    n_jobs=2,
)

# Fit the model to retrieve selected feature indices and their iteration scores
selected_indices, scores_dict = model.fit(data, y)
print("Selected features:", selected_indices)
print("Selection scores:", scores_dict)

# Compute standalone distance-correlation scores for all features vs the target
dcor_scores = distance_correlation(data, y, n_jobs=4, sample_size=None)
print("Distance correlation scores:", dcor_scores)
```

## Quick Start

The Python wheel is a native extension, so install a wheel built for your platform or build the package locally with Maturin.

### Install A Built Wheel

From `crates/selector`:

```bash
python -m maturin build --features python
python -m pip install target/wheels/selector-*.whl
```

### Editable Development Install

From `crates/selector`:

```bash
python -m maturin develop --features python
```

### First Run

```python
import numpy as np
from selector import OrthogonalSelector, distance_correlation

rng = np.random.default_rng(0)
x = rng.normal(size=(256, 8)).astype(np.float32)
y = (2.0 * x[:, 0] - x[:, 3] + rng.normal(scale=0.5, size=256)).astype(np.float32).reshape(-1, 1)

selector = OrthogonalSelector(
    fixed_feature_indices=[],
    score_type="squared_partial_correlation",
    min_score=0.01,
    center_features=True,
    n_jobs=4,
)

selected_indices, scores = selector.fit(x, y)
print(selector)
print(selected_indices)
print(scores)

dcor_scores = distance_correlation(x, y, n_jobs=4)
print(dcor_scores)
```

Runtime docs are available through `help(OrthogonalSelector)` and `help(distance_correlation)`.

For classification problems, use `score_type="logit_gradient"`; it ranks candidate features by improvement in logistic-regression log loss.

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

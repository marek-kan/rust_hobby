from typing import Literal, Optional
from numpy.typing import NDArray
from numpy import float32

class OrthogonalSelector:
    def __init__(
        self,
        fixed_feature_indices: list[int],
        score_type: Literal["residual_variance_ratio", "squared_partial_correlation", "logit_gradient"],
        min_score: float,
        center_features: bool,
        n_jobs: int | None = None,
        *,
        max_iter: int = 500,
        alpha: float = 1.0,
        learning_rate: float = 0.05,
        r_tol: float = 1e-4,
    ) -> None: ...

    def fit(
        self,
        data: NDArray[float32],
        y: NDArray[float32],
        sample_weights: NDArray[float32] | None = None,
    ) -> tuple[list[int], dict[int, float]]: ...

    def __repr__(self) -> str: ...

def distance_correlation(
    x: NDArray[float32],
    y: NDArray[float32],
    n_jobs: int,
    sample_size: Optional[int]
) -> NDArray[float32]:
    """
    Compute the distance correlation between each column of ``x`` and ``y``.

    Parameters
    ----------
    x : NDArray[float32]
        Feature matrix of shape ``(n, p)``.
    y : NDArray[float32]
        Target column vector of shape ``(n, 1)``.
    n_jobs : int
        Number of threads to use.
    sample_size: Optional
        What sub-sample to use. USeful for large datasets

    Returns
    -------
    NDArray[float32]
        Array of length ``p`` with one distance-correlation score per column.
        The target distance matrix is computed once and reused across all columns.
    """
    ...

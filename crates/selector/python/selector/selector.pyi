from typing import Literal
from numpy.typing import NDArray
from numpy import float64

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
        data: NDArray[float64],
        y: NDArray[float64],
        sample_weights: NDArray[float64] | None = None,
    ) -> tuple[list[int], dict[int, float]]: ...

    def __repr__(self) -> str: ...
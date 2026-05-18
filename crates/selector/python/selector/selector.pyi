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
    """
    An orthogonal feature selector that iteratively picks features maximizing a given score.

    At each step, the selector evaluates remaining candidate features by comparing them 
    against the target, while orthogonalizing against the already selected features.

    Parameters
    ----------
    fixed_feature_indices : list[int]
        Indices of features that are always included in the selection model.
    score_type : {"residual_variance_ratio", "squared_partial_correlation", "logit_gradient"}
        The evaluation metric used to select the next feature.
        - "residual_variance_ratio": Measures feature novelty; selects the feature that 
          retains the highest proportion of its original variance after orthogonalizing 
          against already selected features (does not involve the target).
        - "squared_partial_correlation": Suitable for regression; measures the squared 
          partial correlation between the feature and the target, given already selected features.
        - "logit_gradient": Suitable for classification; selects the feature that produces 
          the largest absolute gradient step in a logistic regression model.
    min_score : float
        The minimum score required to select a candidate feature. Selection stops early if 
        no remaining feature achieves this value.
    center_features : bool
        Whether to center the features (subtract the mean) before selection.
    n_jobs : int | None, default=None
        Number of CPU threads to use for parallel feature evaluation.
    max_iter : int, default=500
        Maximum number of iterations, determining the maximum number of features to select.
    alpha : float, default=1.0
        Regularization strength used depending on the score mapping.
    learning_rate : float, default=0.05
        Learning rate for optimization routines (e.g., used internally for classification).
    r_tol : float, default=1e-4
        Relative tolerance parameter for declaring convergence.
    """

    def fit(
        self,
        x: NDArray[float32],
        y: NDArray[float32],
        sample_weights: NDArray[float32] | None = None,
    ) -> tuple[list[int], dict[int, float]]: ...
    """
    Compute score between features, ``x``, and the target, ``y``.

    Parameters
    ----------
    x : NDArray[float32]
        Feature matrix of shape ``(n, m)``.
    y : NDArray[float32]
        Target column vector of shape ``(n, 1)``.
    sample_weights: Optional NDArray[float32]
        For assigning different level of importance to the samples, shape  ``(n, 1)``.

    Returns
    -------
    Tuple[List[int], Dict[int, float]]
        Tuple of length 2.
        Index 0: List of selected indices.
        Index 1: Dictionary where key is the index of a feature and value is its score value.
    """

    def __repr__(self) -> str: ...

def distance_correlation(
    x: NDArray[float32],
    y: NDArray[float32],
    n_jobs: Optional[int] = 1,
    sample_size: Optional[int] = None
) -> NDArray[float32]:
    """
    Compute the distance correlation between each column of ``x`` and ``y``.

    Parameters
    ----------
    x : NDArray[float32]
        Feature matrix of shape ``(n, m)``.
    y : NDArray[float32]
        Target column vector of shape ``(n, 1)``.
    n_jobs : int
        Number of threads to use.
    sample_size: Optional[int]
        What sub-sample to use. Useful for large datasets.

    Returns
    -------
    NDArray[float32]
        Array of length ``p`` with one distance-correlation score per column.
        The target distance matrix is computed once and reused across all columns.
    """
    ...

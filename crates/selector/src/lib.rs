pub(crate) mod distance_correlation;
pub(crate) mod logistic_regression;
pub(crate) mod orthogonal_selector;
pub(crate) mod prelude;

pub use distance_correlation::DistanceCorrelation;
pub use orthogonal_selector::{OrthogonalError, OrthogonalSelector, ScoreType};

pub use logistic_regression::LogisticRegressionParams;

#[cfg(feature = "python")]
mod python_api;

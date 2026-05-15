use crate::prelude::*;

use crate::logistic_regression::LogisticRegressionParams;
use crate::logistic_regression::estimator::*;
use crate::logistic_regression::models::*;

pub(crate) mod omp;
mod scores;

pub use omp::{OrthogonalError, OrthogonalSelector};
pub use scores::ScoreType;

#[cfg(test)]
mod test;

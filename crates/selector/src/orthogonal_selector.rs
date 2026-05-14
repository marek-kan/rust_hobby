use crate::prelude::*;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};

use crate::logistic_regression::LogisticRegressionParams;
use crate::logistic_regression::estimator::*;

pub(crate) mod omp;
mod scores;

pub use omp::{OrthogonalError, OrthogonalSelector};
pub use scores::ScoreType;

#[cfg(test)]
mod test;

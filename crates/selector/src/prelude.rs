pub(crate) use ndarray::{Array, Array1, Array2, Axis, prelude::*};
pub(crate) use ndarray::{Data, ShapeError};
use ndarray_rand::{RandomExt, rand_distr::Normal};
pub(crate) use std::collections::HashMap;
pub(crate) use thiserror::Error;

pub(crate) const EPS: f64 = 1e-12;

#[derive(Default)]
pub(crate) struct StandardScaler {
    pub(crate) mu: Option<Array2<f64>>,
    pub(crate) std: Option<Array2<f64>>,
}
impl StandardScaler {
    fn is_fitted(&self) -> bool {
        self.mu.is_some()
    }

    pub(crate) fn new() -> Self {
        StandardScaler {
            mu: None,
            std: None,
        }
    }

    pub(crate) fn fit(&mut self, x: &Array2<f64>) -> Result<(), FitError> {
        if !self.is_fitted() {
            if let Some(mu) = x.mean_axis(Axis(0)) {
                self.mu = Some(mu.insert_axis(Axis(0)))
            } else {
                return Err(FitError::ErrorDuringFit(
                    "Failed to compute mean in `StandardScaler`".into(),
                ));
            };

            let stds = x.std_axis(Axis(0), 0.).insert_axis(Axis(0));

            if let Some((idx, _)) = stds.iter().enumerate().find(|(_, v)| v.abs() < EPS) {
                return Err(FitError::ConstantFeatureAt(idx));
            }

            self.std = Some(stds);

            Ok(())
        } else {
            Err(FitError::AlreadyFitted)
        }
    }

    pub(crate) fn transform(&self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        if self.is_fitted() {
            let mu = self.mu.as_ref().unwrap();
            let s = self.std.as_ref().unwrap();

            Ok((x - mu) / s)
        } else {
            Err(FitError::NotFitted)
        }
    }

    pub(crate) fn fit_transform(&mut self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        self.fit(x)?;
        self.transform(x)
    }
}

#[derive(Error, Debug)]
pub enum FitError {
    #[error("Not fitted yet!")]
    NotFitted,
    #[error("{0}")]
    ErrorDuringFit(String),
    #[error("Constant feature value detected for column index: {0}")]
    ConstantFeatureAt(usize),
    #[error("Already fitted!")]
    AlreadyFitted,
}

#[allow(dead_code)]
pub(crate) fn get_random_normal(n: usize) -> Array2<f64> {
    Array::random((n, 1), Normal::new(0.0, 1.0).unwrap())
}

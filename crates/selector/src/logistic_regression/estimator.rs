use super::*;

pub(crate) trait Estimator: Clone {
    fn fit(
        &mut self,
        X: &Array2<f32>,
        y: &Array2<f32>,
        sample_weights: &Option<Array2<f32>>,
    ) -> Result<(), FitError>;
    fn decision_boundary(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError>;
    fn loss(
        &self,
        logits: ArrayView2<f32>,
        y: &Array2<f32>,
        sample_weights: &Array2<f32>,
    ) -> Result<f32, FitError>;
    fn predict(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError>;
    fn predict_proba(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError>;
    fn is_fitted(&self) -> bool;
}

#[derive(Clone)]
pub(crate) enum LogisticModel {
    Binary(LogisticRegression),
    Multi(OneVsAll),
}

// Macro to delegate calls to the inner variants
macro_rules! delegate_to_inner {
    ($self:ident, $method:ident $(, $arg:expr)*) => {
        match $self {
            LogisticModel::Binary(inner) => inner.$method($($arg),*),
            LogisticModel::Multi(inner) => inner.$method($($arg),*),
        }
    };
}

impl Estimator for LogisticModel {
    fn is_fitted(&self) -> bool {
        delegate_to_inner!(self, is_fitted)
    }

    fn fit(
        &mut self,
        X: &Array2<f32>,
        y: &Array2<f32>,
        sw: &Option<Array2<f32>>,
    ) -> Result<(), FitError> {
        delegate_to_inner!(self, fit, X, y, sw)
    }

    fn decision_boundary(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        delegate_to_inner!(self, decision_boundary, X)
    }

    fn loss(
        &self,
        logits: ArrayView2<f32>,
        y: &Array2<f32>,
        sw: &Array2<f32>,
    ) -> Result<f32, FitError> {
        delegate_to_inner!(self, loss, logits, y, sw)
    }

    fn predict(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        delegate_to_inner!(self, predict, X)
    }

    fn predict_proba(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        delegate_to_inner!(self, predict_proba, X)
    }
}

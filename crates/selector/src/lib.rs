use core::f64;

use ndarray::{Data, linalg::Dot, prelude::*};
use ndarray_rand::{
    RandomExt,
    rand_distr::{Normal, num_traits::ToPrimitive},
};

const EPS: f64 = 1e-12;

enum ScoreType {
    FeatureInformationRatio,
    ResidualRatio,
    LogitGradient,
}

pub fn get_random_normal(n: usize) -> Array2<f64> {
    Array::random((n, 1), Normal::new(0.0, 1.0).unwrap())
}

pub fn weighted_inner<T, D>(x: &ArrayBase<T, D>, y: &ArrayBase<T, D>, w: &ArrayBase<T, D>) -> f64
where
    T: Data<Elem = f64>,
    D: Dimension,
{
    let product = w * x * y;
    product.sum()
}

pub fn weighted_norm(x: &Array2<f64>, w: &Array2<f64>) -> f64 {
    let product = (w * x).t().dot(x);
    product.sum().sqrt()
}

pub fn orthogonalize(q: &Array2<f64>, x: &Array2<f64>, w: &Array2<f64>) -> Array2<f64> {
    let mut r = x.clone();

    if q.ncols() != 0 {
        let wr = w * &r;
        let coeff = q.t().dot(&wr);
        r = r - q.dot(&coeff);

        // stability pass
        let wr = w * &r;
        let coeff = q.t().dot(&wr);
        r = r - q.dot(&coeff);
    };

    r
}

pub fn feature_information_ratio(x: &Array2<f64>, r: &Array2<f64>, w: &Array2<f64>) -> f64 {
    let denom = (w * x * x).sum();

    if denom <= EPS {
        0.0
    } else {
        (w * r * r).sum() / denom
    }
}

pub fn residual_ratio(r_x: &Array2<f64>, r_y: &Array2<f64>, w: &Array2<f64>) -> f64 {
    let numer = (w * r_x * r_y).sum();
    let denom_x = (w * r_x * r_x).sum();
    let denom_y = (w * r_y * r_y).sum();

    (numer * numer) / (denom_x * denom_y)
}

#[derive(Debug)]
enum LogisticRegressionError {
    NotFitted,
    FitError,
}

pub struct LogisticRegression {
    max_iter: usize,
    alpha: f64,
    learning_rate: f64,
    coeff: Option<Array1<f64>>,
    is_fitted: bool,
}

impl LogisticRegression {
    fn new(max_iter: usize, alpha: f64, learning_rate: f64) -> Self {
        LogisticRegression {
            max_iter,
            alpha,
            learning_rate,
            coeff: None,
            is_fitted: false,
        }
    }

    fn predict(&self, X: &Array2<f64>) -> Result<Array2<f64>, LogisticRegressionError> {
        // if !self.is_fitted {
        //     return Err(LogisticRegressionError::NotFitted);
        // }

        if let Some(c) = &self.coeff {
            let z = X.dot(&c.view().insert_axis(Axis(1)));
            let pred = 1.0 / (1.0 + (-1.0 * &z).exp());
            Ok(pred.mapv(|x| x.max(EPS).min(1.0 - EPS)))
        } else {
            Err(LogisticRegressionError::NotFitted)
        }
    }

    fn loss(
        &self,
        X: &Array2<f64>,
        y: &Array2<f64>,
        n: usize,
    ) -> Result<f64, LogisticRegressionError> {
        let y_hat = self.predict(X)?;
        let c = self.coeff.as_ref().unwrap(); // if no coeffs `predict` throws Error
        let _n = n as f64;

        let base_loss = 1.0 / _n * (-1.0 * y * y_hat.ln() - (1.0 - y) * (1.0 - y_hat).ln());

        let reg_loss = base_loss + self.alpha / (2.0 * _n) * c.t().dot(&c.pow2());

        Ok(reg_loss.sum())
    }

    fn update(
        &mut self,
        X: &Array2<f64>,
        y: &Array2<f64>,
        n: usize,
    ) -> Result<(), LogisticRegressionError> {
        let pred = self.predict(X)?;
        let diff_pred = y - &pred;
        let n_multiplier = 1.0 / n as f64;

        // Gradients
        let grad0 = self.learning_rate * n_multiplier * diff_pred.sum();
        let grad1: Array2<f64> =
            self.learning_rate * n_multiplier * X.slice(s![.., 1..]).t().dot(&diff_pred);
        let decay = 1.0 - self.learning_rate * self.alpha * n_multiplier;

        // Update
        let weights = self.coeff.as_mut().unwrap();

        weights[0] -= grad0;

        let mut w_slice = weights.slice_mut(s![1..]);
        w_slice *= decay;
        w_slice -= &grad1.column(0);

        Ok(())
    }

    fn fit(&mut self, X: &Array2<f64>, y: &Array2<i64>) -> Result<(), LogisticRegressionError> {
        let n = X.nrows();
        let ones = Array::ones((n, 1));
        let X_w_intercept = ndarray::concatenate(Axis(1), &[ones.view(), X.view()]).unwrap();
        let y_float = y.mapv(|v| v as f64);

        self.coeff = Some(Array::zeros(X_w_intercept.ncols()));

        let mut last_loss = f64::INFINITY;

        for iter in 0..self.max_iter {
            self.update(&X_w_intercept, &y_float, n)?;

            let loss = self.loss(&X_w_intercept, &y_float, n)?;

            if last_loss - loss <= 1e-6 {
                break;
            } else {
                last_loss = loss
            }
        }

        self.is_fitted = true;
        Ok(())
    }
}

impl Default for LogisticRegression {
    fn default() -> Self {
        LogisticRegression {
            max_iter: 100,
            alpha: 1.0,
            learning_rate: 0.05,
            coeff: None,
            is_fitted: false,
        }
    }
}
/*
store intercept separately -> z = intercept + X.dot(coeffs)
    Helps with always adding ones vector
    
is_fitted is redundant -> coeff.is_some()


*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_predict_runs() {
        let X: Array2<f64> = array![[0.0], [1.0], [2.0], [3.0]];

        let y: Array2<i64> = array![[0], [0], [1], [1]];

        let mut model = LogisticRegression::default();
        model.fit(&X, &y);

        let ones = ndarray::Array::ones((X.nrows(), 1));
        let X_w_intercept =
            ndarray::concatenate(ndarray::Axis(1), &[ones.view(), X.view()]).unwrap();

        let pred = model.predict(&X_w_intercept).unwrap();

        assert_eq!(pred.nrows(), 4);

        for p in pred.iter() {
            assert!(*p > 0.0 && *p < 1.0);
        }
    }

    #[test]
    fn test_coefficients_change() {
        let X: Array2<f64> = array![[0.0], [1.0], [2.0], [3.0]];

        let y: Array2<i64> = array![[0], [0], [1], [1]];

        let mut model = LogisticRegression::default();
        model.fit(&X, &y);

        let coeff = model.coeff.unwrap();

        let norm: f64 = coeff.iter().map(|v| v.abs()).sum();

        assert!(norm > 0.0);
    }
}

use ndarray::{Data, linalg::Dot, prelude::*};
use ndarray_rand::{RandomExt, rand_distr::Normal};

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

#[derive(Default)]
pub struct StandardScaler {
    mu: Option<Array2<f64>>,
    std: Option<Array2<f64>>,
}
impl StandardScaler {
    fn is_fitted(&self) -> bool {
        self.mu.is_some()
    }

    pub fn new() -> Self {
        StandardScaler {
            mu: None,
            std: None,
        }
    }

    pub fn fit(&mut self, x: &Array2<f64>) -> Result<(), FitError> {
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

    pub fn transform(&self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        if self.is_fitted() {
            let mu = self.mu.as_ref().unwrap();
            let s = self.std.as_ref().unwrap();

            Ok((x - mu) / s)
        } else {
            Err(FitError::NotFitted)
        }
    }

    pub fn fit_transform(&mut self, x: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        self.fit(x)?;
        self.transform(x)
    }
}

#[derive(Debug)]
pub enum FitError {
    NotFitted,
    ErrorDuringFit(String),
    ConstantFeatureAt(usize),
    AlreadyFitted,
}

pub struct LogisticRegression {
    max_iter: usize,
    alpha: f64,
    learning_rate: f64,
    pub coeff: Option<Array1<f64>>,
    pub intercept: Option<f64>,
    pub losses: Vec<f64>,
}

impl LogisticRegression {
    pub fn new(max_iter: usize, alpha: f64, learning_rate: f64) -> Self {
        LogisticRegression {
            max_iter,
            alpha,
            learning_rate,
            coeff: None,
            intercept: None,
            losses: vec![],
        }
    }

    pub fn is_fitted(&self) -> bool {
        self.coeff.is_some()
    }

    pub fn predict(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let proba = self.predict_proba(X)?;
        Ok(proba.round())
    }

    pub fn predict_proba(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let coeff = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let mut z = X.dot(&coeff.view().insert_axis(Axis(1)));

        if let Some(intercept) = self.intercept {
            // we might not have intercept
            z += intercept;
        }

        let pred = 1.0 / (1.0 + (-1.0 * &z).exp());
        Ok(pred.mapv(|x| x.max(EPS).min(1.0 - EPS)))
    }

    fn loss(&self, y_hat: &Array2<f64>, y: &Array2<f64>, n: usize) -> Result<f64, FitError> {
        let c = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let _n = n as f64;

        let base_loss = 1.0 / _n * (-1.0 * y * y_hat.ln() - (1.0 - y) * (1.0 - y_hat).ln());

        let reg_loss = base_loss + self.alpha / (2.0 * _n) * c.t().dot(&c.pow2());

        Ok(reg_loss.sum())
    }

    fn update(&mut self, X: &Array2<f64>, y: &Array2<f64>, n: usize) -> Result<(), FitError> {
        let pred = self.predict_proba(X)?;
        let diff_pred = &pred - y;
        let n_multiplier = 1.0 / n as f64;

        // Gradients
        let grad0 = self.learning_rate * n_multiplier * diff_pred.sum();
        let grad1: Array2<f64> = self.learning_rate * n_multiplier * X.t().dot(&diff_pred);
        let decay = 1.0 - self.learning_rate * self.alpha * n_multiplier;

        // Update
        let weights = self.coeff.as_mut().unwrap();
        if let Some(intercept) = self.intercept.as_mut() {
            *intercept -= grad0;
        };

        *weights *= decay;
        *weights -= &grad1.column(0);

        Ok(())
    }

    pub fn fit(&mut self, X: &Array2<f64>, y: &Array2<i32>) -> Result<(), FitError> {
        let n = X.nrows();
        let y_float = y.mapv(|v| v as f64);

        self.coeff = Some(Array::zeros(X.ncols()));
        self.intercept = Some(0.0);

        let mut last_loss = f64::INFINITY;

        for _ in 0..self.max_iter {
            self.update(&X, &y_float, n)?;

            let y_hat = self.predict_proba(&X)?;

            let loss = self.loss(&y_hat, &y_float, n)?;
            self.losses.push(loss);

            if (last_loss - loss).abs() <= 1e-6 {
                break;
            } else {
                last_loss = loss
            }
        }

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
            intercept: None,
            losses: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaler() {
        let arr: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
        let expected_std = arr.std_axis(Axis(0), 0.0);
        let expected_mu = arr.mean_axis(Axis(0)).unwrap();

        let mut scaler = StandardScaler::default();

        let transformed = scaler.fit_transform(&arr).unwrap();

        let mu = scaler.mu.unwrap();
        let stds = scaler.std.unwrap();

        assert!((mu.column(0)[0] - expected_mu[0]).abs() <= EPS);
        assert!((stds.column(0)[0] - expected_std[0]).abs() <= EPS);

        assert!(&transformed.mean().unwrap().abs() < &EPS);
        assert!((&transformed.std(0.0) - 1.0).abs() < EPS);
    }

    #[test]
    fn test_fitpredict_standardized_runs() {
        let features: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
        let y: Array2<i32> = array![[0], [0], [0], [1], [1]];
        let mut scaler = StandardScaler::new();

        let x = scaler.fit_transform(&features).unwrap();

        let mut model = LogisticRegression::default();
        let _ = model.fit(&x, &y);

        let pred_proba = model.predict_proba(&x).unwrap();
        let pred = model.predict(&x).unwrap();

        assert_eq!(pred.nrows(), y.nrows());

        for p in pred_proba.iter() {
            assert!(*p > 0.0 && *p < 1.0);
        }

        for p in pred.iter() {
            assert!(*p == 1.0 || *p == 0.0);
        }
    }

    #[test]
    fn test_coefficients_change() {
        let features: Array2<f64> = array![[0.0], [1.0], [0.5], [5.0], [3.0]];
        let y: Array2<i32> = array![[0], [0], [0], [1], [1]];
        let mut scaler = StandardScaler::new();

        let x = scaler.fit_transform(&features).unwrap();

        let mut model = LogisticRegression::default();
        let _ = model.fit(&x, &y);

        let coeff = model.coeff.unwrap();

        let norm: f64 = coeff.iter().map(|v| v.abs()).sum();
        let intercept = model.intercept.unwrap().abs();

        assert!(intercept > 0.0);
        assert!(norm > 0.0);
    }
}

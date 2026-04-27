use super::*;

pub struct LogisticRegressionParams {
    pub max_iter: usize,
    pub alpha: f64,
    pub learning_rate: f64,
    pub r_tol: f64,
}
impl Default for LogisticRegressionParams {
    fn default() -> Self {
        LogisticRegressionParams {
            max_iter: 500,
            alpha: 1.0,
            learning_rate: 0.05,
            r_tol: 1e-4,
        }
    }
}

#[derive(Clone)]
pub(crate) struct LogisticRegression {
    pub(crate) max_iter: usize,
    pub(crate) alpha: f64,
    pub(crate) learning_rate: f64,
    pub(crate) r_tol: f64,
    pub(crate) coeff: Option<Array1<f64>>,
    pub(crate) intercept: Option<f64>,
    pub(crate) losses: Vec<f64>,
}

impl LogisticRegression {
    pub(crate) fn new(max_iter: usize, alpha: f64, learning_rate: f64, r_tol: f64) -> Self {
        LogisticRegression {
            max_iter,
            alpha,
            learning_rate,
            r_tol,
            coeff: None,
            intercept: None,
            losses: vec![],
        }
    }

    pub(crate) fn is_fitted(&self) -> bool {
        self.coeff.is_some()
    }

    fn sigmoid(&self, z: &Array2<f64>) -> Array2<f64> {
        z.mapv(|z| {
            if z >= 0.0 {
                1.0 / (1.0 + (-z).exp())
            } else {
                let ez = z.exp();
                ez / (1.0 + ez)
            }
        })
    }

    pub(crate) fn decision_boundary(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let coeff = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let mut z = X.dot(&coeff.view().insert_axis(Axis(1)));

        if let Some(intercept) = self.intercept {
            // we might not have intercept
            z += intercept;
        }

        Ok(z)
    }

    #[allow(dead_code)]
    pub(crate) fn predict(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let proba = self.predict_proba(X)?;

        Ok(proba.mapv(|p| if p >= 0.5 { 1.0 } else { 0.0 }))
    }

    #[allow(dead_code)]
    pub(crate) fn predict_proba(&self, X: &Array2<f64>) -> Result<Array2<f64>, FitError> {
        let z = self.decision_boundary(X)?;

        let pred = self.sigmoid(&z);
        Ok(pred.mapv(|x| x.clamp(EPS, 1.0 - EPS)))
    }

    pub(crate) fn loss(
        &self,
        logits: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: &Array2<f64>,
    ) -> Result<f64, FitError> {
        let c = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let weight_sum = sample_weights.sum();

        // logaddexp(0, z) = ln(1 + exp(z))  (stable form below)
        let log_term = logits.mapv(|z| {
            if z > 0.0 {
                z + (-z).exp().ln_1p()
            } else {
                z.exp().ln_1p()
            }
        });

        let base_loss = (sample_weights * (log_term - y * logits)).sum() / weight_sum;

        let reg_loss = self.alpha / (2.0 * weight_sum) * c.pow2().sum();

        Ok(base_loss + reg_loss)
    }

    fn update(
        &mut self,
        X: &Array2<f64>,
        logits: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: &Array2<f64>,
    ) -> Result<(), FitError> {
        let pred = self.sigmoid(logits);
        let diff_pred = (&pred - y) * sample_weights;
        let norm = 1.0 / sample_weights.sum();

        // Gradients
        let grad0 = self.learning_rate * norm * diff_pred.sum();
        let grad1 = self.learning_rate * norm * X.t().dot(&diff_pred);
        let decay = 1.0 - self.learning_rate * self.alpha * norm;

        // Update
        let weights = self.coeff.as_mut().unwrap();
        if let Some(intercept) = self.intercept.as_mut() {
            *intercept -= grad0;
        };

        *weights *= decay;
        *weights -= &grad1.column(0);

        Ok(())
    }

    pub(crate) fn fit(
        &mut self,
        X: &Array2<f64>,
        y: &Array2<f64>,
        sample_weights: Option<Array2<f64>>,
    ) -> Result<(), FitError> {
        if !self.is_fitted() {
            return Err(FitError::AlreadyFitted);
        }

        let n = X.nrows();

        let sw = match sample_weights {
            Some(sw) => sw,
            None => Array::ones((n, 1)),
        };

        self.coeff = Some(Array::zeros(X.ncols()));
        self.intercept = Some(0.0);

        let mut last_loss = f64::INFINITY;

        for _ in 0..self.max_iter {
            let logits_hat = self.decision_boundary(X)?;

            let loss = self.loss(&logits_hat, y, &sw)?;
            self.losses.push(loss);

            if (last_loss - loss).abs() / last_loss.abs().max(1.0) <= self.r_tol {
                break;
            } else {
                self.update(X, &logits_hat, y, &sw)?;
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
            learning_rate: 0.1,
            r_tol: 1e-4,
            coeff: None,
            intercept: None,
            losses: vec![],
        }
    }
}

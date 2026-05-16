use super::*;

/// One vs. all classes logistic regression implementation
#[derive(Default, Clone)]
pub(crate) struct OneVsAll {
    pub(crate) params: LogisticRegressionParams,
    pub(crate) estimators: HashMap<i64, LogisticRegression>,
}

impl OneVsAll {
    pub(crate) fn new(max_iter: usize, alpha: f32, learning_rate: f32, r_tol: f32) -> Self {
        OneVsAll {
            params: LogisticRegressionParams {
                max_iter,
                alpha,
                learning_rate,
                r_tol,
            },
            estimators: HashMap::new(),
        }
    }

    fn create_class_mask(&self, y: &Array2<f32>, target_class: &f32) -> Array2<f32> {
        // Create a binary mask where 1.0 represents the target class and 0.0 for others
        let mask: ArrayBase<ndarray::OwnedRepr<f32>, Dim<[usize; 2]>, f32> =
            y.mapv(|el| if &el == target_class { 1.0 } else { 0.0 });

        // Reshape the mask into a 2D array with one column
        mask.into_shape_with_order((y.len(), 1))
            .expect("Failed to reshape masked target vector")
        // mask.to_shape((y.len(), 1)).unwrap()
    }
}

impl Estimator for OneVsAll {
    fn is_fitted(&self) -> bool {
        if !self.estimators.is_empty() {
            self.estimators.iter().all(|(_, est)| est.is_fitted())
        } else {
            false
        }
    }

    fn fit(
        &mut self,
        X: &Array2<f32>,
        y: &Array2<f32>,
        sample_weights: &Option<Array2<f32>>,
    ) -> Result<(), FitError> {
        let classes = unique_f32(y);

        for cls in classes {
            let y_masked = self.create_class_mask(y, &cls);

            let mut est = LogisticRegression::new(
                self.params.max_iter,
                self.params.alpha,
                self.params.learning_rate,
                self.params.r_tol,
            );

            est.fit(X, &y_masked, sample_weights)
                .unwrap_or_else(|_| panic!("Failed to fit estimator for class: {}", cls));

            self.estimators.insert(cls as i64, est);
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn decision_boundary(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        if !self.is_fitted() {
            return Err(FitError::NotFitted);
        }

        let mut pred: Array2<f32> = Array2::zeros((X.nrows(), self.estimators.len()));

        for (cls, est) in &self.estimators {
            let z = est.decision_boundary(X)?;
            let col = *cls as usize;
            pred.slice_mut(s![.., col..col + 1]).assign(&z);
        }

        Ok(pred)
    }

    #[allow(dead_code)]
    fn loss(
        &self,
        logits: ArrayView2<f32>,
        y: &Array2<f32>,
        sample_weights: &Array2<f32>,
    ) -> Result<f32, FitError> {
        let mut total_loss = 0.0;

        for (cls, est) in &self.estimators {
            let y_masked = self.create_class_mask(y, &(*cls as f32));
            let col = *cls as usize;
            let cls_logits = logits.slice(s![.., col..col + 1]);
            total_loss += est.loss(cls_logits, &y_masked, sample_weights)?;
        }

        Ok(total_loss)
    }

    #[allow(dead_code)]
    fn predict(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        let proba = self.predict_proba(X)?;
        Ok(proba
            .map_axis(Axis(1), |a| {
                a.argmax().expect("Failed to find argmax") as f32
            })
            .insert_axis(Axis(1)))
    }

    #[allow(dead_code)]
    fn predict_proba(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        let mut proba = self.decision_boundary(X)?;

        for (cls, est) in &self.estimators {
            let col = *cls as usize;
            let mut col_slice = proba.slice_mut(s![.., col..col + 1]);
            let p = est.sigmoid(&col_slice.to_owned());
            col_slice.assign(&p.mapv(|x| x.clamp(EPS, 1.0)));
        }

        let row_sums = proba.sum_axis(Axis(1)).insert_axis(Axis(1));
        proba /= &row_sums;

        Ok(proba)
    }
}

#[derive(Clone)]
pub(crate) struct LogisticRegression {
    pub(crate) max_iter: usize,
    pub(crate) alpha: f32,
    pub(crate) learning_rate: f32,
    pub(crate) r_tol: f32,
    pub(crate) coeff: Option<Array1<f32>>,
    pub(crate) intercept: Option<f32>,
    pub(crate) losses: Vec<f32>,
}

impl LogisticRegression {
    pub(crate) fn new(max_iter: usize, alpha: f32, learning_rate: f32, r_tol: f32) -> Self {
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

    fn sigmoid(&self, z: &Array2<f32>) -> Array2<f32> {
        z.mapv(|z| {
            if z >= 0.0 {
                1.0 / (1.0 + (-z).exp())
            } else {
                let ez = z.exp();
                ez / (1.0 + ez)
            }
        })
    }

    fn update(
        &mut self,
        X: &Array2<f32>,
        logits: &Array2<f32>,
        y: &Array2<f32>,
        sample_weights: &Array2<f32>,
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
}

impl Estimator for LogisticRegression {
    fn is_fitted(&self) -> bool {
        self.coeff.is_some()
    }

    fn decision_boundary(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        let coeff = self.coeff.as_ref().ok_or(FitError::NotFitted)?;
        let mut z = X.dot(&coeff.view().insert_axis(Axis(1)));

        if let Some(intercept) = self.intercept {
            // we might not have intercept
            z += intercept;
        }

        Ok(z)
    }

    #[allow(dead_code)]
    fn predict(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        let proba = self.predict_proba(X)?;

        Ok(proba.mapv(|p| if p >= 0.5 { 1.0 } else { 0.0 }))
    }

    #[allow(dead_code)]
    fn predict_proba(&self, X: &Array2<f32>) -> Result<Array2<f32>, FitError> {
        let z = self.decision_boundary(X)?;

        let pred = self.sigmoid(&z);
        Ok(pred.mapv(|x| x.clamp(EPS, 1.0)))
    }

    fn loss(
        &self,
        logits: ArrayView2<f32>,
        y: &Array2<f32>,
        sample_weights: &Array2<f32>,
    ) -> Result<f32, FitError> {
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

        let base_loss = (sample_weights * (log_term - y * &logits)).sum() / weight_sum;

        let reg_loss = self.alpha / (2.0 * weight_sum) * c.pow2().sum();

        Ok(base_loss + reg_loss)
    }

    fn fit(
        &mut self,
        X: &Array2<f32>,
        y: &Array2<f32>,
        sample_weights: &Option<Array2<f32>>,
    ) -> Result<(), FitError> {
        if self.is_fitted() {
            return Err(FitError::AlreadyFitted);
        }

        let n = X.nrows();

        let sw = match sample_weights {
            Some(sw) => sw,
            None => &Array::ones((n, 1)),
        };

        self.coeff = Some(Array::zeros(X.ncols()));
        self.intercept = Some(0.0);

        let mut last_loss = f32::INFINITY;

        for _ in 0..self.max_iter {
            let logits_hat = self.decision_boundary(X)?;

            let loss = self.loss(logits_hat.view(), y, sw)?;
            self.losses.push(loss);

            if (last_loss - loss).abs() / last_loss.abs().max(1.0) <= self.r_tol {
                break;
            } else {
                self.update(X, &logits_hat, y, sw)?;
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

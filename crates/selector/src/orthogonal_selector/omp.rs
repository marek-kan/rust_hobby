use super::*;
use crate::orthogonal_selector::scores::*;

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

#[derive(Error, Debug)]
pub enum OrthogonalError {
    #[error("Fixed feature not orthogonal: {0} to the set: {1:?}")]
    FixedFeatureNotOrthogonal(usize, Vec<usize>),

    #[error[transparent]]
    QShapeError(#[from] ShapeError),
}

pub struct OrthogonalSelector {
    fixed_feature_indices: Vec<usize>,
    score_type: ScoreType,
    min_score: f64,
    center_featues: bool,
    thread_pool: ThreadPool,
    logistic_regression_params: LogisticRegressionParams,
}

impl OrthogonalSelector {
    pub fn new(
        fixed_feature_indices: Vec<usize>,
        score_type: ScoreType,
        min_score: f64,
        center_featues: bool,
        n_jobs: Option<usize>,
        logistic_regression_params: Option<LogisticRegressionParams>,
    ) -> Self {
        let log_reg_params = logistic_regression_params.unwrap_or_default();
        let n_threads = n_jobs.unwrap_or(1);

        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(n_threads)
            .build()
            .expect("Failed to create thread pool");

        OrthogonalSelector {
            fixed_feature_indices,
            score_type,
            min_score,
            center_featues,
            thread_pool,
            logistic_regression_params: log_reg_params,
        }
    }

    fn calculate_score(
        &self,
        x: &Array2<f64>,
        x_orth: &Array2<f64>,
        q: &Array2<f64>,
        y: &Array2<f64>,
        sw: &Array2<f64>,
    ) -> f64 {
        match self.score_type {
            ScoreType::ResidualVarianceRatio => residual_variance_ratio(x, x_orth, sw),
            ScoreType::SquaredPartialCorrelation => squared_partial_correlation(x_orth, y, q, sw),
            ScoreType::LogitGradient => {
                delta_log_loss(&self.logistic_regression_params, q, x_orth, y, sw)
                    .expect("Unexpected error during `delta_log_loss`")
            }
        }
    }

    fn weighted_center(&self, data: &Array2<f64>, sample_weights: &Array2<f64>) -> Array2<f64> {
        let weight_sum = sample_weights.sum();
        let mut means = data.t().dot(sample_weights) / weight_sum;
        means.reverse_axes();

        data - means
    }

    pub fn fit(
        &self,
        data: &mut Array2<f64>,
        y: &Array2<f64>,
        sample_weights: Option<Array2<f64>>,
    ) -> Result<(Vec<usize>, HashMap<usize, f64>), OrthogonalError> {
        let n = data.nrows();

        let sw = match sample_weights {
            Some(sw) => sw,
            None => Array::ones((n, 1)),
        };

        let mut q: Array2<f64> = Array2::zeros((n, 0));
        let mut selected: Vec<usize> = vec![];
        let mut scores: HashMap<usize, f64> = HashMap::new();
        let mut explore_feature_indices: Vec<usize> = (0..data.ncols()).collect();
        println!("{:?}", explore_feature_indices);

        if self.center_featues {
            *data = self.weighted_center(data, &sw)
        }

        for i in &self.fixed_feature_indices {
            let idx = i.to_owned();

            let pop_idx = explore_feature_indices
                .iter()
                .position(|f_idx| f_idx == i)
                .expect("couldn't find index of fixed feature");

            explore_feature_indices.remove(pop_idx); // remove fixed features from upcomming search

            let x = data.slice(s![.., idx..idx + 1]).to_owned();

            let x_orth = orthogonalize(&q, &x, &sw);
            let x_norm = weighted_norm(&x_orth, &sw);

            let score = self.calculate_score(&x, &x_orth, &q, y, &sw);
            println!("Idx: {} has score: {}", idx, score);

            q.push_column((x_orth / x_norm).column(0))?;
            selected.push(idx);
            scores.insert(idx, score);
        }

        println!("{:?}", explore_feature_indices);

        while !explore_feature_indices.is_empty() {
            let results: Vec<(usize, f64)> = self.thread_pool.install(|| {
                explore_feature_indices
                    .par_iter()
                    .map(|i| {
                        let idx = *i;
                        let x = data.slice(s![.., idx..idx + 1]).to_owned();
                        let x_orth = orthogonalize(&q, &x, &sw);

                        let score = self.calculate_score(&x, &x_orth, &q, y, &sw);
                        (idx, score)
                    })
                    .collect()
            });

            let best_feature = self.thread_pool.install(|| {
                results
                    .par_iter()
                    .max_by(|&&a, &&b| a.1.partial_cmp(&b.1).unwrap())
            });

            if let Some(&(best_feature_idx, best_feature_score)) = best_feature {
                scores.insert(best_feature_idx, best_feature_score);

                if best_feature_score >= self.min_score {
                    let x = data
                        .slice(s![.., best_feature_idx..best_feature_idx + 1])
                        .to_owned();
                    let x_orth = orthogonalize(&q, &x, &sw);
                    let x_norm = weighted_norm(&x_orth, &sw);

                    q.push_column((x_orth / x_norm).column(0))?;
                    selected.push(best_feature_idx);

                    explore_feature_indices.remove(
                        explore_feature_indices
                            .iter()
                            .position(|x| x == &best_feature_idx)
                            .expect(
                                "Failed to find `best_feature_index` in `explore_feature_indices`",
                            ),
                    );
                } else {
                    // Add scores from last iteration
                    for (k, v) in results {
                        scores.entry(k).or_insert(v);
                    }
                    break;
                }
            } else {
                break;
            };
        }

        Ok((selected, scores))
    }
}

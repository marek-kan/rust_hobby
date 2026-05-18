use std::sync::atomic::{AtomicUsize, Ordering};

use ndarray::Zip;

use crate::prelude::*;

pub struct DistanceCorrelation {
    thread_pool: ThreadPool,
    sample_size: Option<usize>,
}

impl Default for DistanceCorrelation {
    fn default() -> Self {
        DistanceCorrelation::new(1, None)
    }
}

impl DistanceCorrelation {
    pub fn new(n_jobs: usize, sample_size: Option<usize>) -> Self {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(n_jobs)
            .build()
            .expect("Failed to build thread pool!");

        DistanceCorrelation {
            thread_pool,
            sample_size,
        }
    }

    fn fit(&self, x: ArrayView2<f32>, y: ArrayView2<f32>) -> Vec<f32> {
        let b = double_center(
            &distance_matrix(y, Some(&self.thread_pool)),
            &self.thread_pool,
        );
        let counter = AtomicUsize::new(0);
        let total = x.ncols();

        self.thread_pool.install(|| {
            (0..x.ncols())
                .into_par_iter()
                .map(|j| {
                    let col = x.slice(s![.., j..j + 1]);
                    let a = double_center(
                        &distance_matrix(col, Some(&self.thread_pool)),
                        &self.thread_pool,
                    );

                    let dcov = parallel_product_mean(&a, &b);
                    let dvar_x = parallel_product_mean(&a, &a);
                    let dvar_y = parallel_product_mean(&b, &b);

                    let denom = (dvar_x * dvar_y).sqrt();

                    if denom < 1e-12 {
                        return 0.0;
                    }
                    let done = counter.fetch_add(1, Ordering::Relaxed) + 1;
                    println!("Progress: {done}/{total}");

                    (dcov / denom).sqrt().max(0.0)
                })
                .collect()
        })
    }

    pub fn fit_transform(&self, x: ArrayView2<f32>, y: ArrayView2<f32>) -> Vec<f32> {
        if let Some(sample_size) = self.sample_size {
            let mut rnd_seed = rand::rng();

            let all_indices = arr1(&Vec::from_iter(0..x.nrows()));

            let sampled_indices = all_indices
                .sample_axis_using(
                    Axis(0),
                    sample_size,
                    SamplingStrategy::WithoutReplacement,
                    &mut rnd_seed,
                )
                .to_vec();

            self.fit(
                x.select(Axis(0), &sampled_indices).view(),
                y.select(Axis(0), &sampled_indices).view(),
            )
        } else {
            self.fit(x, y)
        }
    }
}

#[allow(dead_code)]
fn parallel_mean(a: ArrayView2<f32>) -> f32 {
    let n = (a.nrows() * a.ncols()) as f32;
    let sum: f32 = a.into_par_iter().sum();
    sum / n
}

/// Computes the mean of the element-wise product of two arrays in parallel
fn parallel_product_mean(a: &Array2<f32>, b: &Array2<f32>) -> f32 {
    let n = (a.nrows() * a.ncols()) as f32;
    let sum: f32 = Zip::from(a)
        .and(b)
        .into_par_iter()
        .map(|(&x, &y)| x * y)
        .sum();
    sum / n
}

/// Compute the pairwise absolute distance matrix for a column vector `x`.
///
/// Each element `(i, j)` is `|x[i] - x[j]|`.
/// Row iteration is parallelized via rayon.
fn distance_matrix(x: ArrayView2<f32>, thread_pool: Option<&ThreadPool>) -> Array2<f32> {
    let n = x.nrows();
    let x_flat: Array1<f32> = x.column(0).to_owned();
    let mut dist = Array2::<f32>::zeros((n, n));
    let mut_rows = dist.axis_iter_mut(Axis(0));

    let compute_row = |(i, mut row): (usize, ArrayViewMut1<f32>)| {
        let xi = x_flat[i];
        for (j, val) in row.iter_mut().enumerate() {
            *val = (xi - x_flat[j]).abs();
        }
    };

    match thread_pool {
        Some(pool) => {
            pool.install(|| {
                mut_rows.into_par_iter().enumerate().for_each(compute_row);
            });
        }
        None => {
            mut_rows.enumerate().for_each(compute_row);
        }
    }

    dist
}

/// `A[i,j] = D[i,j] - row_mean[i] - col_mean[j] + grand_mean`
fn double_center_simple(d: &Array2<f32>) -> Array2<f32> {
    let row_mean = d.mean_axis(Axis(1)).expect("empty array in double_center");
    let col_mean = d.mean_axis(Axis(0)).expect("empty array in double_center");
    let grand_mean = d.mean().expect("empty array in double_center");

    let n = d.nrows();
    // broadcast: subtract row_mean (column vector) and col_mean (row vector), add grand_mean
    let row_mean_2d = row_mean.into_shape_with_order((n, 1)).unwrap();
    let col_mean_2d = col_mean.into_shape_with_order((1, n)).unwrap();

    d - &row_mean_2d - &col_mean_2d + grand_mean
}

/// Using paralel implementation calculate:
/// `A[i,j] = D[i,j] - row_mean[i] - col_mean[j] + grand_mean`
fn double_center(d: &Array2<f32>, pool: &ThreadPool) -> Array2<f32> {
    let n = d.nrows();

    pool.install(|| {
        // Parallel row means
        let row_means = d
            .axis_iter(Axis(0))
            .into_par_iter()
            .map(|row| row.mean().unwrap())
            .collect::<Vec<_>>();

        // Parallel col means
        let col_means = d
            .axis_iter(Axis(1))
            .into_par_iter()
            .map(|col| col.mean().unwrap())
            .collect::<Vec<_>>();

        // Note: grand_mean is simply the mean of row_means
        let grand_mean: f32 = row_means.par_iter().sum::<f32>() / n as f32;

        let mut out = Array2::<f32>::zeros((n, d.ncols()));

        // Perform the broadcast math in parallel without intermediate allocations
        Zip::from(out.rows_mut())
            .and(d.rows())
            .and(&row_means)
            .par_for_each(|mut out_row, d_row, &r_mean| {
                for j in 0..d_row.len() {
                    let c_mean = col_means[j];
                    out_row[j] = d_row[j] - r_mean - c_mean + grand_mean;
                }
            });

        out
    })
}

/// Compute distance correlation between two column vectors `x` and `y` for testing purpose.
///
/// Both `x` and `y` must be 2-D arrays of shape `(n, 1)`.
///
/// Returns a value in `[0, 1]` where `0` indicates no relationship and `1`
/// indicates perfect dependence.
fn distance_correlation(x: ArrayView2<f32>, y: ArrayView2<f32>) -> f32 {
    let a = double_center_simple(&distance_matrix(x, None));
    let b = double_center_simple(&distance_matrix(y, None));

    let dcov = (&a * &b)
        .mean()
        .expect("empty array in distance_correlation");
    let dvar_x = (&a * &a)
        .mean()
        .expect("empty array in distance_correlation");
    let dvar_y = (&b * &b)
        .mean()
        .expect("empty array in distance_correlation");

    let denom = (dvar_x * dvar_y).sqrt();
    if denom < 1e-12 {
        return 0.0;
    }

    (dcov / denom).sqrt().max(0.0)
}

#[cfg(test)]
mod test {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_perfect_correlation() {
        let x = array![[1.0], [2.0], [3.0], [4.0], [5.0]];
        let y = array![[2.0], [4.0], [6.0], [8.0], [10.0]];
        let dcor = DistanceCorrelation::default().fit_transform(x.view(), y.view())[0];
        assert!((dcor - 1.0).abs() < 1e-9, "expected 1.0, got {dcor}");
    }

    #[test]
    fn test_self_correlation() {
        let x = array![[1.0], [3.0], [2.0], [5.0], [4.0]];
        let dcor = DistanceCorrelation::default().fit_transform(x.view(), x.view())[0];
        assert!((dcor - 1.0).abs() < 1e-9, "expected 1.0, got {dcor}");
    }

    #[test]
    fn test_constant_x() {
        let x = array![[3.0], [3.0], [3.0], [3.0]];
        let y = array![[1.0], [2.0], [3.0], [4.0]];
        let dcor = DistanceCorrelation::default().fit_transform(x.view(), y.view())[0];
        assert_eq!(dcor, 0.0);
    }

    #[test]
    fn test_batch_matches_single() {
        let x = array![[1.0, 5.0], [2.0, 3.0], [3.0, 1.0], [4.0, 4.0], [5.0, 2.0]];
        let y = array![[1.0], [2.0], [3.0], [4.0], [5.0]];

        let batch = DistanceCorrelation::default().fit_transform(x.view(), y.view());
        for j in 0..x.ncols() {
            let col = x.slice(s![.., j..j + 1]);
            let single = distance_correlation(col, y.view());
            assert!(
                (batch[j] - single).abs() < 1e-12,
                "col {j}: batch={} single={}",
                batch[j],
                single
            );
        }
    }

    #[test]
    fn test_square_x() {
        let x = Array::random((1000, 1), Normal::new(0., 1.).unwrap());
        let y = x.pow2();

        let score = DistanceCorrelation::new(2, Some(200)).fit_transform(x.view(), y.view())[0];
        println!("Dist. corr score: {:?}", score);
        assert!(score > 0.5)
    }

    #[test]
    fn test_double_center() {
        let x = Array::random((1000, 1), Normal::new(0., 1.).unwrap());

        let distance = distance_matrix(x.view(), None);

        let res_simple = double_center_simple(&distance);

        let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
        let res_multip = double_center(&distance, &pool);

        let diff = (&res_simple - &res_multip).abs();
        let mu = diff.mean().unwrap();

        println!("Mean abs diff: {:?}", mu);

        assert!(mu < 1e-3)
    }
}

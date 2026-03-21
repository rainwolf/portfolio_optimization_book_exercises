use argmin::core::CostFunction;
use argmin::core::Error;
use argmin::core::Executor;
use argmin::core::observers::ObserverMode;
use argmin::solver::neldermead::NelderMead;
use argmin_observer_slog::SlogLogger;
use faer::prelude::Solve;
use faer::{Col, Mat, Side, linalg::solvers::Llt};
use statrs::function::gamma::ln_gamma;
use std::f64::consts::PI;

// Multivariate Student's t log-likelihood
struct MultivariateTLLF {
    data: Mat<f64>,
    dim: usize,
    jitter: f64, // Regularization strength
}

impl MultivariateTLLF {
    fn new(data: Vec<Vec<f64>>, jitter: f64) -> Self {
        let n = data.len();
        let d = data[0].len();

        let mut matrix = Mat::zeros(n, d);
        for (i, row) in data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                matrix[(i, j)] = val;
            }
        }

        Self {
            data: matrix,
            dim: d,
            jitter,
        }
    }

    // Apply jitter to ensure positive definiteness
    fn apply_jitter(&self, sigma: &Mat<f64>) -> Mat<f64> {
        let mut sigma_jittered = sigma.clone();
        for i in 0..self.dim {
            sigma_jittered[(i, i)] += self.jitter;
        }
        sigma_jittered
    }

    // Log PDF of multivariate t-distribution for one observation
    fn ln_pdf_single(&self, x: &Col<f64>, mu: &Col<f64>, sigma: &Mat<f64>, nu: f64) -> Option<f64> {
        let d = self.dim as f64;

        if nu <= 0.0 {
            return None;
        }

        // Apply jitter and compute Cholesky
        let sigma_jittered = self.apply_jitter(sigma);
        let llt = Llt::new(sigma_jittered.as_ref(), Side::Lower).ok()?;
        let log_det_sigma = 2.0 * (0..self.dim).map(|i| llt.L()[(i, i)].ln()).sum::<f64>();

        // Mahalanobis distance
        let diff = x - mu;
        let y = llt.solve(diff.as_ref()); // y = Σ⁻¹·diff
        let mahalanobis: f64 = (0..self.dim).map(|i| diff[i] * y[i]).sum();

        // Log PDF formula
        let log_gamma_num = ln_gamma((d + nu) / 2.0);
        let log_gamma_den = ln_gamma(nu / 2.0);
        let log_gamma_d2 = ln_gamma(d / 2.0);

        let log_const = log_gamma_num
            - log_gamma_den
            - log_gamma_d2
            - (d / 2.0) * nu.ln()
            - (d / 2.0) * PI.ln()
            - 0.5 * log_det_sigma;

        let log_kernel = -((nu + d) / 2.0) * (1.0 + mahalanobis / nu).ln();

        Some(log_const + log_kernel)
    }
}

impl CostFunction for MultivariateTLLF {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, params: &Self::Param) -> Result<Self::Output, Error> {
        let d = self.dim;

        if params.len() != d + d * d + 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Parameter vector has incorrect length",
            )
            .into());
        }

        let mu = Col::from_fn(d, |i| params[i]);

        // Reconstruct Sigma (symmetrize)
        let mut sigma = Mat::zeros(d, d);
        let mut idx = d;
        for i in 0..d {
            for j in 0..d {
                sigma[(i, j)] = params[idx];
                idx += 1;
            }
        }
        sigma = (&sigma + sigma.transpose()) / 2.0;

        let nu = params[params.len() - 1];

        // Only nu needs hard constraints now (jitter handles Σ)
        if nu <= 0.0 {
            return Ok(f64::INFINITY);
        }

        // Calculate negative log-likelihood
        let mut nll = 0.0;
        for i in 0..self.data.nrows() {
            let x = Col::from_fn(self.dim, |j| self.data[(i, j)]);
            match self.ln_pdf_single(&x, &mu, &sigma, nu) {
                Some(ln_pdf) => nll -= ln_pdf,
                None => return Ok(f64::INFINITY),
            }
        }

        Ok(nll)
    }
}

pub fn estimate_covariance_matrix_student_t_ml_estimator(data: &[Vec<f64>]) -> Mat<f64> {
    let dim = data[0].len();

    // Jitter: 1e-6 to 1e-4 is typical
    // Larger values = more stable but more biased
    let jitter = 1e-6;

    let problem = MultivariateTLLF::new(data.into(), jitter);

    // flat parameterization: [mu..., sigma_flat..., nu]
    let mut initial_params: Vec<f64> = vec![0.0; dim + dim * dim + 1];
    initial_params[0..dim].copy_from_slice(&vec![0.0; dim]);

    // Sigma = identity
    for i in 0..dim {
        initial_params[dim + i * dim + i] = 1.0;
    }
    let last = initial_params.len() - 1;

    initial_params[last] = 5.0; // default initial guess for degrees of freedom (heavy-tailed)

    // Build initial simplex: initial_params + one vertex per dimension (perturbed)
    let n = initial_params.len();
    let mut simplex = vec![initial_params.clone()];
    for i in 0..n {
        let mut vertex = initial_params.clone();
        vertex[i] += if vertex[i].abs() < 1e-10 {
            0.05
        } else {
            vertex[i] * 0.05
        };
        simplex.push(vertex);
    }
    let solver = NelderMead::new(simplex);

    let res = Executor::new(problem, solver)
        .configure(|state| state.param(initial_params).max_iters(50_000))
        .add_observer(SlogLogger::term(), ObserverMode::Every(100))
        .run()
        .unwrap();

    let params = res.state.best_param.unwrap();

    let mu = Col::from_fn(dim, |i| params[i]);

    let mut sigma = Mat::zeros(dim, dim);
    let mut idx = dim;
    for i in 0..dim {
        for j in 0..dim {
            sigma[(i, j)] = params[idx];
            idx += 1;
        }
    }
    sigma = (&sigma + sigma.transpose()) / 2.0;

    let nu = params[params.len() - 1];

    println!("\n=== MLE Results ===");
    println!("Location (μ):\n{:?}", mu);
    println!("\nScale matrix (Σ):\n{:?}", sigma);
    println!("\nDegrees of freedom (ν): {:.3}", nu);
    println!("Jitter used: {:.2e}", jitter);

    if nu < 30.0 {
        println!("\n✓ Heavy tails detected! (ν < 30)");
    }
    sigma
}

use argmin::core::CostFunction;
use argmin::core::Error;
use argmin::core::Executor;
use argmin::core::observers::ObserverMode;
use argmin::solver::neldermead::NelderMead;
use argmin_observer_slog::SlogLogger;
use faer::prelude::Solve;
use faer::{Col, Mat, Side, linalg::solvers::Llt};
use statrs::function::gamma::{digamma, ln_gamma};
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

/// Estimate covariance matrix via EM algorithm for multivariate Student-t MLE.
///
/// EM algorithm (Liu & Rubin 1995):
///   E-step: w_i = (ν + d) / (ν + δ_i)  where δ_i = (x_i - μ)ᵀ Σ⁻¹ (x_i - μ)
///   M-step: closed-form for μ and Σ; 1D bisection for ν
///
/// Much more efficient than Nelder-Mead: scales O(n·d²) per iteration,
/// no need for a (d²+d+1)-dimensional simplex.
pub fn estimate_covariance_matrix_student_t_em(data: &[Vec<f64>]) -> Mat<f64> {
    let n = data.len();
    let d = data[0].len();
    let jitter = 1e-6;

    let mut x = Mat::zeros(n, d);
    for (i, row) in data.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            x[(i, j)] = val;
        }
    }

    // Initialize μ = sample mean
    let mut mu = vec![0.0_f64; d];
    for i in 0..n {
        for j in 0..d {
            mu[j] += x[(i, j)];
        }
    }
    for v in mu.iter_mut() {
        *v /= n as f64;
    }

    // Initialize Σ = sample covariance + jitter
    let mut sigma = Mat::zeros(d, d);
    for i in 0..n {
        for r in 0..d {
            for c in 0..d {
                sigma[(r, c)] += (x[(i, r)] - mu[r]) * (x[(i, c)] - mu[c]);
            }
        }
    }
    for r in 0..d {
        for c in 0..d {
            sigma[(r, c)] /= n as f64;
        }
    }
    for i in 0..d {
        sigma[(i, i)] += jitter;
    }

    let mut nu = 5.0_f64;

    for iter in 0..1000 {
        // E-step: w_i = (ν + d) / (ν + δ_i)
        let llt = Llt::new(sigma.as_ref(), Side::Lower)
            .expect("Sigma not positive definite — increase jitter");
        let mut weights = vec![0.0_f64; n];
        for i in 0..n {
            let diff = Col::from_fn(d, |j| x[(i, j)] - mu[j]);
            let y = llt.solve(diff.as_ref());
            let delta: f64 = (0..d).map(|j| diff[j] * y[j]).sum();
            weights[i] = (nu + d as f64) / (nu + delta);
        }
        let w_sum: f64 = weights.iter().sum();

        // M-step: μ
        let mu_old = mu.clone();
        mu = vec![0.0; d];
        for i in 0..n {
            for j in 0..d {
                mu[j] += weights[i] * x[(i, j)];
            }
        }
        for v in mu.iter_mut() {
            *v /= w_sum;
        }

        // M-step: Σ
        let sigma_old = sigma.clone();
        sigma = Mat::zeros(d, d);
        for i in 0..n {
            for r in 0..d {
                for c in 0..d {
                    sigma[(r, c)] += weights[i] * (x[(i, r)] - mu[r]) * (x[(i, c)] - mu[c]);
                }
            }
        }
        for r in 0..d {
            for c in 0..d {
                sigma[(r, c)] /= n as f64;
            }
        }
        for i in 0..d {
            sigma[(i, i)] += jitter;
        }

        // M-step: ν via bisection on
        //   ψ((ν+d)/2) - ln((ν+d)/2) - ψ(ν/2) + ln(ν/2) + c = 0
        //   where c = (1/n) Σ [ln(w_i) - w_i] + 1
        let c = weights.iter().map(|&w| w.ln() - w).sum::<f64>() / n as f64 + 1.0;
        let f_nu = |v: f64| {
            digamma((v + d as f64) / 2.0) - ((v + d as f64) / 2.0).ln()
                - digamma(v / 2.0) + (v / 2.0).ln()
                + c
        };
        let (mut lo, mut hi) = (1e-4_f64, 1000.0_f64);
        if f_nu(lo) * f_nu(hi) < 0.0 {
            for _ in 0..100 {
                let mid = (lo + hi) / 2.0;
                if f_nu(lo) * f_nu(mid) <= 0.0 { hi = mid; } else { lo = mid; }
                if hi - lo < 1e-9 { break; }
            }
            nu = (lo + hi) / 2.0;
        }

        // Convergence check
        let mu_change: f64 = mu.iter().zip(&mu_old).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt();
        let mut sigma_sq_diff = 0.0_f64;
        for r in 0..d {
            for c in 0..d {
                let diff = sigma[(r, c)] - sigma_old[(r, c)];
                sigma_sq_diff += diff * diff;
            }
        }
        let sigma_change = sigma_sq_diff.sqrt();
        if mu_change < 1e-8 && sigma_change < 1e-8 {
            println!("EM converged at iteration {}", iter + 1);
            break;
        }
    }

    println!("\n=== Student-t MLE Results (EM) ===");
    println!("Degrees of freedom (ν): {:.3}", nu);
    if nu < 30.0 {
        println!("Heavy tails detected (ν < 30)");
    }

    sigma
}

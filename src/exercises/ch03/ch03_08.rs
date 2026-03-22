pub fn exercise03_08() {
    println!("\x1B[2J"); // clear the terminal
    println!();
    println!(
        "8)  (Derivation of Gaussian ML estimators) Given T N-dimensional observations, the Gaussian ML estimation for μ and Σ is formulated as:"
    );
    println!("      min_μ,Σ 1/T ∑_{{t=1}}^T (x_t - μ)^T Σ^{{-1}} (x_t - μ) + log|Σ|");
    println!(
        "   Derive the estimators by setting the gradient of the objective function with respect to μ and Σ^{{−1}} to zero"
    );
    println!();
    println!("   Solution:");
    println!("   Setting the gradient with respect to μ to zero gives:");
    println!("      μ = (1/T) ∑_{{t=1}}^T x_t");
    println!("   Setting the gradient with respect to Σ^{{−1}} to zero gives:");
    println!("      Σ = (1/T) ∑_{{t=1}}^T (x_t - μ)(x_t - μ)^T");
}

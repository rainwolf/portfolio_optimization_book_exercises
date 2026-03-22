pub fn exercise03_09() {
    println!("\x1B[2J"); // clear the terminal
    println!();
    println!(
        "9)  (Derivation of heavy-tailed ML estimators) Given T N-dimensional observations, the heavy-tailed ML estimation (under the t distribution with degrees of freedom ν) for μ and Σ is formulated as:"
    );
    println!(
        "      min_μ,Σ  log|Σ| + (ν+N)/T ∑_{{t=1}}^T log(1 + (1/ν) (x_t - μ)^T Σ^{{-1}} (x_t - μ))"
    );
    println!(
        "   Derive the estimators by setting the gradient of the objective function with respect to μ and Σ^{{−1}} to zero"
    );
    println!();
    println!("   Solution:");
    println!("   Setting the gradient with respect to μ to zero gives:");
    println!(
        "      ∑_{{t=1}}^T [(ν+N) / (ν + (x_t - μ)^T Σ^{{-1}} (x_t - μ))] Σ^{{-1}} (x_t - μ) = 0"
    );
    println!("   Setting the gradient with respect to Σ^{{-1}} to zero gives:");
    println!(
        "      Σ = (1/T) ∑_{{t=1}}^T [(ν+N) / (ν + (x_t - μ)^T Σ^{{-1}} (x_t - μ))] (x_t - μ)(x_t - μ)^T"
    );
    todo!("maybe come back to this later.")
}
